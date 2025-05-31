use std::fmt::{self, Display, Formatter};

use crate::common::Value;

/// An instruction in a block.
pub enum Instruction {
    /// An instruction to push a value to the stack.
    /// `[...]` -> `[...][value]`
    Push(Value),

    /// An instruction to add the top two values of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs + rhs]`
    Add,

    /// An instruction to subtract the top value of the stack from the
    /// second-top value of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs - rhs]`
    Subtract,

    /// An instruction to multiply the top two values of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs * rhs]`
    Multiply,

    /// An instruction to divide the second-top value of the stack by the top
    /// value of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs / rhs]`
    Divide,

    /// An instruction to evaluate the remainder of dividing the second-top
    /// value of the stack by the top value of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs % rhs]`
    Modulo,

    /// An instruction to logically negate the top value of the stack.
    /// `[...][rhs]` -> `[...][lhs == 0 ? 1 : 0]`
    Not,

    /// An instruction to compare the second-top value of the stack as greater
    /// than the top value of the stack.
    /// `[...][lhs][rhs]` -> `[...][lhs > rhs ? 1 : 0]`
    Greater,

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
    Put,

    /// An instruction to push an integer value to the stack from user input.
    /// `[...]` -> `[...][int]`
    InputInt,

    /// An instruction to push a character value to the stack from user input.
    /// `[...]` -> `[...][char]`
    InputChar,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::Push(v) => return write!(f, "{:8}{}", "push", v.into_i32()),
            Self::Add => "add",
            Self::Subtract => "sub",
            Self::Multiply => "mul",
            Self::Divide => "divide",
            Self::Modulo => "modulo",
            Self::Not => "not",
            Self::Greater => "greater",
            Self::Duplicate => "dup",
            Self::Swap => "swap",
            Self::Pop => "pop",
            Self::OutputInt => "outint",
            Self::OutputChar => "outchar",
            Self::Get => "get",
            Self::Put => "put",
            Self::InputInt => "inint",
            Self::InputChar => "inchar",
        };
        f.write_str(data)
    }
}
