use std::fmt::{self, Display, Formatter};

use crate::common::Value;

use super::{BinOp, UnOp};

/// An expression.
#[derive(Clone)]
pub enum Expr {
    /// A literal value expression.
    Literal(Value),

    /// An integer input expression.
    InputInt,

    /// A character input expression.
    InputChar,

    /// A binary expression.
    Binary(BinOp, Box<Self>, Box<Self>),

    /// A unary expression.
    Unary(UnOp, Box<Self>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(v) => v.into_i32().fmt(f),
            Self::InputInt => f.write_str("input_int()"),
            Self::InputChar => f.write_str("input_char()"),
            Self::Binary(o, l, r) => write!(f, "({l} {o} {r})"),
            Self::Unary(o, r) => write!(f, "{o}{r}"),
        }
    }
}
