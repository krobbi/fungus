use crate::ir::{Block, Exit, Label, Program};

/// A control flow graph.
pub struct Graph<'a> {
    /// The inner program.
    program: &'a mut Program,
}

impl<'a> Graph<'a> {
    /// Creates a new graph from a program.
    pub fn new(program: &'a mut Program) -> Self {
        Self { program }
    }

    /// Returns whether an edge exists between a predecessor label and a
    /// successor label.
    pub fn has_edge(&self, predecessor: &Label, successor: &Label) -> bool {
        self.program.blocks[predecessor]
            .exit
            .to_labels()
            .contains(&successor)
    }

    /// Returns the foreign (not self-referential) successor label reached from
    /// an unconditional jump from a predecessor label. Returns `None` if no
    /// foreign jump successor exists.
    pub fn foreign_jump_successor(&self, predecessor: &Label) -> Option<&Label> {
        match &self.program.blocks[predecessor].exit {
            Exit::Jump(l) if l != predecessor => Some(l),
            _ => None,
        }
    }

    /// Returns an iterator over the labels.
    pub fn labels(&self) -> impl Iterator<Item = &Label> {
        self.program.blocks.keys()
    }

    /// Returns a mutable iterator over the blocks.
    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut Block> {
        self.program.blocks.values_mut()
    }

    /// Returns a mutable reference to a block from its label.
    pub fn block_mut(&mut self, label: &Label) -> &mut Block {
        self.program
            .blocks
            .get_mut(label)
            .expect("label should exist")
    }

    /// Removes and returns a block from its label.
    pub fn remove_block(&mut self, label: &Label) -> Block {
        self.program
            .blocks
            .remove(label)
            .expect("label should exist")
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
