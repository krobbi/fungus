use std::collections::{BTreeMap, BTreeSet};

use crate::common::{
    Playfield, ProgramCounter,
    program_counter::{Direction, Mode},
};

use super::{BasicBlock, ExitPoint, Label, Program};

/// Builds a program from a playfield.
pub fn build_program(playfield: &Playfield) -> Program {
    let mut ctx = BuildContext::new(playfield, ProgramCounter::default());

    while let Some(program_counter) = ctx.unvisited_program_counters.pop_first() {
        ctx.build_basic_block(&program_counter);
    }

    ctx.into_program()
}

/// Context for building a program.
struct BuildContext<'a> {
    /// The playfield.
    playfield: &'a Playfield,

    /// The program.
    program: Program,

    /// Unvisited program counters for building basic blocks.
    unvisited_program_counters: BTreeSet<ProgramCounter>,
}

impl<'a> BuildContext<'a> {
    /// Creates a new build context from a playfield and a main program counter.
    fn new(playfield: &'a Playfield, main_program_counter: ProgramCounter) -> Self {
        let mut ctx = Self {
            playfield,
            program: Program {
                basic_blocks: BTreeMap::new(),
            },
            unvisited_program_counters: BTreeSet::new(),
        };

        ctx.insert_basic_block(
            Label::Main,
            BasicBlock {
                exit_point: main_program_counter.into(),
            },
        );

        ctx
    }

    /// Inserts a new basic block at a label.
    fn insert_basic_block(&mut self, label: Label, basic_block: BasicBlock) {
        for program_counter in basic_block.exit_point.to_program_counters() {
            self.unvisited_program_counters.insert(program_counter);
        }

        self.program.basic_blocks.insert(label, basic_block);
    }

    /// Builds a basic block at a program counter.
    fn build_basic_block(&mut self, program_counter: &ProgramCounter) {
        let label = program_counter.clone().into();

        if self.program.basic_blocks.contains_key(&label) {
            return;
        }

        let exit_point = self.build_exit_point(program_counter);
        self.insert_basic_block(label, BasicBlock { exit_point });
    }

    /// Builds an exit point from a program counter.
    fn build_exit_point(&self, program_counter: &ProgramCounter) -> ExitPoint {
        let command = self
            .playfield
            .get(program_counter.position())
            .expect("program counter should be bound to playfield")
            .into_printable_ascii_char_lossy();
        let bounds = self.playfield.bounds();

        match (program_counter.mode(), command) {
            (Mode::Command, '>') => program_counter
                .moved_in_direction(Direction::Right, bounds)
                .into(),
            (Mode::Command, '<') => program_counter
                .moved_in_direction(Direction::Left, bounds)
                .into(),
            (Mode::Command, '^') => program_counter
                .moved_in_direction(Direction::Up, bounds)
                .into(),
            (Mode::Command, 'v') => program_counter
                .moved_in_direction(Direction::Down, bounds)
                .into(),
            (Mode::Command, '"') => program_counter
                .with_mode(Mode::String)
                .moved_forward(bounds)
                .into(),
            (Mode::Command, '#') => program_counter
                .moved_forward(bounds)
                .moved_forward(bounds)
                .into(),
            (Mode::Command, '@') => ExitPoint::End,
            (Mode::String, '"') => program_counter
                .with_mode(Mode::Command)
                .moved_forward(bounds)
                .into(),
            (Mode::Command | Mode::String, _) => program_counter.moved_forward(bounds).into(),
        }
    }

    /// Consumes the build context and returns the program.
    fn into_program(self) -> Program {
        self.program
    }
}

impl ExitPoint {
    /// Converts the exit point to a vector of program counters.
    fn to_program_counters(&self) -> Vec<ProgramCounter> {
        match self {
            Self::Jump(l) => l.to_program_counter().into_iter().collect(),
            Self::End => Vec::new(),
        }
    }
}

impl Label {
    /// Converts the label to an optional program counter.
    fn to_program_counter(&self) -> Option<ProgramCounter> {
        match self {
            Self::Main => None,
            Self::ProgramCounter(p) => Some(p.clone()),
        }
    }
}
