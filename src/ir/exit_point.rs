use std::fmt::{self, Display, Formatter};

use crate::common::ProgramCounter;

use super::Label;

/// A basic block's exit point.
pub enum ExitPoint {
    /// An unconditional jump to a basic block.
    Jump(Label),

    /// A conditional branch to one of two basic blocks.
    Branch(Label, Label),

    /// A program ending.
    End,
}

impl From<ProgramCounter> for ExitPoint {
    fn from(value: ProgramCounter) -> Self {
        Self::Jump(value.into())
    }
}

impl Display for ExitPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jump(l) => write!(f, "{:8}{l}", "jump"),
            Self::Branch(t, e) => write!(f, "{:8}{t}, {e}", "branch"),
            Self::End => f.write_str("end"),
        }
    }
}
