use crate::cfg::{Expr, Instruction};

impl Instruction {
    /// Returns [`true`] if the [`Instruction`] has no visible side effects.
    pub fn is_quiet(&self) -> bool {
        match self {
            Self::Push(expr) => expr.is_read_only(),
            Self::Pop
            | Self::Duplicate
            | Self::Swap
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Assoc(_) => false,
            Self::OutputInt | Self::OutputChar | Self::Print(_) => true,
        }
    }
}

impl Expr {
    /// Returns [`true`] if the [`Expr`] has no side effects.
    pub fn is_read_only(&self) -> bool {
        match self {
            Self::Const(_) => true,
            Self::InputInt | Self::InputChar => false,
            Self::Unary(_, rhs) => rhs.is_read_only(),
            Self::Binary(_, lhs, rhs) => lhs.is_read_only() && rhs.is_read_only(),
            Self::Assoc(_, terms) => terms.iter().all(Self::is_read_only),
            Self::Sequence(prefix, expr) => {
                prefix.iter().all(Self::is_read_only) && expr.is_read_only()
            }
        }
    }
}
