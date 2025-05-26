mod cursor;

use std::collections::{BTreeMap, BTreeSet};

use cursor::Cursor;

use crate::{
    ast::{BinOp, Expr},
    common::Playfield,
    ir::{
        Block, Exit, Instruction, Label, Program, State,
        state::{Direction, Mode},
    },
};

/// Parses a program from a playfield.
pub fn parse_program(playfield: &Playfield) -> Program {
    let mut program = Program {
        blocks: BTreeMap::new(),
    };

    let main_state = State::default();
    program.blocks.insert(
        Label::Main,
        Exit::Jump(Label::State(main_state.clone())).into_block(),
    );

    let mut unexplored_states = BTreeSet::new();
    unexplored_states.insert(main_state);

    while let Some(state) = unexplored_states.pop_first() {
        let label = Label::State(state.clone());
        if program.blocks.contains_key(&label) {
            continue;
        }

        let cursor = Cursor::new(playfield, state);
        let block = parse_block(cursor);

        for state in block.exit.to_states() {
            unexplored_states.insert(state);
        }

        program.blocks.insert(label, block);
    }

    program
}

/// Parses a block from a cursor.
fn parse_block(cursor: Cursor) -> Block {
    let value = cursor.value();
    match (cursor.mode(), value.into_printable_ascii_char_lossy()) {
        (Mode::Command, '0') => Instruction::Push(Expr::Literal(0.into())).into_block(cursor),
        (Mode::Command, '1') => Instruction::Push(Expr::Literal(1.into())).into_block(cursor),
        (Mode::Command, '2') => Instruction::Push(Expr::Literal(2.into())).into_block(cursor),
        (Mode::Command, '3') => Instruction::Push(Expr::Literal(3.into())).into_block(cursor),
        (Mode::Command, '4') => Instruction::Push(Expr::Literal(4.into())).into_block(cursor),
        (Mode::Command, '5') => Instruction::Push(Expr::Literal(5.into())).into_block(cursor),
        (Mode::Command, '6') => Instruction::Push(Expr::Literal(6.into())).into_block(cursor),
        (Mode::Command, '7') => Instruction::Push(Expr::Literal(7.into())).into_block(cursor),
        (Mode::Command, '8') => Instruction::Push(Expr::Literal(8.into())).into_block(cursor),
        (Mode::Command, '9') => Instruction::Push(Expr::Literal(9.into())).into_block(cursor),
        (Mode::Command, '+') => Instruction::Binary(BinOp::Add).into_block(cursor),
        (Mode::Command, '-') => Instruction::Binary(BinOp::Subtract).into_block(cursor),
        (Mode::Command, '*') => Instruction::Binary(BinOp::Multiply).into_block(cursor),
        (Mode::Command, '/') => Instruction::Binary(BinOp::Divide).into_block(cursor),
        (Mode::Command, '%') => Instruction::Binary(BinOp::Modulo).into_block(cursor),
        (Mode::Command, '`') => Instruction::Binary(BinOp::Greater).into_block(cursor),
        (Mode::Command, '>') => cursor.go(Direction::Right).into(),
        (Mode::Command, '<') => cursor.go(Direction::Left).into(),
        (Mode::Command, '^') => cursor.go(Direction::Up).into(),
        (Mode::Command, 'v') => cursor.go(Direction::Down).into(),
        (Mode::Command, '?') => random(cursor),
        (Mode::Command, '_') => branch(cursor, Direction::Left, Direction::Right),
        (Mode::Command, '|') => branch(cursor, Direction::Up, Direction::Down),
        (Mode::Command | Mode::String, '"') => cursor.toggle_mode().step().into(),
        (Mode::Command, '#') => cursor.step().step().into(),
        (Mode::Command, '@') => Exit::End.into_block(),
        (Mode::Command, _) => cursor.step().into(),
        (Mode::String, _) => Instruction::Push(Expr::Literal(value)).into_block(cursor),
    }
}

/// Creates a random block from a cursor.
fn random(cursor: Cursor) -> Block {
    let right_label = cursor.clone().go(Direction::Right).into();
    let down_label = cursor.clone().go(Direction::Down).into();
    let left_label = cursor.clone().go(Direction::Left).into();
    let up_label = cursor.go(Direction::Up).into();
    Exit::Random(right_label, down_label, left_label, up_label).into_block()
}

/// Creates a branch block from a cursor and directions.
fn branch(cursor: Cursor, then_direction: Direction, else_direction: Direction) -> Block {
    let then_label = cursor.clone().go(then_direction).into();
    let else_label = cursor.go(else_direction).into();
    Exit::Branch(then_label, else_label).into_block()
}

impl Instruction {
    /// Converts the instruction to a block with a cursor.
    fn into_block(self, cursor: Cursor) -> Block {
        let instructions = vec![self];
        let exit = cursor.step().into();
        Block { instructions, exit }
    }
}

impl Exit {
    /// Converts the exit to a block.
    fn into_block(self) -> Block {
        Block {
            instructions: Vec::new(),
            exit: self,
        }
    }

    /// Converts the exit to a vector of states.
    fn to_states(&self) -> Vec<State> {
        match self {
            Self::Jump(l) => l.to_state().into_iter().collect(),
            Self::Random(r, d, l, u) => r
                .to_state()
                .into_iter()
                .chain(d.to_state())
                .chain(l.to_state())
                .chain(u.to_state())
                .collect(),
            Self::Branch(t, e) => t.to_state().into_iter().chain(e.to_state()).collect(),
            Self::End => Vec::new(),
        }
    }
}

impl Label {
    /// Converts the label to an optional state.
    fn to_state(&self) -> Option<State> {
        match self {
            Self::Main => None,
            Self::State(s) => Some(s.clone()),
        }
    }
}
