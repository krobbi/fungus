/// A Befunge program counter's state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct State {
    /// The Y position in cells.
    pub y: u16,

    /// The X position in cells.
    pub x: u16,

    /// The [`Mode`].
    pub mode: Mode,

    /// The [`Direction`].
    pub direction: Direction,
}

/// A [`State`]'s mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mode {
    /// Execute [`Value`][crate::value::Value]s as commands.
    #[default]
    Command,

    /// Push [`Value`][crate::value::Value]s to the stack.
    String,
}

/// A [`State`]'s direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    /// Facing right.
    #[default]
    Right,

    /// Facing down.
    Down,

    /// Facing left.
    Left,

    /// Facing up.
    Up,
}
