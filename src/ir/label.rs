use std::fmt::{self, Display, Formatter};

use crate::common::ProgramCounter;

/// A label referencing a basic block.
pub enum Label {
    /// A label for a basic block built at a program counter.
    ProgramCounter(ProgramCounter),
}

impl From<ProgramCounter> for Label {
    fn from(value: ProgramCounter) -> Self {
        Self::ProgramCounter(value)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProgramCounter(p) => p.fmt(f),
        }
    }
}
