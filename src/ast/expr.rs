use std::fmt::{self, Display, Formatter};

use crate::common::Value;

/// An expression.
pub enum Expr {
    /// A literal value expression.
    Literal(Value),

    /// An integer input expression.
    InputInt,

    /// A character input expression.
    InputChar,
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(v) => v.into_i32().fmt(f),
            Self::InputInt => f.write_str("input_int()"),
            Self::InputChar => f.write_str("input_char()"),
        }
    }
}
