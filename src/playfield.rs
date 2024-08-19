use std::ops;

use crate::{
    error::{Error, Result},
    pointer::{Label, Pointer},
};

/// A value stored in a playfield or on the stack.
pub type Value = i32;

/// A 2D grid of values.
pub struct Playfield {
    /// The width in value cells.
    width: usize,

    /// The height in value cells.
    height: usize,

    /// The values.
    values: Vec<Value>,
}

impl Playfield {
    /// A playfield's maximum width or height in value cells.
    pub const MAX_LENGTH: usize = Label::MAX_POSITION + 1;

    /// Create a new playfield from source code.
    pub fn new(source: &str) -> Result<Self> {
        let source: Vec<Vec<char>> = source.lines().map(|line| line.chars().collect()).collect();
        let mut width = 1;

        for line in &source {
            width = line.len().max(width);
        }

        let height = source.len().max(1);

        if width > Self::MAX_LENGTH || height > Self::MAX_LENGTH {
            return Err(Error::PlayfieldTooLarge);
        }

        let mut values = vec![0; width * height];

        for (y, line) in source.iter().enumerate() {
            for (x, &value) in line.iter().enumerate() {
                values[y * width + x] = Self::char_to_value(value);
            }
        }

        Ok(Self {
            width,
            height,
            values,
        })
    }

    /// Convert a character to a value.
    pub const fn char_to_value(value: char) -> Value {
        value as Value
    }

    /// Convert a value to a character.
    pub fn value_to_char(value: Value) -> char {
        #[allow(clippy::cast_sign_loss)]
        char::from_u32(value as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
    }

    /// Get the width in value cells.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height in value cells.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get a value from its position.
    pub fn value(&self, x: Value, y: Value) -> Value {
        let x = usize::try_from(x).unwrap_or(usize::MAX);
        let y = usize::try_from(y).unwrap_or(usize::MAX);

        if x < self.width && y < self.height {
            self.values[y * self.width + x]
        } else {
            0
        }
    }
}

impl ops::Index<&Pointer> for Playfield {
    type Output = Value;

    fn index(&self, index: &Pointer) -> &Self::Output {
        let (x, y) = index.position();

        assert!(
            x < self.width && y < self.height,
            "playfield index out of bounds"
        );

        &self.values[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use crate::pointer::Direction;

    use super::*;

    /// Test that values are signed integers with enough bits.
    #[test]
    fn test_value() {
        assert_eq!(Value::default(), 0, "values are not integers");

        assert_ne!(
            Value::default().wrapping_sub(1),
            Value::MAX,
            "values are unsigned"
        );

        assert!(
            Value::BITS > 21,
            "values are not large enough to store a character plus a sign bit"
        );

        assert!(
            Value::BITS > Label::POSITION_BITS,
            "values are not large enough to store a position plus a sign bit"
        );
    }

    /// Test that empty playfields contain a single null character.
    #[test]
    fn test_empty() {
        let playfield = new_playfield("", 1, 1);
        check_index(&playfield, 0, '\0');
    }

    /// Test that playfields are stored in row-major order.
    #[test]
    fn test_order() {
        let playfield = new_playfield("012\n345", 3, 2);
        check_index(&playfield, 0, '0');
        check_index(&playfield, 1, '1');
        check_index(&playfield, 2, '2');
        check_index(&playfield, 3, '3');
        check_index(&playfield, 4, '4');
        check_index(&playfield, 5, '5');
    }

    /// Test that staggered playfields are padded with null characters.
    #[test]
    fn test_padding() {
        let playfield = new_playfield("012\n3\n67", 3, 3);
        check_index(&playfield, 0, '0');
        check_index(&playfield, 1, '1');
        check_index(&playfield, 2, '2');
        check_index(&playfield, 3, '3');
        check_index(&playfield, 4, '\0');
        check_index(&playfield, 5, '\0');
        check_index(&playfield, 6, '6');
        check_index(&playfield, 7, '7');
        check_index(&playfield, 8, '\0');
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
        fn check(playfield: &Playfield, pointer: &mut Pointer, value: char) {
            pointer.advance(playfield);

            assert_eq!(
                playfield[pointer],
                Playfield::char_to_value(value),
                "playfield character is not '{}'",
                value.escape_default()
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
        let playfield = Playfield::new(source).unwrap();

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
            playfield.values.len(),
            width * height,
            "playfield size is not {width}x{height}"
        );

        playfield
    }

    /// Check that a playfield's indexed value matches a given character.
    fn check_index(playfield: &Playfield, index: usize, value: char) {
        assert_eq!(
            playfield.values[index],
            Playfield::char_to_value(value),
            "indexed character is not '{}'",
            value.escape_default()
        );
    }
}
