use std::{cmp::Ordering, fmt};

use crate::playfield::Playfield;

/// An instruction pointer with a playfield position.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Pointer {
    /// The X position.
    x: usize,

    /// The Y position.
    y: usize,

    /// The direction.
    direction: Direction,
}

impl Pointer {
    /// Get the position.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Face the pointer in a direction.
    pub fn face(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Advance the pointer by one character on a playfield.
    pub fn advance(&mut self, playfield: &Playfield) {
        match self.direction {
            Direction::Right => {
                self.x = if self.x == playfield.width() - 1 {
                    0
                } else {
                    self.x + 1
                };
            }
            Direction::Down => {
                self.y = if self.y == playfield.height() - 1 {
                    0
                } else {
                    self.y + 1
                };
            }
            Direction::Left => {
                self.x = if self.x == 0 {
                    playfield.width() - 1
                } else {
                    self.x - 1
                };
            }
            Direction::Up => {
                self.y = if self.y == 0 {
                    playfield.height() - 1
                } else {
                    self.y - 1
                };
            }
        }
    }
}

impl PartialOrd for Pointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pointer {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        };

        match self.x.cmp(&other.x) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        };

        self.direction.cmp(&other.direction)
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x{}_y{}_{}", self.x, self.y, self.direction)
    }
}

/// A direction traveled by a pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    /// A direction where the X position is incremented.
    #[default]
    Right,

    /// A direction where the Y position is incremented.
    Down,

    /// A direction where the X position is decremented.
    Left,

    /// A direction where the Y position is decremented.
    Up,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Right => write!(f, "right"),
            Self::Down => write!(f, "down"),
            Self::Left => write!(f, "left"),
            Self::Up => write!(f, "up"),
        }
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

        assert_eq!(
            pointer.direction,
            Direction::Right,
            "default pointer direction is not right"
        );
    }

    /// Test that pointers advance and wrap to the expected positions.
    #[test]
    fn test_advance() {
        let mut tester = Tester::new("012\n345\n678");
        tester.advance(1, 0);
        tester.advance(2, 0);
        tester.advance(0, 0);
        tester.advance(1, 0);

        tester.face(Direction::Down);
        tester.advance(1, 1);
        tester.advance(1, 2);
        tester.advance(1, 0);
        tester.advance(1, 1);

        tester.face(Direction::Left);
        tester.advance(0, 1);
        tester.advance(2, 1);
        tester.advance(1, 1);

        tester.face(Direction::Up);
        tester.advance(1, 0);
        tester.advance(1, 2);
        tester.advance(1, 1);
    }

    /// Test that pointers wrap on 1x1 playfields.
    #[test]
    fn test_empty_wrap() {
        let mut tester = Tester::new("");
        tester.advance(0, 0);
        tester.face(Direction::Down);
        tester.advance(0, 0);
        tester.face(Direction::Left);
        tester.advance(0, 0);
        tester.face(Direction::Up);
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

        /// Face the pointer in a direction.
        fn face(&mut self, direction: Direction) {
            let (x, y) = self.pointer.position();
            self.pointer.face(direction);

            assert_eq!(
                self.pointer.direction, direction,
                "pointer direction is not {direction}"
            );

            assert_eq!(
                self.pointer.x, x,
                "pointer changed x position while turning"
            );

            assert_eq!(
                self.pointer.y, y,
                "pointer changed y position while turning"
            );
        }

        /// Advance the pointer with an expected target position.
        fn advance(&mut self, x: usize, y: usize) {
            let direction = self.pointer.direction;
            self.pointer.advance(&self.playfield);
            let (getter_x, getter_y) = self.pointer.position();

            assert_eq!(
                getter_x, self.pointer.x,
                "pointer x position getter does not match property"
            );

            assert_eq!(
                getter_y, self.pointer.y,
                "pointer y position getter does not match property"
            );

            assert_eq!(self.pointer.x, x, "pointer x position is not {x}");
            assert_eq!(self.pointer.y, y, "pointer y position is not {y}");

            assert_eq!(
                self.pointer.direction, direction,
                "pointer changed direction while advancing"
            );
        }
    }
}
