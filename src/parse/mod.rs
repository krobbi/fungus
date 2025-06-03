mod cursor;
mod flow_graph;

use std::collections::{BTreeMap, BTreeSet};

use cursor::Cursor;
use flow_graph::FlowGraph;

use crate::{
    common::Playfield,
    ir::{
        Block, Exit, Instruction, Label, Program, State,
        ops::{BinOp, DivOp, UnOp},
        state::{Direction, Mode},
    },
};

/// Parses a program from a playfield.
pub fn parse_program(playfield: &Playfield) -> Program {
    parse_program_state(playfield, State::default())
}

/// Parses a program from a playfield and a main state.
pub fn parse_program_state(playfield: &Playfield, main_state: State) -> Program {
    let mut program = Program {
        blocks: BTreeMap::new(),
    };
    program.blocks.insert(
        Label::Main,
        Exit::Jump(Label::State(main_state.clone())).into_block(),
    );

    let mut flow_graph = FlowGraph::new(main_state.position());
    let mut unexplored_states = BTreeSet::new();
    unexplored_states.insert(main_state);

    while let Some(state) = unexplored_states.pop_first() {
        let label = Label::State(state.clone());
        if program.blocks.contains_key(&label) {
            continue;
        }

        let position = state.position();
        let cursor = Cursor::new(playfield, state);
        let block = parse_block(cursor);

        for unexplored_state in block.exit.states() {
            flow_graph.insert_connection(position, unexplored_state.position());
            unexplored_states.insert(unexplored_state.clone());
        }

        program.blocks.insert(label, block);
    }

    flow_graph.dump();
    program
}

/// Parses a block from a cursor.
fn parse_block(cursor: Cursor) -> Block {
    let value = cursor.value();
    match (cursor.mode(), value.into_char_lossy()) {
        (Mode::Command, '0') => push(0, cursor),
        (Mode::Command, '1') => push(1, cursor),
        (Mode::Command, '2') => push(2, cursor),
        (Mode::Command, '3') => push(3, cursor),
        (Mode::Command, '4') => push(4, cursor),
        (Mode::Command, '5') => push(5, cursor),
        (Mode::Command, '6') => push(6, cursor),
        (Mode::Command, '7') => push(7, cursor),
        (Mode::Command, '8') => push(8, cursor),
        (Mode::Command, '9') => push(9, cursor),
        (Mode::Command, '+') => binary(BinOp::Add, cursor),
        (Mode::Command, '-') => binary(BinOp::Subtract, cursor),
        (Mode::Command, '*') => binary(BinOp::Multiply, cursor),
        (Mode::Command, '/') => divide(DivOp::Quotient, cursor),
        (Mode::Command, '%') => divide(DivOp::Remainder, cursor),
        (Mode::Command, '!') => unary(UnOp::Not, cursor),
        (Mode::Command, '`') => binary(BinOp::Greater, cursor),
        (Mode::Command, '>') => cursor.go(Direction::Right).into(),
        (Mode::Command, '<') => cursor.go(Direction::Left).into(),
        (Mode::Command, '^') => cursor.go(Direction::Up).into(),
        (Mode::Command, 'v') => cursor.go(Direction::Down).into(),
        (Mode::Command, '?') => random(cursor),
        (Mode::Command, '_') => branch(Direction::Left, Direction::Right, cursor),
        (Mode::Command, '|') => branch(Direction::Up, Direction::Down, cursor),
        (Mode::Command | Mode::String, '"') => cursor.toggle_mode().step().into(),
        (Mode::Command, ':') => Instruction::Duplicate.into_block(cursor),
        (Mode::Command, '\\') => Instruction::Swap.into_block(cursor),
        (Mode::Command, '$') => Instruction::Pop.into_block(cursor),
        (Mode::Command, '.') => Instruction::OutputInt.into_block(cursor),
        (Mode::Command, ',') => Instruction::OutputChar.into_block(cursor),
        (Mode::Command, '#') => cursor.step().step().into(),
        (Mode::Command, 'g') => Instruction::Get.into_block(cursor),
        (Mode::Command, 'p') => put(cursor),
        (Mode::Command, '&') => Instruction::InputInt.into_block(cursor),
        (Mode::Command, '~') => Instruction::InputChar.into_block(cursor),
        (Mode::Command, '@') => Exit::End.into_block(),
        (Mode::Command, _) => cursor.step().into(),
        (Mode::String, _) => push(value.into_i32(), cursor),
    }
}

/// Creates a new push block from a value and a cursor.
fn push(value: i32, cursor: Cursor) -> Block {
    Instruction::Push(value.into()).into_block(cursor)
}

/// Creates a new unary operation block from an operator and a cursor.
fn unary(op: UnOp, cursor: Cursor) -> Block {
    Instruction::Unary(op).into_block(cursor)
}

/// Creates a new binary operation block from an operator and a cursor.
fn binary(op: BinOp, cursor: Cursor) -> Block {
    Instruction::Binary(op).into_block(cursor)
}

/// Creates a new division operation block from an operator and a cursor.
fn divide(op: DivOp, cursor: Cursor) -> Block {
    Instruction::Divide(op).into_block(cursor)
}

/// Creates a new random block from a cursor.
fn random(cursor: Cursor) -> Block {
    let right_label = cursor.clone().go(Direction::Right).into();
    let down_label = cursor.clone().go(Direction::Down).into();
    let left_label = cursor.clone().go(Direction::Left).into();
    let up_label = cursor.go(Direction::Up).into();
    Exit::Random(right_label, down_label, left_label, up_label).into_block()
}

/// Creates a new branch block from directions and a cursor.
fn branch(then_direction: Direction, else_direction: Direction, cursor: Cursor) -> Block {
    let then_label = cursor.clone().go(then_direction).into();
    let else_label = cursor.go(else_direction).into();
    Exit::Branch(then_label, else_label).into_block()
}

/// Creates a new put block from a cursor.
fn put(cursor: Cursor) -> Block {
    let state: State = cursor.step().into();
    Block {
        instructions: vec![Instruction::Put(state.clone())],
        exit: Exit::Jump(Label::State(state)),
    }
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

    /// Returns an iterator over the states.
    fn states(&self) -> impl Iterator<Item = &State> {
        self.to_labels().into_iter().filter_map(Label::to_state)
    }
}

impl Label {
    /// Converts the label to a state. Returns `None` if the label does not have
    /// a state.
    fn to_state(&self) -> Option<&State> {
        match self {
            Self::Main => None,
            Self::State(s) => Some(s),
        }
    }
}
