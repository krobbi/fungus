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
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "halt"),
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
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negate => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Divide => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::Greater => write!(f, ">"),
            Self::Get => write!(f, "get"),
        }
    }
}

impl Display for AssocOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => write!(f, "+"),
            Self::Product => write!(f, "*"),
        }
    }
}
