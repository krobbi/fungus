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

    /// Returns an iterator over a label's exit labels.
    pub fn exit_labels(&self, label: &Label) -> impl Iterator<Item = &Label> {
        self.block(label).exit.to_labels().into_iter()
    }

    /// Returns whether an edge exists between a predecessor label and a
    /// successor label.
    pub fn has_edge(&self, predecessor: &Label, successor: &Label) -> bool {
        self.block(predecessor)
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

    /// Returns a cloned boxed slice of the labels.
    pub fn labels_cloned(&self) -> Box<[Label]> {
        self.labels().cloned().collect()
    }

    /// Returns a mutable iterator over the blocks.
    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut Block> {
        self.program.blocks.values_mut()
    }

    /// Returns a reference to a block from its label.
    pub fn block(&self, label: &Label) -> &Block {
        &self.program.blocks[label]
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
