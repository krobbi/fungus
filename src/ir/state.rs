use std::fmt::{self, Display, Formatter};

/// A Befunge program counter's state.
// Do not change the field order to be more 'pretty' - it allows the `Ord` trait
// to sort states in a user-friendly order. Ordering states also allows
// compilation and debug dumps to be deterministic.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    /// The Y coordinate in cells from the top edge of a playfield.
    pub y: usize,

    /// The X coordinate in cells from the left edge of a playfield.
    pub x: usize,

    /// The program counter mode.
    pub mode: Mode,

    /// The forward program counter direction.
    pub direction: Direction,
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

/// A Befunge program counter's mode.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// A mode where playfield values are executed as commands.
    #[default]
    Command,

    /// A mode where playfield values are pushed to the stack.
    String,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::Command => "command",
            Self::String => "string",
        };

        f.write_str(data)
    }
}

/// A Befunge program counter's direction.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// A direction where the X coordinate is incremented.
    #[default]
    Right,

    /// A direction where the Y coordinate is incremented.
    Down,

    /// A direction where the X coordinate is decremented.
    Left,

    /// A direction where the X coordinate is decremented.
    Up,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::Right => "right",
            Self::Down => "down",
            Self::Left => "left",
            Self::Up => "up",
        };

        f.write_str(data)
    }
}
