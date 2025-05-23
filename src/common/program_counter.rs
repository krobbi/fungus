use std::fmt::{self, Display, Formatter};

/// A Befunge program counter.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramCounter {
    // Do not change the order of these fields to be more 'pretty' - it allows
    // `#[derive(Ord)]` to sort program counters in a user-friendly order.
    // Ordering program counters also allows compilation and debug dumps to be
    // deterministic.

    /// The Y coordinate in cells from the top edge.
    y: usize,

    /// The X coordinate in cells from the left edge.
    x: usize,

    /// The mode.
    mode: Mode,

    /// The forward direction.
    direction: Direction,
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

/// Prints some program counters in order to show how they are ordered.
// TODO: Remove this temporary test function.
pub fn temp_test_ordering() {
    fn direction_program_counters(x: usize, y: usize, mode: Mode) -> Vec<ProgramCounter> {
        let mut program_counters = Vec::with_capacity(4);
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            program_counters.push(ProgramCounter {
                x,
                y,
                mode,
                direction,
            })
        }

        program_counters
    }

    fn mode_program_counters(x: usize, y: usize) -> Vec<ProgramCounter> {
        let mut program_counters = Vec::with_capacity(8);
        program_counters.append(&mut direction_program_counters(x, y, Mode::String));
        program_counters.append(&mut direction_program_counters(x, y, Mode::Command));

        program_counters
    }

    let mut program_counters = Vec::with_capacity(32);
    program_counters.append(&mut mode_program_counters(0, 0));
    program_counters.append(&mut mode_program_counters(0, 1));
    program_counters.append(&mut mode_program_counters(1, 0));
    program_counters.append(&mut mode_program_counters(1, 1));
    program_counters.sort();

    for program_counter in program_counters {
        println!("{program_counter}");
    }
}
