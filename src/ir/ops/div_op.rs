use std::fmt::{self, Display, Formatter, Write};

/// A binary division operator with side effects if the right-hand operand is
/// zero.
#[derive(Clone, Copy)]
pub enum DivOp {
    /// A binary quotient division operator `/`.
    Quotient,

    /// A binary remainder division operator `%`.
    Remainder,
}

impl Display for DivOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Quotient => '/',
            Self::Remainder => '%',
        };
        f.write_char(c)
    }
}
