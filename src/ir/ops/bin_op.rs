use std::{
    fmt::{self, Display, Formatter, Write},
    num::Wrapping,
};

use crate::common::Value;

use super::DivOp;

/// A pure binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// A binary addition operator.
    Add,

    /// A binary subtraction operator.
    Subtract,

    /// A binary multiplication operator.
    Multiply,

    /// A binary logical greater than operator.
    Greater,

    /// A binary divide by non-zero operator.
    Divide,

    /// A binary modulo by non-zero operator.
    Modulo,
}

impl BinOp {
    /// Evaluates the binary operator with operands.
    pub fn eval(self, lhs: Value, rhs: Value) -> Value {
        let (lhs, rhs) = (Wrapping(lhs.into_i32()), Wrapping(rhs.into_i32()));

        let result = match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
            Self::Multiply => lhs * rhs,
            Self::Greater => Wrapping((lhs > rhs).into()),
            Self::Divide => lhs / rhs,
            Self::Modulo => lhs % rhs,
        };
        result.0.into()
    }
}

impl From<DivOp> for BinOp {
    fn from(value: DivOp) -> Self {
        match value {
            DivOp::Quotient => Self::Divide,
            DivOp::Remainder => Self::Modulo,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Add => '+',
            Self::Subtract => '-',
            Self::Multiply => '*',
            Self::Greater => '>',
            Self::Divide => '/',
            Self::Modulo => '%',
        };
        f.write_char(c)
    }
}
