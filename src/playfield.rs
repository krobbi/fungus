/// A 2D grid of characters.
pub struct Playfield {
    /// The width in characters.
    width: usize,

    /// The height in characters.
    height: usize,

    /// The characters.
    // TODO: Remove this attribute after playfield characters are used.
    #[allow(dead_code)]
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

        let width = width;
        let height = source.len().max(1);
        let mut cells = vec!['\0'; width * height];

        for (y, line) in source.iter().enumerate() {
            for (x, &character) in line.iter().enumerate() {
                cells[y * width + x] = character;
            }
        }

        let cells = cells;

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

#[cfg(test)]
mod tests {
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
        check("double\nspaced \n \n", 7, 3);
        check("double\r\ncrlf\r\n\r\n", 6, 3);
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
