use std::fmt::{self, Display, Formatter};

use crate::value::Value;

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
            Self::Not => !rhs,
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Not => "!",
        };

        f.write_str(symbol)
    }
}
