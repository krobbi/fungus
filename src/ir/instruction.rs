use std::fmt::{self, Display, Formatter};

use crate::ast::{BinOp, Expr, UnOp};

/// An instruction in a block.
pub enum Instruction {
    /// An instruction to push an expression.
    /// `[...]` -> `[...][expr]`
    Push(Expr),

    /// An instruction to apply a binary operation.
    /// `[...][lhs][rhs]` -> `[...][lhs op rhs]`
    Binary(BinOp),

    /// An instruction to apply a unary operation.
    /// `[...][rhs]` -> `[...][op rhs]`
    Unary(UnOp),

    /// An instruction to duplicate the top value of the stack.
    /// `[...][value]` -> `[...][value][value]`
    Duplicate,

    /// An instruction to swap the top two values of the stack.
    /// `[...][under][top]` -> `[...][top][under]`
    Swap,

    /// An instruction to pop a value and discard it.
    /// `[...][dropped]` -> `[...]`
    Pop,

    /// An instruction to pop a value and output it as an integer.
    /// `[...][int]` -> `[...]`
    OutputInt,

    /// An instruction to pop a value and output it as a character.
    /// `[...][char]` -> `[...]`
    OutputChar,

    /// An instruction to get a value from the playfield at a position.
    /// `[...][x][y]` -> `[...][value]`
    Get,

    /// An instruction to put a value to the playfield at a position.
    /// `[...][value][x][y]` -> `[...]`
    Put,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::Push(e) => return write!(f, "{:8}{e}", "push"),
            Self::Binary(o) => return write!(f, "{:8}{o}", "binary"),
            Self::Unary(o) => return write!(f, "{:8}{o}", "unary"),
            Self::Duplicate => "dup",
            Self::Swap => "swap",
            Self::Pop => "pop",
            Self::OutputInt => "outint",
            Self::OutputChar => "outchar",
            Self::Get => "get",
            Self::Put => "put",
        };

        f.write_str(data)
    }
}
