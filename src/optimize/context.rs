use crate::ir::{Block, Exit, Label, Program};

// TODO: Consider separating the program from the context. The context isn't
// very useful because borrowing rules forbid it from being accessed while
// mutating the program.

/// Context for optimizing a program.
pub struct OptimizationContext<'a> {
    /// The program.
    program: &'a mut Program,

    /// Whether an optimization pass should be run.
    should_run_pass: bool,
}

impl<'a> OptimizationContext<'a> {
    /// Creates a new optimization context from a program.
    pub fn new(program: &'a mut Program) -> Self {
        Self {
            program,
            should_run_pass: true,
        }
    }

    /// Returns whether an optimization pass should be run.
    pub fn should_run_pass(&mut self) -> bool {
        let should_run_pass = self.should_run_pass;

        // Do not run another pass unless changes are made.
        self.should_run_pass = false;

        should_run_pass
    }

    /// Marks that a change was made to the program.
    pub fn mark_change(&mut self) {
        // Changes were made, so more optimization passes should be run.
        self.should_run_pass = true;
    }

    /// Returns an iterator over the program's labels.
    pub fn labels(&self) -> impl Iterator<Item = &Label> {
        self.program.blocks.keys()
    }

    /// Returns an iterator over the program's labels, except for a given label.
    pub fn labels_except(&self, label: &Label) -> impl Iterator<Item = &Label> {
        self.labels().filter(move |l| *l != label)
    }

    /// Returns a mutable iterator over the program's blocks.
    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut Block> {
        self.program.blocks.values_mut()
    }

    /// Returns a mutable reference to a block from its label.
    pub fn block_mut(&mut self, label: &Label) -> &mut Block {
        self.program
            .blocks
            .get_mut(label)
            .expect("label should exist in program")
    }

    /// Removes and returns a block from its label.
    pub fn remove_block(&mut self, label: &Label) -> Block {
        self.program
            .blocks
            .remove(label)
            .expect("label should exist in program")
    }

    /// Returns whether an edge exists between a predecessor label and a
    /// successor label.
    pub fn has_edge(&self, predecessor: &Label, successor: &Label) -> bool {
        self.program.blocks[predecessor]
            .exit
            .to_labels()
            .contains(&successor)
    }

    /// Returns the optional foreign (not self-referential) successor label
    /// reached from an unconditional jump from a predecessor label.
    pub fn foreign_jump_successor(&self, predecessor: &Label) -> Option<&Label> {
        match &self.program.blocks[predecessor].exit {
            Exit::Jump(l) if l != predecessor => Some(l),
            _ => None,
        }
    }
}

impl Exit {
    /// Converts the exit to a boxed slice of labels.
    fn to_labels(&self) -> Box<[&Label]> {
        match self {
            Self::Jump(l) => Box::new([l]),
            Self::Random(r, d, l, u) => Box::new([r, d, l, u]),
            Self::Branch(t, e) => Box::new([t, e]),
            Self::End => Box::new([]),
        }
    }
}
