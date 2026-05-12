use std::fmt::{self, Display, Formatter, Write as _};

use crate::state::{Direction, Mode, State};

use super::{BasicBlock, Cfg, Label, Terminator};

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
        write!(f, "{}", self.terminator)
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "halt"),
            Self::Jump(label) => write!(f, "{:8}{label}", "jump"),
            Self::Branch(then_label, else_label) => {
                write!(f, "{:8}{then_label} else {else_label}", "branch")
            }
        }
    }
}
