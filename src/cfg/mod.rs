mod display;

use std::collections::{HashMap, HashSet};

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
    pub fn labels(&self) -> impl Iterator<Item = Label> {
        let mut labels: Vec<_> = self.labels_unstable().collect();
        labels.sort_unstable();
        labels.into_iter()
    }

    /// Returns an [`Iterator`] over the `Cfg`'s [`Label`]s in an arbitrary
    /// order.
    pub fn labels_unstable(&self) -> impl Iterator<Item = Label> {
        self.basic_blocks.keys().copied()
    }

    /// Returns a reference to a [`BasicBlock`] from its [`Label`].
    pub fn basic_block(&self, label: Label) -> &BasicBlock {
        &self.basic_blocks[&label]
    }

    /// Returns a mutable reference to a [`BasicBlock`] from its [`Label`].
    pub fn basic_block_mut(&mut self, label: Label) -> &mut BasicBlock {
        self.basic_blocks
            .get_mut(&label)
            .expect("label should exist")
    }

    /// Returns an [`Iterator`] over mutable references to the `Cfg`'s
    /// [`BasicBlock`]s in an arbitrary order.
    pub fn basic_blocks_mut_unstable(&mut self) -> impl Iterator<Item = &mut BasicBlock> {
        self.basic_blocks.values_mut()
    }

    /// Inserts a [`BasicBlock`] into the `Cfg` with a [`Label`].
    pub fn insert_basic_block(&mut self, label: Label, basic_block: BasicBlock) {
        let old_basic_block = self.basic_blocks.insert(label, basic_block);
        debug_assert!(old_basic_block.is_none(), "label already exists");
    }

    /// Removes and returns a [`BasicBlock`] from the `Cfg` from its [`Label`].
    pub fn remove_basic_block(&mut self, label: Label) -> BasicBlock {
        self.basic_blocks
            .remove(&label)
            .expect("label should exist")
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
    /// The [`Playfield`][crate::playfield::Playfield] positions.
    pub positions: HashSet<(u16, u16)>,

    /// The [`Instruction`]s.
    pub instructions: Vec<Instruction>,

    /// The [`Terminator`].
    pub terminator: Terminator,
}

/// An instruction which must not terminate a [`BasicBlock`].
#[derive(Clone, Debug)]
pub enum Instruction {
    /// Evaluate an [`Expr`] and push its [`Value`] to the stack.
    Push(Expr),

    /// Pop a [`Value`] from the stack and discard it.
    Pop,

    /// Pop a [`Value`] from the stack and push it to the stack twice.
    Duplicate,

    /// Pop two [`Value`]s from the stack and push them to the stack in reverse
    /// order.
    Swap,

    /// Pop a [`Value`] from the stack, apply a [`UnOp`] to it, and push it to
    /// the stack.
    Unary(UnOp),

    /// Pop a right-hand side [`Value`] from the stack, then a left-hand side
    /// [`Value`], apply a [`BinOp`] to them, and push the result [`Value`] to
    /// the stack.
    Binary(BinOp),

    /// Pop two [`Value`]s from the stack, apply an [`AssocOp`] to them, and
    /// push the result [`Value`] to the stack.
    Assoc(AssocOp),

    /// Pop a [`Value`] from the stack and output it as an integer with a
    /// trailing space.
    OutputInt,

    /// Pop a [`Value`] from the stack and output it as a character.
    OutputChar,

    /// Print a [`String`].
    Print(String),
}

/// A [`BasicBlock`]'s terminator.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum Expr {
    /// A constant [`Value`].
    Const(Value),

    /// An integer [`Value`] from user input.
    InputInt,

    /// A character [`Value`] from user input.
    InputChar,

    /// A unary expression.
    Unary(UnOp, Box<Self>),

    /// A binary expression.
    Binary(BinOp, Box<Self>, Box<Self>),

    /// An associative expression.
    Assoc(AssocOp, Vec<Self>),
}

/// A unary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    /// An arithmetic negation.
    Negate,

    /// A Boolean cast.
    Bool,

    /// A logical negation.
    Not,
}

/// A binary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    /// A quotient division.
    Divide,

    /// A remainder division.
    Modulo,

    /// A greater than comparison.
    Greater,

    /// A greater than or equal comparison.
    GreaterEqual,

    /// A less than comparison.
    Less,

    /// A less than or equal comparison.
    LessEqual,

    /// A [`Playfield`][crate::playfield::Playfield] access.
    Get,
}

/// An associative and commutative operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssocOp {
    /// A sum.
    Sum,

    /// A product.
    Product,
}
