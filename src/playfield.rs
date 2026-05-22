use std::mem;

use crate::{errors::FungusError, value::Value};

/// A Befunge playfield.
pub struct Playfield {
    /// The width in cells.
    width: u16,

    /// The height in cells.
    height: u16,

    /// The [`Value`]s.
    values: Box<[Value]>,
}

impl Playfield {
    /// Creates a new `Playfield` from source code. This function returns a
    /// [`FungusError`] if the source code is too large to fit on a `Playfield`.
    pub fn new(source: &str) -> Result<Self, FungusError> {
        let width = source.lines().fold(1, |a, l| l.chars().count().max(a));
        let height = source.lines().count().max(1);

        let (Ok(width), Ok(height)) = (width.try_into(), height.try_into()) else {
            return Err(FungusError::SourceTooLarge);
        };

        let mut values =
            vec![Value::SPACE; usize::from(width) * usize::from(height)].into_boxed_slice();

        for (y, line) in source.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let index = y * usize::from(width) + x;
                values[index] = char.into();
            }
        }

        Ok(Self {
            width,
            height,
            values,
        })
    }

    /// Returns the `Playfield`'s bounds in cells.
    pub const fn bounds(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Returns a [`Value`] from the `Playfield` at a position in cells. This
    /// function returns [`None`] if the position is out of bounds.
    pub fn value(&self, x: u16, y: u16) -> Option<Value> {
        if x >= self.width {
            return None;
        }

        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        self.values.get(index).copied()
    }

    /// Puts a [`Value`] to the `Playfield` at a position in cells and returns
    /// the old [`Value`]. This function returns [`None`] if the position is out
    /// of bounds.
    pub fn put_value(&mut self, x: u16, y: u16, value: Value) -> Option<Value> {
        if x >= self.width {
            return None;
        }

        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        let cell = self.values.get_mut(index)?;
        Some(mem::replace(cell, value))
    }
}
