#![expect(
    clippy::zero_sized_map_values,
    reason = "basic blocks will be non-zero sized later"
)]

mod display;

use std::{collections::HashMap, vec::IntoIter};

/// A control flow graph.
#[derive(Debug)]
pub struct Cfg {
    /// The [`BasicBlock`]s.
    basic_blocks: HashMap<Label, BasicBlock>,
}

impl Cfg {
    /// Creates a new `Cfg`.
    pub fn new() -> Self {
        Self {
            basic_blocks: HashMap::new(),
        }
    }

    /// Returns a sorted [`Iterator`] over the `Cfg`'s [`Label`]s.
    pub fn labels(&self) -> IntoIter<Label> {
        let mut labels: Vec<_> = self.basic_blocks.keys().copied().collect();
        labels.sort_unstable();
        labels.into_iter()
    }

    /// Returns a reference to a [`BasicBlock`] from its [`Label`].
    pub fn basic_block(&self, label: Label) -> &BasicBlock {
        &self.basic_blocks[&label]
    }

    /// Inserts a [`BasicBlock`] into the `Cfg` with a [`Label`].
    pub fn insert_basic_block(&mut self, label: Label, basic_block: BasicBlock) {
        let old_basic_block = self.basic_blocks.insert(label, basic_block);
        debug_assert!(old_basic_block.is_none(), "label already exists");
    }
}

/// A label for a [`BasicBlock`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Label {
    /// The main entry point.
    Main,
}

/// A basic block.
#[derive(Debug)]
pub struct BasicBlock {
    /// The [`Terminator`].
    pub terminator: Terminator,
}

/// A [`BasicBlock`]'s terminator.
#[derive(Debug)]
pub enum Terminator {
    /// An unconditional jump to a [`Label`].
    Jump(Label),
}
