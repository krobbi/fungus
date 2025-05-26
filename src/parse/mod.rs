use std::collections::{BTreeMap, BTreeSet};

use crate::common::{
    Playfield, ProgramCounter,
    program_counter::{Direction, Mode},
};

use crate::ir::{Block, Exit, Label, Program};

/// Parses a program from a playfield.
pub fn parse_program(playfield: &Playfield) -> Program {
    let mut ctx = ParseContext::new(playfield, ProgramCounter::default());

    while let Some(program_counter) = ctx.unvisited_program_counters.pop_first() {
        ctx.parse_block(&program_counter);
    }

    ctx.into_program()
}

/// Parsing context for a program.
struct ParseContext<'a> {
    /// The playfield.
    playfield: &'a Playfield,

    /// The program.
    program: Program,

    /// Unvisited program counters for parsing blocks.
    unvisited_program_counters: BTreeSet<ProgramCounter>,
}

impl<'a> ParseContext<'a> {
    /// Creates a new parsing context from a playfield and a main program counter.
    fn new(playfield: &'a Playfield, main_program_counter: ProgramCounter) -> Self {
        let mut ctx = Self {
            playfield,
            program: Program {
                blocks: BTreeMap::new(),
            },
            unvisited_program_counters: BTreeSet::new(),
        };

        ctx.insert_block(
            Label::Main,
            Block {
                exit: main_program_counter.into(),
            },
        );

        ctx
    }

    /// Inserts a new block at a label.
    fn insert_block(&mut self, label: Label, block: Block) {
        for program_counter in block.exit.to_program_counters() {
            self.unvisited_program_counters.insert(program_counter);
        }

        self.program.blocks.insert(label, block);
    }

    /// Parses a block at a program counter.
    fn parse_block(&mut self, program_counter: &ProgramCounter) {
        let label = program_counter.clone().into();

        if self.program.blocks.contains_key(&label) {
            return;
        }

        let exit = self.parse_exit(program_counter);
        self.insert_block(label, Block { exit });
    }

    /// Parses an exit from a program counter.
    fn parse_exit(&self, program_counter: &ProgramCounter) -> Exit {
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
            (Mode::Command, '_') => {
                self.create_branch(program_counter, Direction::Left, Direction::Right)
            }
            (Mode::Command, '|') => {
                self.create_branch(program_counter, Direction::Up, Direction::Down)
            }
            (Mode::Command, '"') => program_counter
                .with_mode(Mode::String)
                .moved_forward(bounds)
                .into(),
            (Mode::Command, '#') => program_counter
                .moved_forward(bounds)
                .moved_forward(bounds)
                .into(),
            (Mode::Command, '@') => Exit::End,
            (Mode::String, '"') => program_counter
                .with_mode(Mode::Command)
                .moved_forward(bounds)
                .into(),
            (Mode::Command | Mode::String, _) => program_counter.moved_forward(bounds).into(),
        }
    }

    /// Creates a branch exit from a program counter and directions.
    fn create_branch(
        &self,
        program_counter: &ProgramCounter,
        then_direction: Direction,
        else_direction: Direction,
    ) -> Exit {
        let bounds = self.playfield.bounds();

        let then_label = program_counter
            .moved_in_direction(then_direction, bounds)
            .into();
        let else_label = program_counter
            .moved_in_direction(else_direction, bounds)
            .into();

        Exit::Branch(then_label, else_label)
    }

    /// Consumes the parse context and returns the program.
    fn into_program(self) -> Program {
        self.program
    }
}

impl Exit {
    /// Converts the exit to a vector of program counters.
    fn to_program_counters(&self) -> Vec<ProgramCounter> {
        match self {
            Self::Jump(l) => l.to_program_counter().into_iter().collect(),
            Self::Branch(t, e) => t
                .to_program_counter()
                .into_iter()
                .chain(e.to_program_counter())
                .collect(),
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
