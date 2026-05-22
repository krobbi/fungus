use std::fmt::{self, Display, Formatter, Write as _};

use crate::state::{Direction, Mode, State};

use super::{AssocOp, BasicBlock, BinOp, Cfg, Expr, Instruction, Label, Terminator, UnOp};

impl Display for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for label in self.labels() {
            writeln!(buffer, "{label}:")?;

            for line in self.basic_block(label).to_string().lines() {
                writeln!(buffer, "{:8}{line}", "")?;
            }

            writeln!(buffer)?;
        }

        write!(f, "{}", buffer.trim_end())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => write!(f, "main"),
            Self::State(state) => write!(f, "{state}"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x{}_y{}_{}_{}",
            self.x, self.y, self.mode, self.direction
        )
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Command => "command",
            Self::String => "string",
        };

        write!(f, "{name}")
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Right => "right",
            Self::Down => "down",
            Self::Left => "left",
            Self::Up => "up",
        };

        write!(f, "{name}")
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut positions: Vec<_> = self.positions.iter().copied().collect();
        positions.sort_unstable_by_key(|(x, y)| (*y, *x));

        for chunk in positions.chunks(8) {
            let (first_x, first_y) = chunk[0];
            write!(f, "; ({first_x}, {first_y})")?;

            for (x, y) in &chunk[1..] {
                write!(f, ", ({x}, {y})")?;
            }

            writeln!(f)?;
        }

        for instruction in &self.instructions {
            writeln!(f, "{instruction}")?;
        }

        write!(f, "{}", self.terminator)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(expr) => write!(f, "{:16}{expr}", "push"),
            Self::Pop => write!(f, "pop"),
            Self::Duplicate => write!(f, "duplicate"),
            Self::Swap => write!(f, "swap"),
            Self::Unary(op) => write!(f, "{:16}{op}", "unary"),
            Self::Binary(op) => write!(f, "{:16}{op}", "binary"),
            Self::Assoc(op) => write!(f, "{:16}{op}", "assoc"),
            Self::OutputInt => write!(f, "output_int"),
            Self::OutputChar => write!(f, "output_char"),
            Self::Print(string) => write!(f, "{:16}{string:?}", "print"),
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "halt"),
            Self::InfiniteLoop => write!(f, "infinite_loop"),
            Self::Jump(label) => write!(f, "{:16}{label}", "jump"),
            Self::Branch(then_label, else_label) => {
                write!(f, "{:16}{then_label} else {else_label}", "branch")
            }
            Self::Random(right_label, down_label, left_label, up_label) => write!(
                f,
                "{:16}{right_label}, {down_label}, {left_label}, {up_label}",
                "random"
            ),
            Self::Put(label) => write!(f, "{:16}{label}", "put"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(value) => write!(f, "{value}"),
            Self::InputInt => write!(f, "input_int()"),
            Self::InputChar => write!(f, "input_char()"),
            Self::Unary(op, rhs) => fmt_unary_expr(f, *op, rhs),
            Self::Binary(op, lhs, rhs) => fmt_binary_expr(f, *op, lhs, rhs),
            Self::Assoc(op, terms) => fmt_assoc_expr(f, *op, terms),
            Self::Sequence(prefix, expr) => fmt_sequence_expr(f, prefix, expr),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Negate => "-",
            Self::Bool => "bool",
            Self::Not => "!",
        };

        write!(f, "{symbol}")
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Get => "get",
        };

        write!(f, "{symbol}")
    }
}

impl Display for AssocOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Sum => "+",
            Self::Product => "*",
        };

        write!(f, "{symbol}")
    }
}

/// Formats a unary [`Expr`] with a [`Formatter`].
fn fmt_unary_expr(f: &mut Formatter<'_>, op: UnOp, rhs: &Expr) -> fmt::Result {
    match (op, rhs) {
        (UnOp::Negate, Expr::Const(_)) | (UnOp::Bool, _) => write!(f, "{op}({rhs})"),
        (_, _) => write!(f, "{op}{rhs}"),
    }
}

/// Formats a binary [`Expr`] with a [`Formatter`].
fn fmt_binary_expr(f: &mut Formatter<'_>, op: BinOp, lhs: &Expr, rhs: &Expr) -> fmt::Result {
    match op {
        BinOp::Get => write!(f, "{op}({lhs}, {rhs})"),
        _ => write!(f, "({lhs} {op} {rhs})"),
    }
}

/// Formats an associative [`Expr`] with a [`Formatter`].
fn fmt_assoc_expr(f: &mut Formatter<'_>, op: AssocOp, terms: &[Expr]) -> fmt::Result {
    match terms {
        [] => write!(f, "(... {op} ...)"),
        [lhs] => write!(f, "({lhs} {op} ...)"),
        [lhs, rest @ ..] => {
            write!(f, "({lhs}")?;

            for arg in rest {
                write!(f, " {op} {arg}")?;
            }

            write!(f, ")")
        }
    }
}

/// Formats a sequence [`Expr`] with a [`Formatter`].
fn fmt_sequence_expr(f: &mut Formatter<'_>, prefix: &[Expr], expr: &Expr) -> fmt::Result {
    if prefix.is_empty() {
        write!(f, "(..., {expr})")
    } else {
        write!(f, "(")?;

        for prefix_expr in prefix {
            write!(f, "{prefix_expr}, ")?;
        }

        write!(f, "{expr})")
    }
}
