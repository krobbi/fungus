use crate::cfg::Expr;

impl Expr {
    /// Returns [`true`] if the [`Expr`] has no side effects.
    pub fn is_read_only(&self) -> bool {
        match self {
            Self::Const(_) => true,
            Self::InputInt | Self::InputChar => false,
            Self::Unary(_, rhs) => rhs.is_read_only(),
            Self::Binary(_, lhs, rhs) => lhs.is_read_only() && rhs.is_read_only(),
            Self::Assoc(_, terms) => terms.iter().all(Self::is_read_only),
        }
    }
}
