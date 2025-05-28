use std::fmt::{self, Display, Formatter};

use super::State;

/// A label referencing a block.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Label {
    /// A label for the main entry point block.
    Main,

    /// A label for a block built at a state.
    State(State),
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => f.write_str("main"),
            Self::State(s) => s.fmt(f),
        }
    }
}
