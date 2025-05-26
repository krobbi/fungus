use std::fmt::{self, Display, Formatter};

use crate::common::ProgramCounter;

use super::Label;

/// A block's exit.
pub enum Exit {
    /// An unconditional jump to a block.
    Jump(Label),

    /// A conditional branch to one of two blocks.
    Branch(Label, Label),

    /// A program ending.
    End,
}

impl From<ProgramCounter> for Exit {
    fn from(value: ProgramCounter) -> Self {
        Self::Jump(value.into())
    }
}

impl Display for Exit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jump(l) => write!(f, "{:8}{l}", "jump"),
            Self::Branch(t, e) => write!(f, "{:8}{t}, {e}", "branch"),
            Self::End => f.write_str("end"),
        }
    }
}
