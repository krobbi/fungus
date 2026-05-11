use std::fmt::{self, Display, Formatter, Write as _};

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
        }
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
            Self::Jump(label) => write!(f, "{:8}{label}", "jump"),
        }
    }
}
