use crate::{
    cfg::{Label, Terminator},
    playfield::Playfield,
    state::{Direction, State},
    value::Value,
};

use super::Item;

/// A [`State`] bound to a [`Playfield`].
#[derive(Clone, Copy)]
pub struct Cursor<'ply> {
    /// The [`Playfield`].
    playfield: &'ply Playfield,

    /// The [`State`].
    state: State,
}

impl<'ply> Cursor<'ply> {
    /// Creates a new `Cursor` from a [`Playfield`] and a [`State`].
    pub const fn new(playfield: &'ply Playfield, state: State) -> Self {
        Self { playfield, state }
    }

    /// Returns the [`Value`] under the cursor.
    pub fn value(self) -> Value {
        self.playfield
            .value(self.state.x, self.state.y)
            .expect("cursor should be in bounds of playfield")
    }

    /// Returns a copy of the `Cursor` moved forward by one cell.
    pub fn step(mut self) -> Self {
        let (coordinate, bound) = match self.state.direction {
            Direction::Right | Direction::Left => (&mut self.state.x, self.playfield.bounds().0),
            Direction::Down | Direction::Up => (&mut self.state.y, self.playfield.bounds().1),
        };

        *coordinate = match self.state.direction {
            Direction::Right | Direction::Down => (*coordinate + 1) % bound,
            Direction::Left | Direction::Up => coordinate.checked_sub(1).unwrap_or(bound - 1),
        };

        self
    }

    /// Returns a copy of the `Cursor` moved forward by one cell in a
    /// [`Direction`].
    pub fn go(mut self, direction: Direction) -> Self {
        self.state.direction = direction;
        self.step()
    }
}

impl From<Cursor<'_>> for Item {
    fn from(value: Cursor<'_>) -> Self {
        Self::Terminator(Terminator::Jump(Label::State(value.state)))
    }
}
