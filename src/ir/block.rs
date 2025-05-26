use std::fmt::{self, Display, Formatter};

use super::Exit;

/// A linear sequence of instructions with a single exit.
pub struct Block {
    /// The exit.
    pub exit: Exit,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.exit.fmt(f)
    }
}
