mod label;

pub use label::Label;

use crate::playfield::Playfield;

/// An instruction pointer with a playfield position.
#[derive(Clone)]
pub struct Pointer {
    /// The X position.
    x: usize,

    /// The Y position.
    y: usize,

    /// The direction.
    direction: Direction,

    /// The mode.
    mode: Mode,
}

impl Pointer {
    /// Get the position.
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Get the mode.
    pub fn mode(&self) -> Mode {
        self.mode
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

    /// Toggle the mode.
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Command => Mode::String,
            Mode::String => Mode::Command,
        };
    }

    /// Get a new label branched from the pointer in a direction on a playfield.
    pub fn branch_label(&self, direction: Direction, playfield: &Playfield) -> Label {
        let mut pointer = self.clone();
        pointer.face(direction);
        pointer.advance(playfield);
        Label::from(pointer)
    }
}

/// A direction traveled by a pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// A direction where the X position is incremented.
    Right,

    /// A direction where the Y position is incremented.
    Down,

    /// A direction where the X position is decremented.
    Left,

    /// A direction where the Y position is decremented.
    Up,
}

/// A mode used by a pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// A mode where the pointer executes characters as commands.
    Command,

    /// A mode where the pointer pushes characters to the stack as values.
    String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the default pointer has the expected fields.
    #[test]
    fn test_default() {
        let pointer = Pointer::from(Label::default());
        assert_eq!(pointer.x, 0, "default pointer x position is not 0");
        assert_eq!(pointer.y, 0, "default pointer y position is not 0");

        assert_eq!(
            pointer.direction,
            Direction::Right,
            "default pointer direction is not right"
        );

        assert_eq!(
            pointer.mode,
            Mode::Command,
            "default pointer mode is not command"
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

    /// Test that pointers toggle to the expected modes.
    #[test]
    fn test_toggle_mode() {
        /// Check that a pointer toggled to the expected mode.
        fn check(pointer: &mut Pointer, mode: Mode) {
            let (x, y) = pointer.position();
            let direction = pointer.direction;
            pointer.toggle_mode();

            assert_eq!(
                pointer.mode(),
                pointer.mode,
                "pointer mode getter does not match property"
            );

            assert_eq!(pointer.mode, mode, "pointer mode is not {mode:?}");

            assert_eq!(
                pointer.x, x,
                "pointer changed x position while toggling mode"
            );

            assert_eq!(
                pointer.y, y,
                "pointer changed y position while toggling mode"
            );

            assert_eq!(
                pointer.direction, direction,
                "pointer changed direction while toggling mode"
            );
        }

        let mut pointer = Pointer::from(Label::default());
        check(&mut pointer, Mode::String);
        check(&mut pointer, Mode::Command);
        check(&mut pointer, Mode::String);
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
                playfield: Playfield::new(source).unwrap(),
                pointer: Pointer::from(Label::default()),
            }
        }

        /// Face the pointer in a direction.
        fn face(&mut self, direction: Direction) {
            let (x, y) = self.pointer.position();
            let mode = self.pointer.mode;
            self.pointer.face(direction);

            assert_eq!(
                self.pointer.direction, direction,
                "pointer direction is not {direction:?}"
            );

            assert_eq!(
                self.pointer.x, x,
                "pointer changed x position while turning"
            );

            assert_eq!(
                self.pointer.y, y,
                "pointer changed y position while turning"
            );

            assert_eq!(
                self.pointer.mode, mode,
                "pointer changed mode while turning"
            );
        }

        /// Advance the pointer with an expected target position.
        fn advance(&mut self, x: usize, y: usize) {
            let direction = self.pointer.direction;
            let mode = self.pointer.mode;
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

            assert_eq!(
                self.pointer.mode, mode,
                "pointer changed mode while advancing"
            );
        }
    }
}
