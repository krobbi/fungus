mod display;

use std::{collections::HashMap, vec::IntoIter};

use crate::{state::State, value::Value};

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

    /// Returns [`true`] if the `Cfg` contains a [`Label`].
    pub fn contains_label(&self, label: Label) -> bool {
        self.basic_blocks.contains_key(&label)
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

    /// A [`BasicBlock`] originating at a [`State`].
    State(State),
}

/// A basic block.
#[derive(Debug)]
pub struct BasicBlock {
    /// The [`Instruction`]s.
    pub instructions: Vec<Instruction>,

    /// The [`Terminator`].
    pub terminator: Terminator,
}

/// An instruction which must not terminate a [`BasicBlock`].
#[derive(Debug)]
pub enum Instruction {
    /// Evaluate an [`Expr`] and push its [`Value`] to the stack.
    Push(Expr),
}

/// A [`BasicBlock`]'s terminator.
#[derive(Debug)]
pub enum Terminator {
    /// Halt execution.
    Halt,

    /// Unconditionally jump to a [`Label`].
    Jump(Label),

    /// Conditionally branch to one of two [`Label`]s.
    Branch(Label, Label),

    /// Randomly branch to one of four [`Label`]s.
    Random(Label, Label, Label, Label),

    /// Put a [`Value`] which potentially causes self-modifying code to the
    /// [`Playfield`][crate::playfield::Playfield`].
    Put(Label),
}

impl Terminator {
    /// Returns a boxed slice of [`Label`]s targeted by the terminator.
    pub fn labels(&self) -> Box<[Label]> {
        match self {
            Self::Halt => Box::new([]),
            Self::Jump(label) | Self::Put(label) => Box::new([*label]),
            Self::Branch(then_label, else_label) => Box::new([*then_label, *else_label]),
            Self::Random(right_label, down_label, left_label, up_label) => {
                Box::new([*right_label, *down_label, *left_label, *up_label])
            }
        }
    }
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A constant [`Value`].
    Const(Value),
}
