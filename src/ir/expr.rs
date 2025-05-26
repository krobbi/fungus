use std::fmt::{self, Display, Formatter};

use crate::common::Value;

/// An expression.
pub enum Expr {
    /// A literal value expression.
    Literal(Value),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(v) => v.into_i32().fmt(f),
        }
    }
}
