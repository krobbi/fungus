use std::ops;

use crate::pointer::{Label, Pointer};

/// A 2D grid of characters.
pub struct Playfield {
    /// The width in characters.
    width: usize,

    /// The height in characters.
    height: usize,

    /// The characters.
    cells: Vec<char>,
}

impl Playfield {
    /// Create a new playfield from source code.
    pub fn new(source: &str) -> Self {
        let source: Vec<Vec<char>> = source.lines().map(|line| line.chars().collect()).collect();
        let mut width = 1;

        for line in &source {
            width = line.len().max(width);
        }

        let height = source.len().max(1);

        assert!(
            width <= Label::MAX_PLAYFIELD_LENGTH && height <= Label::MAX_PLAYFIELD_LENGTH,
            "playfield too large"
        );

        let mut cells = vec!['\0'; width * height];

        for (y, line) in source.iter().enumerate() {
            for (x, &character) in line.iter().enumerate() {
                cells[y * width + x] = character;
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Get the width in cells.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height in cells.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl ops::Index<&Pointer> for Playfield {
    type Output = char;

    fn index(&self, index: &Pointer) -> &Self::Output {
        let (x, y) = index.position();

        assert!(
            x < self.width && y < self.height,
            "playfield index out of bounds"
        );

        &self.cells[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use crate::pointer::Direction;

    use super::*;

    /// Test that empty playfields contain a single null character.
    #[test]
    fn test_empty() {
        let playfield = new_playfield("", 1, 1);

        assert_eq!(
            playfield.cells[0], '\0',
            "empty playfield is not a null character"
        );
    }

    /// Test that playfields are stored in row-major order.
    #[test]
    fn test_order() {
        let playfield = new_playfield("012\n345", 3, 2);
        assert_eq!(playfield.cells[0], '0');
        assert_eq!(playfield.cells[1], '1');
        assert_eq!(playfield.cells[2], '2');
        assert_eq!(playfield.cells[3], '3');
        assert_eq!(playfield.cells[4], '4');
        assert_eq!(playfield.cells[5], '5');
    }

    /// Test that staggered playfields are padded with null characters.
    #[test]
    fn test_padding() {
        let playfield = new_playfield("012\n3\n67", 3, 3);
        assert_eq!(playfield.cells[0], '0');
        assert_eq!(playfield.cells[1], '1');
        assert_eq!(playfield.cells[2], '2');
        assert_eq!(playfield.cells[3], '3');
        assert_eq!(playfield.cells[4], '\0');
        assert_eq!(playfield.cells[5], '\0');
        assert_eq!(playfield.cells[6], '6');
        assert_eq!(playfield.cells[7], '7');
        assert_eq!(playfield.cells[8], '\0');
    }

    /// Test that trailing empty lines are not included in playfields.
    #[test]
    fn test_trailing_lines() {
        /// Check that source code creates a playfield with an expected size.
        fn check(source: &str, width: usize, height: usize) {
            new_playfield(source, width, height);
        }

        check("none", 4, 1);
        check("lf\n", 2, 1);
        check("crlf\r\n", 4, 1);
        check("space before \n", 13, 1);
        check("space after\n ", 11, 2);
        check("mixed\ncha\racters\r\n", 10, 2);
        check("double\nlf\n\n", 6, 3);
        check("double\r\ncrlf\r\n\r\n", 6, 3);
        check("double\nspaced \n \n", 7, 3);
    }

    /// Test that a playfield can be indexed with wrapping.
    #[test]
    fn test_index() {
        /// Check that a pointer indexes to a given character after advancing.
        fn check(playfield: &Playfield, pointer: &mut Pointer, character: char) {
            pointer.advance(playfield);

            assert_eq!(
                playfield[pointer], character,
                "playfield character is not {character}"
            );
        }

        let playfield = new_playfield("012\n345\n678", 3, 3);
        let mut pointer = Pointer::from(Label::default());

        pointer.face(Direction::Left);
        check(&playfield, &mut pointer, '2');
        check(&playfield, &mut pointer, '1');

        pointer.face(Direction::Up);
        check(&playfield, &mut pointer, '7');
        check(&playfield, &mut pointer, '4');

        pointer.face(Direction::Right);
        check(&playfield, &mut pointer, '5');
        check(&playfield, &mut pointer, '3');

        pointer.face(Direction::Down);
        check(&playfield, &mut pointer, '6');
        check(&playfield, &mut pointer, '0');
    }

    /// Test that indexing a playfield out of bounds causes a panic.
    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let playfield = new_playfield("01", 2, 1);
        let mut pointer = Pointer::from(Label::default());
        pointer.advance(&playfield);

        // The playfield needs to be swapped out to work around the wrapping.
        let playfield = new_playfield("0\n1", 1, 2);
        playfield[&pointer];
    }

    /// Create a new playfield from source code with an expected size.
    fn new_playfield(source: &str, width: usize, height: usize) -> Playfield {
        let playfield = Playfield::new(source);

        assert_eq!(
            playfield.width(),
            playfield.width,
            "playfield width getter does not match property"
        );

        assert_eq!(
            playfield.height(),
            playfield.height,
            "playfield height getter does not match property"
        );

        assert_eq!(playfield.width, width, "playfield width is not {width}");
        assert_eq!(playfield.height, height, "playfield height is not {height}");

        assert_eq!(
            playfield.cells.len(),
            width * height,
            "playfield size is not {width}x{height}"
        );

        playfield
    }
}
