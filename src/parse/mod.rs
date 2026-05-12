mod cursor;

use crate::{
    cfg::{BasicBlock, Cfg, Label, Terminator},
    playfield::Playfield,
    state::{Direction, Mode, State},
};

use self::cursor::Cursor;

/// Parses a [`Cfg`] from a [`Playfield`].
pub fn parse_playfield(playfield: &Playfield) -> Cfg {
    parse_playfield_at(playfield, State::default())
}

/// A parsed [`Terminator`].
enum Item {
    /// A [`Terminator`].
    Terminator(Terminator),
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
        Mode::String => todo!("parsing string mode"),
    };

    #[expect(
        clippy::infallible_destructuring_match,
        reason = "more item variants will be added later"
    )]
    let terminator = match item {
        Item::Terminator(terminator) => terminator,
    };

    BasicBlock { terminator }
}

/// Parses an [`Item`] from a [`Cursor`] in command mode.
fn parse_command(cursor: Cursor<'_>) -> Item {
    match cursor.value().to_char_lossy() {
        '>' => cursor.go(Direction::Right).into(),
        '<' => cursor.go(Direction::Left).into(),
        '^' => cursor.go(Direction::Up).into(),
        'v' => cursor.go(Direction::Down).into(),
        '_' => branch_item(cursor, Direction::Left, Direction::Right),
        '|' => branch_item(cursor, Direction::Up, Direction::Down),
        '#' => cursor.step().step().into(),
        '@' => Terminator::Halt.into(),
        _ => cursor.step().into(),
    }
}

/// Returns a new branch [`Item`] from a [`Cursor`] and branch [`Direction`]s.
fn branch_item(cursor: Cursor<'_>, then_direction: Direction, else_direction: Direction) -> Item {
    let then_label = cursor.go(then_direction).into();
    let else_label = cursor.go(else_direction).into();
    Terminator::Branch(then_label, else_label).into()
}
