use std::fmt::{self, Display, Formatter};

use super::ExitPoint;

/// A linear sequence of instructions with a single exit point.
pub struct BasicBlock {
    /// The exit point.
    pub(super) exit_point: ExitPoint,
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.exit_point.fmt(f)
    }
}
