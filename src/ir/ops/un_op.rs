use std::fmt::{self, Display, Formatter, Write};

use crate::common::Value;

/// A pure unary operator.
#[derive(Clone, Copy)]
pub enum UnOp {
    /// A unary logical negation operator.
    Not,
}

impl UnOp {
    /// Evaluates the unary operator with an operand.
    pub fn eval(self, rhs: Value) -> Value {
        match self {
            Self::Not => i32::from(rhs.into_i32() == 0).into(),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Not => '!',
        };
        f.write_char(c)
    }
}
