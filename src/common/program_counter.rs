use std::fmt::{self, Display, Formatter};

/// A Befunge program counter.
// Do not change the field order to be more 'pretty' - it allows the `Ord` trait
// to sort program counters in a user-friendly order. Ordering program counters
// also allows compilation and debug dumps to be deterministic.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramCounter {
    /// The Y coordinate in cells from the top edge of a playfield.
    y: usize,

    /// The X coordinate in cells from the left edge of a playfield.
    x: usize,

    /// The mode.
    mode: Mode,

    /// The forward direction.
    direction: Direction,
}

impl ProgramCounter {
    /// Returns the position in cells from the top-left corner of a playfield.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns the mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns a clone of the program counter with a mode.
    pub fn with_mode(&self, mode: Mode) -> Self {
        let mut program_counter = self.clone();
        program_counter.mode = mode;
        program_counter
    }

    /// Returns a clone of the program counter moved forward by one cell with
    /// bounds in cells.
    pub fn moved_forward(&self, bounds: (usize, usize)) -> Self {
        self.moved_in_direction(self.direction, bounds)
    }

    /// Returns a clone of the program counter moved in a direction by one cell
    /// with bounds in cells.
    pub fn moved_in_direction(&self, direction: Direction, bounds: (usize, usize)) -> Self {
        let mut program_counter = self.clone();
        program_counter.direction = direction;

        let (coordinate, max_coordinate) = match direction {
            Direction::Right | Direction::Left => (&mut program_counter.x, bounds.0 - 1),
            Direction::Down | Direction::Up => (&mut program_counter.y, bounds.1 - 1),
        };

        match direction {
            Direction::Right | Direction::Down => {
                *coordinate = if *coordinate < max_coordinate {
                    *coordinate + 1
                } else {
                    0
                }
            }
            Direction::Left | Direction::Up => {
                *coordinate = coordinate.checked_sub(1).unwrap_or(max_coordinate);
            }
        }

        program_counter
    }
}

impl Display for ProgramCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x{}_y{}_{}_{}",
            self.x, self.y, self.mode, self.direction
        )
    }
}

/// A Befunge program counter direction.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// A direction where the X coordinate is incremented.
    #[default]
    Right,

    /// A direction where the Y coordinate is incremented.
    Down,

    /// A direction where the X coordinate is decremented.
    Left,

    /// A direction where the Y coordinate is decemented.
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

/// A Befunge program counter mode.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// A mode where cells are executed as commands.
    #[default]
    Command,

    /// A mode where cell values are pushed to the stack.
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
