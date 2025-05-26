use std::fmt::{self, Display, Formatter};

use super::Value;

/// A Befunge playfield.
pub struct Playfield {
    /// The width in cells.
    width: usize,

    /// The height in cells.
    height: usize,

    /// The cells.
    cells: Vec<Value>,
}

impl Playfield {
    /// Creates a new playfield from source code.
    pub fn new(source: &str) -> Self {
        let mut lines = source.lines();

        let width = lines.clone().fold(1, |a, l| l.chars().count().max(a));
        let height = lines.clone().count().max(1);
        assert!(width > 0 && height > 0);

        let mut cells = vec![' '.into(); width * height];
        for row in cells.chunks_exact_mut(width) {
            if let Some(line) = lines.next() {
                let line: Box<[Value]> = line.chars().map(Into::into).collect();
                row[..line.len()].copy_from_slice(&line);
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns the bounds in cells.
    pub fn bounds(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns the value at a position in cells. Returns `None` if the position
    /// is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<Value> {
        if x < self.width {
            self.cells.get(x + y * self.width).copied()
        } else {
            None
        }
    }
}

impl Display for Playfield {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data = String::with_capacity((self.width + 1) * self.height);
        for row in self.cells.chunks_exact(self.width) {
            for &value in row {
                data.push(value.into_printable_ascii_char_lossy());
            }
            data.push('\n');
        }

        f.write_str(data.trim_end())
    }
}
