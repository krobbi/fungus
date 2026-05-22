use crate::{
    cfg::{AssocOp, BinOp, Expr, Instruction, UnOp},
    value::Value,
};

impl Instruction {
    /// Returns [`true`] if the `Instruction` has no visible side effects.
    pub fn is_quiet(&self) -> bool {
        match self {
            Self::Push(expr) => expr.is_read_only(),
            Self::Pop
            | Self::Duplicate
            | Self::Swap
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Assoc(_) => true,
            Self::OutputInt | Self::OutputChar | Self::Print(_) => false,
        }
    }
}

impl Expr {
    /// Returns [`true`] if the `Expr` has no side effects.
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

    /// Evaluates and returns the `Expr`'s constant [`Value`]. This function
    /// returns [`None`] if the `Expr` has no known constant [`Value`].
    pub fn eval_const(&self) -> Option<Value> {
        match self {
            Self::Const(value) => Some(*value),
            Self::InputInt | Self::InputChar => None,
            Self::Unary(op, rhs) => {
                let rhs = rhs.eval_const()?;
                Some(op.eval(rhs))
            }
            Self::Binary(op, lhs, rhs) => {
                let lhs = lhs.eval_const()?;
                let rhs = rhs.eval_const()?;
                op.eval_const(lhs, rhs)
            }
            Self::Assoc(op, terms) => {
                let mut value = op.identity();

                for term in terms {
                    let term = term.eval_const()?;
                    value = op.eval(value, term);
                }

                Some(value)
            }
            Self::Sequence(_, expr) => expr.eval_const(),
        }
    }
}

impl UnOp {
    /// Evaluates and returns the `UnOp`'s result [`Value`] from an operand
    /// [`Value`].
    pub fn eval(self, rhs: Value) -> Value {
        match self {
            Self::Negate => -rhs,
            Self::Bool => rhs.is_non_zero().into(),
            Self::Not => !rhs,
        }
    }
}

impl BinOp {
    /// Evaluates and returns the `BinOp`'s contant result [`Value`] from
    /// operand [`Value`]s. This function returns [`None`] if the `BinOp` could
    /// not be evaluated to a constant [`Value`].
    pub fn eval_const(self, lhs: Value, rhs: Value) -> Option<Value> {
        let value = match self {
            Self::Divide => lhs / rhs,
            Self::Modulo => lhs % rhs,
            Self::Greater => (lhs > rhs).into(),
            Self::GreaterEqual => (lhs >= rhs).into(),
            Self::Less => (lhs < rhs).into(),
            Self::LessEqual => (lhs <= rhs).into(),
            Self::Get => return None,
        };

        Some(value)
    }
}

impl AssocOp {
    /// Returns the `AssocOp`'s identity [`Value`].
    pub const fn identity(self) -> Value {
        match self {
            Self::Sum => Value(0),
            Self::Product => Value(1),
        }
    }

    /// Evaluates and returns the `AssocOp`'s result [`Value`] from two term
    /// [`Value`]s.
    pub fn eval(self, lhs: Value, rhs: Value) -> Value {
        match self {
            Self::Sum => lhs + rhs,
            Self::Product => lhs * rhs,
        }
    }
}
