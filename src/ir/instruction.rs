use std::fmt::{self, Display, Formatter};

use crate::ast::{BinOp, Expr, UnOp};

/// An instruction in a block.
pub enum Instruction {
    /// An instruction to push an expression to the stack.
    /// `[...]` -> `[...][expr]`
    Push(Expr),

    /// An instruction to apply a binary operation to the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs op rhs]`
    Binary(BinOp),

    /// An instruction to apply a unary operation to the stack.
    /// `[...][rhs]` -> `[...][op rhs]`
    Unary(UnOp),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(e) => write!(f, "{:8}{e}", "push"),
            Self::Binary(o) => write!(f, "{:8}{o}", "binary"),
            Self::Unary(o) => write!(f, "{:8}{o}", "unary"),
        }
    }
}
