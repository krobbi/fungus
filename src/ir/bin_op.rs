use std::fmt::{self, Display, Formatter, Write};

/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// An addition operator.
    Add,

    /// A subtraction operator.
    Subtract,

    /// A multiplication operator.
    Multiply,

    /// A division operator.
    Divide,

    /// A modulo operator.
    Modulo,

    /// A greater than operator.
    Greater,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Add => '+',
            Self::Subtract => '-',
            Self::Multiply => '*',
            Self::Divide => '/',
            Self::Modulo => '%',
            Self::Greater => '>',
        };

        f.write_char(c)
    }
}
