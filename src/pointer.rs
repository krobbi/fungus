use std::fmt;

use crate::playfield::Playfield;

/// An instruction pointer with a playfield position.
#[derive(Clone, Default)]
pub struct Pointer {
    /// The X position.
    x: usize,

    /// The Y position.
    y: usize,
}

impl Pointer {
    /// Advance the pointer by one character on a playfield.
    pub fn advance(&mut self, playfield: &Playfield) {
        self.x = if self.x == playfield.width() - 1 {
            0
        } else {
            self.x + 1
        };
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x{}_y{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the default pointer has the expected fields.
    #[test]
    fn test_default() {
        let pointer = Pointer::default();
        assert_eq!(pointer.x, 0, "default pointer x position is not 0");
        assert_eq!(pointer.y, 0, "default pointer y position is not 0");
    }

    /// Test that pointers advance and wrap to the expected positions.
    #[test]
    fn test_advance() {
        let mut tester = Tester::new("012\n345\n678");
        tester.advance(1, 0);
        tester.advance(2, 0);
        tester.advance(0, 0);
    }

    /// Test that pointers wrap on 1x1 playfields.
    #[test]
    fn test_empty_wrap() {
        let mut tester = Tester::new("");
        tester.advance(0, 0);
    }

    /// Tests pointer movement on a playfield.
    struct Tester {
        /// The playfield.
        playfield: Playfield,

        /// The pointer.
        pointer: Pointer,
    }

    impl Tester {
        /// Create a new tester from source code.
        fn new(source: &str) -> Self {
            Self {
                playfield: Playfield::new(source),
                pointer: Pointer::default(),
            }
        }

        /// Advance the pointer with an expected target position.
        fn advance(&mut self, x: usize, y: usize) {
            self.pointer.advance(&self.playfield);
            assert_eq!(self.pointer.x, x, "pointer x position is not {x}");
            assert_eq!(self.pointer.y, y, "pointer y position is not {y}");
        }
    }
}
