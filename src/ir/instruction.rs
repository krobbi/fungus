use std::fmt::{self, Display, Formatter};

use super::Expr;

/// A block's instruction.
pub enum Instruction {
    /// An instruction to push an expression to the stack.
    /// `[...]` -> `[...][expr]`
    Push(Expr),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(e) => write!(f, "{:8}{e}", "push"),
        }
    }
}
