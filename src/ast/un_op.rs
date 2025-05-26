use std::fmt::{self, Display, Formatter, Write};

/// A unary operator.
#[derive(Clone, Copy)]
pub enum UnOp {
    /// A logical not operator.
    Not,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Not => '!',
        };

        f.write_char(c)
    }
}
