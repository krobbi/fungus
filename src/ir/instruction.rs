use std::fmt::{self, Display, Formatter};

use crate::common::Value;

use super::{
    State,
    ops::{BinOp, DivOp, UnOp},
};

/// An instruction in a block.
#[derive(Clone)]
pub enum Instruction {
    /// An instruction to push a value to the stack.
    /// `[...]` -> `[...][value]`
    Push(Value),

    /// An instruction to apply a pure unary operator to the top value of the
    /// stack.
    /// `[...][rhs]` -> `[...][op rhs]`
    Unary(UnOp),

    /// An instruction to apply a pure binary operator to the top two values of
    /// the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs op rhs]`
    Binary(BinOp),

    /// An instruction to apply a division operator to the top two values of the
    /// stack that has side effects if the right-hand operand is zero.
    /// `[...][lhs][rhs]` -> `[...][lhs op rhs]`
    Divide(DivOp),

    /// An instruction to pop a value from the stack and push it to the stack
    /// twice.
    /// `[...][value]` -> `[...][value][value]`
    Duplicate,

    /// An instruction to pop the top two values of the stack and push them to
    /// the stack in reverse order.
    /// `[...][under][top]` -> `[...][top][under]`
    Swap,

    /// An instruction to pop a value from the stack and discard it.
    /// `[...][popped]` -> `[...]`
    Pop,

    /// An instruction to pop a value from the stack and output it as an
    /// integer.
    /// `[...][int]` -> `[...]`
    OutputInt,

    /// An instruction to pop a value from the stack and output it as a
    /// character.
    /// `[...][char]` -> `[...]`
    OutputChar,

    /// An instruction to pop two coordinate values from the stack and push the
    /// value from the playfield at the coordinates to the stack.
    /// `[...][x][y]` -> `[...][value]`
    Get,

    /// An instruction to pop two coordinate values and a stored value from the
    /// stack and store the stored value in the playfield at the coordinates.
    /// `[...][value][x][y]` -> `[...]`
    Put(State),

    /// An instruction to push an integer value to the stack from user input.
    /// `[...]` -> `[...][int]`
    InputInt,

    /// An instruction to push a character value to the stack from user input.
    /// `[...]` -> `[...][char]`
    InputChar,

    /// An instruction to output a string with no stack effect.
    Print(String),

    /// An instruction to push a value from the playfield at known, in-bounds
    /// coordinates to the stack.
    /// `[...]` -> `[...][value]`
    GetAt(usize, usize),

    /// An instruction to pop a value from the stack and store it in the
    /// playfield at known, in-bounds, non-modifying coordinates.
    /// `[...][value]` -> `[...]`
    PutAt(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::Push(v) => return write!(f, "{:8}{}", "push", v.into_i32()),
            Self::Unary(o) => return write!(f, "{:8}{o}", "unary"),
            Self::Binary(o) => return write!(f, "{:8}{o}", "binary"),
            Self::Divide(o) => return write!(f, "{:8}{o}", "divide"),
            Self::Duplicate => "dup",
            Self::Swap => "swap",
            Self::Pop => "pop",
            Self::OutputInt => "outint",
            Self::OutputChar => "outchar",
            Self::Get => "get",
            Self::Put(s) => return write!(f, "{:8}{s}", "put"),
            Self::InputInt => "inint",
            Self::InputChar => "inchar",
            Self::Print(s) => return write!(f, "{:8}\"{}\"", "print", s.escape_default()),
            Self::GetAt(x, y) => return write!(f, "{:8}{x}, {y}", "getat"),
            Self::PutAt(x, y) => return write!(f, "{:8}{x}, {y}", "putat"),
        };
        f.write_str(data)
    }
}
