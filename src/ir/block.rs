use std::fmt::{self, Display, Formatter};

use super::{Exit, Instruction};

/// A linear sequence of instructions with a single exit.
pub struct Block {
    /// The instructions.
    pub instructions: Vec<Instruction>,

    /// The exit.
    pub exit: Exit,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for instruction in &self.instructions {
            writeln!(f, "{instruction}")?;
        }

        self.exit.fmt(f)
    }
}
