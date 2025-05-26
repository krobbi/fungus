use crate::{
    common::{Playfield, Value},
    ir::{
        Exit, Label, State,
        state::{Direction, Mode},
    },
};

/// A state bound to a playfield.
#[derive(Clone)]
pub struct Cursor<'a> {
    /// The playfield.
    playfield: &'a Playfield,

    /// The state.
    state: State,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor from a playfield and a state.
    pub fn new(playfield: &'a Playfield, state: State) -> Self {
        let (width, height) = playfield.bounds();
        assert!(state.x < width && state.y < height);

        Self { playfield, state }
    }

    /// Returns the mode.
    pub fn mode(&self) -> Mode {
        self.state.mode
    }

    /// Returns the value under the cursor.
    pub fn value(&self) -> Value {
        self.playfield
            .get(self.state.x, self.state.y)
            .expect("cursor should be bound to playfield")
    }

    /// Moves the cursor forward by one cell.
    pub fn step(mut self) -> Self {
        let (coordinate, bound) = match self.state.direction {
            Direction::Right | Direction::Left => (&mut self.state.x, self.playfield.bounds().0),
            Direction::Down | Direction::Up => (&mut self.state.y, self.playfield.bounds().1),
        };

        match self.state.direction {
            Direction::Right | Direction::Down => *coordinate = (*coordinate + 1) % bound,
            Direction::Left | Direction::Up => {
                *coordinate = coordinate.checked_sub(1).unwrap_or(bound - 1);
            }
        }

        self
    }

    /// Moves the cursor in a direction by one cell.
    pub fn go(mut self, direction: Direction) -> Self {
        self.state.direction = direction;
        self.step()
    }

    /// Toggles the mode.
    pub fn toggle_mode(mut self) -> Self {
        self.state.mode = match self.state.mode {
            Mode::Command => Mode::String,
            Mode::String => Mode::Command,
        };

        self
    }
}

impl From<Cursor<'_>> for Exit {
    fn from(value: Cursor<'_>) -> Self {
        Self::Jump(value.into())
    }
}

impl From<Cursor<'_>> for Label {
    fn from(value: Cursor<'_>) -> Self {
        Self::State(value.state)
    }
}
