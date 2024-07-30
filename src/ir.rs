use std::fmt;

use crate::{playfield::Playfield, pointer::Pointer};

/// A basic block.
pub struct Block {
    /// The exit point.
    exit: Exit,
}

impl Block {
    /// Create a new basic block from a playfield and a pointer.
    pub fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        Self {
            exit: Exit::new(playfield, pointer),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.exit.fmt(f)
    }
}

/// A basic block's exit point.
pub enum Exit {
    /// An unconditional jump to a basic block.
    Jump(Pointer),
}

impl Exit {
    /// Create a new exit from a playfield and a pointer.
    fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        let mut pointer = pointer.clone();
        pointer.advance(playfield);
        let pointer = pointer;
        Self::Jump(pointer)
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(pointer) => write!(f, "goto {pointer};"),
        }
    }
}
