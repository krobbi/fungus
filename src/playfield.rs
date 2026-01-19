use std::mem;

use crate::{fungus_error::FungusError, value::Value};

/// A Befunge playfield.
pub struct Playfield {
    /// The width in cells.
    width: u16,

    /// The height in cells.
    height: u16,

    /// The values.
    values: Box<[Value]>,
}

impl Playfield {
    /// Creates a new `Playfield` from source code. This function returns a
    /// [`FungusError`] if the source code is too large to fit on a `Playfield`.
    pub fn try_new(source: &str) -> Result<Self, FungusError> {
        let width = source.lines().fold(1, |a, l| l.chars().count().max(a));
        let height = source.lines().count().max(1);

        let (Ok(width), Ok(height)) = (width.try_into(), height.try_into()) else {
            return Err(FungusError::SourceTooLarge);
        };

        let mut values =
            vec![Value::default(); usize::from(width) * usize::from(height)].into_boxed_slice();

        for (y, line) in source.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                values[x + y * usize::from(width)] = char.into();
            }
        }

        Ok(Self {
            width,
            height,
            values,
        })
    }

    /// Returns the bounds in cells.
    pub fn bounds(&self) -> (usize, usize) {
        (self.width.into(), self.height.into())
    }

    /// Returns the [`Value`] at a position in cells. This function returns
    /// [`None`] if the position is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<Value> {
        if x < self.width.into() {
            self.values.get(x + y * usize::from(self.width)).copied()
        } else {
            None
        }
    }

    /// Puts a [`Value`] at a position in cells and returns the previous
    /// [`Value`]. This function returns [`None`] if the position is out of
    /// bounds.
    pub fn put(&mut self, x: usize, y: usize, value: Value) -> Option<Value> {
        if x < self.width.into() {
            let cell = self.values.get_mut(x + y * usize::from(self.width))?;
            Some(mem::replace(cell, value))
        } else {
            None
        }
    }
}
