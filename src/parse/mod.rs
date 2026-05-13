mod cursor;

use crate::{
    cfg::{BasicBlock, Cfg, Expr, Instruction, Label, Terminator, UnOp},
    playfield::Playfield,
    state::{Direction, Mode, State},
    value::Value,
};

use self::cursor::Cursor;

/// Parses a [`Cfg`] from a [`Playfield`].
pub fn parse_playfield(playfield: &Playfield) -> Cfg {
    parse_playfield_at(playfield, State::default())
}

/// A parsed [`Instruction`] or [`Terminator`].
enum Item {
    /// An [`Instruction`].
    Instruction(Instruction),

    /// A [`Terminator`].
    Terminator(Terminator),
}

impl From<Instruction> for Item {
    fn from(value: Instruction) -> Self {
        Self::Instruction(value)
    }
}

impl From<Terminator> for Item {
    fn from(value: Terminator) -> Self {
        Self::Terminator(value)
    }
}

/// Parses a [`Cfg`] from a [`Playfield`] at a main [`State`].
fn parse_playfield_at(playfield: &Playfield, main_state: State) -> Cfg {
    let mut cfg = Cfg::new();
    cfg.insert_basic_block(
        Label::Main,
        BasicBlock {
            instructions: Vec::new(),
            terminator: Terminator::Jump(Label::State(main_state)),
        },
    );

    let mut pending_states = vec![main_state];

    while let Some(state) = pending_states.pop() {
        let label = Label::State(state);

        if cfg.contains_label(label) {
            continue;
        }

        let basic_block = parse_basic_block(playfield, state);

        for pending_label in basic_block.terminator.labels() {
            let Label::State(pending_state) = pending_label else {
                unreachable!("all pending labels should point to states");
            };

            pending_states.push(pending_state);
        }

        cfg.insert_basic_block(label, basic_block);
    }

    cfg
}

/// Parses a [`BasicBlock`] from a [`Playfield`] and a [`State`].
fn parse_basic_block(playfield: &Playfield, state: State) -> BasicBlock {
    let cursor = Cursor::new(playfield, state);

    let item = match state.mode {
        Mode::Command => parse_command(cursor),
        Mode::String => parse_string(cursor),
    };

    let (instructions, terminator) = match item {
        Item::Instruction(instruction) => (vec![instruction], cursor.step().into()),
        Item::Terminator(terminator) => (Vec::new(), terminator),
    };

    BasicBlock {
        instructions,
        terminator,
    }
}

/// Parses an [`Item`] from a [`Cursor`] in command mode.
fn parse_command(cursor: Cursor<'_>) -> Item {
    match cursor.value().to_char_lossy() {
        digit @ '0'..='9' => {
            let value = (u32::from(digit) - u32::from('0')).into();
            Instruction::Push(Expr::Const(Value(value))).into()
        }
        '!' => Instruction::Unary(UnOp::Not).into(),
        '>' => cursor.go(Direction::Right).into(),
        '<' => cursor.go(Direction::Left).into(),
        '^' => cursor.go(Direction::Up).into(),
        'v' => cursor.go(Direction::Down).into(),
        '?' => {
            let right_label = cursor.go(Direction::Right).into();
            let down_label = cursor.go(Direction::Down).into();
            let left_label = cursor.go(Direction::Left).into();
            let up_label = cursor.go(Direction::Up).into();
            Terminator::Random(right_label, down_label, left_label, up_label).into()
        }
        '_' => branch_item(cursor, Direction::Left, Direction::Right),
        '|' => branch_item(cursor, Direction::Up, Direction::Down),
        '"' => cursor.step_with_mode(Mode::String).into(),
        ':' => Instruction::Duplicate.into(),
        '\\' => Instruction::Swap.into(),
        '$' => Instruction::Pop.into(),
        '#' => cursor.step().step().into(),
        'p' => Terminator::Put(cursor.step().into()).into(),
        '&' => Instruction::Push(Expr::InputInt).into(),
        '~' => Instruction::Push(Expr::InputChar).into(),
        '@' => Terminator::Halt.into(),
        _ => cursor.step().into(),
    }
}

/// Parses an [`Item`] from a [`Cursor`] in string mode.
fn parse_string(cursor: Cursor<'_>) -> Item {
    let value = cursor.value();

    if value == '"'.into() {
        cursor.step_with_mode(Mode::Command).into()
    } else {
        Instruction::Push(Expr::Const(value)).into()
    }
}

/// Returns a new branch [`Item`] from a [`Cursor`] and branch [`Direction`]s.
fn branch_item(cursor: Cursor<'_>, then_direction: Direction, else_direction: Direction) -> Item {
    let then_label = cursor.go(then_direction).into();
    let else_label = cursor.go(else_direction).into();
    Terminator::Branch(then_label, else_label).into()
}
