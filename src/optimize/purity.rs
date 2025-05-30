use crate::ast::{BinOp, Expr};

/// An expression's purity level.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Purity {
    /// A purity level of an expression that may have side effects.
    Impure,

    /// A purity level of an expression with no side effects, but with a
    /// dependency on non-local data, such as stack or playfield values.
    /// Partially pure expressions may produce different values each time they
    /// are evaluated.
    PartiallyPure,

    /// A purity level of an expression with no side effects and no data
    /// dependencies. Pure expressions should evaluate to a constant value.
    Pure,
}

impl Expr {
    /// Returns whether the expression can be statically popped.
    pub fn can_pop(&self) -> bool {
        self.purity() >= Purity::PartiallyPure
    }

    /// Returns the expression's purity level.
    fn purity(&self) -> Purity {
        match self {
            Self::Literal(_) => Purity::Pure,
            Self::InputInt
            | Self::InputChar
            | Self::Binary(BinOp::Divide | BinOp::Modulo, _, _) => Purity::Impure,
            Self::Binary(_, l, r) => l.purity().min(r.purity()),
            Self::Unary(_, r) => r.purity(),
        }
    }
}
