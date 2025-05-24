use crate::common::{
    Playfield, ProgramCounter,
    program_counter::{Direction, Mode},
};

use super::{BasicBlock, ExitPoint};

/// Builds a basic block at a program counter in a playfield.
pub fn build_basic_block(playfield: &Playfield, program_counter: &ProgramCounter) -> BasicBlock {
    let value = playfield
        .get(program_counter.position())
        .expect("program counter should be bound to playfield");
    let bounds = playfield.bounds();

    let exit_point = match (
        program_counter.mode(),
        value.into_printable_ascii_char_lossy(),
    ) {
        (Mode::Command, c) => build_exit_point(c, program_counter, bounds),
        (Mode::String, '"') => program_counter
            .with_mode(Mode::Command)
            .moved_forward(bounds)
            .into(),
        (Mode::String, _) => program_counter.moved_forward(bounds).into(),
    };

    BasicBlock { exit_point }
}

/// Builds an exit point from a command and a program counter with bounds.
fn build_exit_point(
    command: char,
    program_counter: &ProgramCounter,
    bounds: (usize, usize),
) -> ExitPoint {
    match command {
        '>' => program_counter
            .moved_in_direction(Direction::Right, bounds)
            .into(),
        '<' => program_counter
            .moved_in_direction(Direction::Left, bounds)
            .into(),
        '^' => program_counter
            .moved_in_direction(Direction::Up, bounds)
            .into(),
        'v' => program_counter
            .moved_in_direction(Direction::Down, bounds)
            .into(),
        '"' => program_counter
            .with_mode(Mode::String)
            .moved_forward(bounds)
            .into(),
        '#' => program_counter
            .moved_forward(bounds)
            .moved_forward(bounds)
            .into(),
        _ => program_counter.moved_forward(bounds).into(),
    }
}
