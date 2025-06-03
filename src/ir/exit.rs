use std::fmt::{self, Display, Formatter};

use super::Label;

/// A block's exit.
#[derive(Clone)]
pub enum Exit {
    /// An unconditional jump to a block.
    Jump(Label),

    /// A random branch to one of four blocks.
    Random(Label, Label, Label, Label),

    /// A conditional branch to one of two blocks.
    Branch(Label, Label),

    /// A program ending.
    End,
}

impl Exit {
    /// Converts the exit to a boxed slice of labels.
    pub fn to_labels(&self) -> Box<[&Label]> {
        match self {
            Self::Jump(l) => Box::new([l]),
            Self::Random(r, d, l, u) => Box::new([r, d, l, u]),
            Self::Branch(t, e) => Box::new([t, e]),
            Self::End => Box::new([]),
        }
    }
}

impl Display for Exit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jump(l) => write!(f, "{:8}{l}", "jump"),
            Self::Random(right, down, left, up) => {
                write!(f, "{:8}{right}, {down}, {left}, {up}", "random")
            }
            Self::Branch(t, e) => write!(f, "{:8}{t}, {e}", "branch"),
            Self::End => f.write_str("end"),
        }
    }
}
