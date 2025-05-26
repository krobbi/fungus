mod cursor;

use std::collections::{BTreeMap, BTreeSet};

use cursor::Cursor;

use crate::{
    common::Playfield,
    ir::{
        Block, Exit, Label, Program, State,
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
        Block {
            exit: Exit::Jump(Label::State(main_state.clone())),
        },
    );

    let mut unexplored_states = BTreeSet::new();
    unexplored_states.insert(main_state);

    while let Some(state) = unexplored_states.pop_first() {
        let label = Label::State(state.clone());
        if program.blocks.contains_key(&label) {
            continue;
        }

        let cursor = Cursor::new(playfield, state);
        let exit = parse_exit(cursor);

        for state in exit.to_states() {
            unexplored_states.insert(state);
        }

        program.blocks.insert(label, Block { exit });
    }

    program
}

/// Parses an exit point from a cursor.
fn parse_exit(cursor: Cursor) -> Exit {
    let value = cursor.value();
    let cursor = match (cursor.mode(), value.into_printable_ascii_char_lossy()) {
        (Mode::Command, '>') => cursor.go(Direction::Right),
        (Mode::Command, '<') => cursor.go(Direction::Left),
        (Mode::Command, '^') => cursor.go(Direction::Up),
        (Mode::Command, 'v') => cursor.go(Direction::Down),
        (Mode::Command, '?') => return random(cursor),
        (Mode::Command, '_') => return branch(cursor, Direction::Left, Direction::Right),
        (Mode::Command, '|') => return branch(cursor, Direction::Up, Direction::Down),
        (Mode::Command | Mode::String, '"') => cursor.toggle_mode().step(),
        (Mode::Command, '#') => cursor.step().step(),
        (Mode::Command, '@') => return Exit::End,
        (Mode::Command | Mode::String, _) => cursor.step(),
    };

    cursor.into()
}

/// Creates a random exit from a cursor.
fn random(cursor: Cursor) -> Exit {
    let right_label = cursor.clone().go(Direction::Right).into();
    let down_label = cursor.clone().go(Direction::Down).into();
    let left_label = cursor.clone().go(Direction::Left).into();
    let up_label = cursor.go(Direction::Up).into();
    Exit::Random(right_label, down_label, left_label, up_label)
}

/// Creates a branch exit from a cursor and directions.
fn branch(cursor: Cursor, then_direction: Direction, else_direction: Direction) -> Exit {
    let then_label = cursor.clone().go(then_direction).into();
    let else_label = cursor.go(else_direction).into();
    Exit::Branch(then_label, else_label)
}

impl Exit {
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
