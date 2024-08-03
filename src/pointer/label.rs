use std::fmt;

use super::{Direction, Mode, Pointer};

/// The type used by a label for storing its data.
type Bits = u32;

/// The number of bits used for storing a label's direction and mode.
const FLAG_BIT_COUNT: u32 = 2 + 1;

/// The number of bits used for storing a label's X or Y position.
const POSITION_BIT_COUNT: u32 = (Bits::BITS - FLAG_BIT_COUNT) / 2;

/// A compacted representation of a pointer for labeling basic blocks.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    /// The data.
    data: Bits,
}

impl Label {
    /// The maximum width or height of a playfield pointed to by a label.
    pub const MAX_PLAYFIELD_LENGTH: usize = 2usize.pow(POSITION_BIT_COUNT);
}

impl From<Pointer> for Label {
    fn from(value: Pointer) -> Self {
        let direction = match value.direction {
            Direction::Right => 0b00,
            Direction::Down => 0b01,
            Direction::Left => 0b10,
            Direction::Up => 0b11,
        };

        let mode = match value.mode {
            Mode::Command => 0b000,
            Mode::String => 0b100,
        };

        let x = Bits::try_from(value.x).unwrap() << FLAG_BIT_COUNT;
        let y = Bits::try_from(value.y).unwrap() << (FLAG_BIT_COUNT + POSITION_BIT_COUNT);

        Self {
            data: y | x | mode | direction,
        }
    }
}

impl From<Label> for Pointer {
    fn from(value: Label) -> Self {
        const POSITION_MASK: usize = Label::MAX_PLAYFIELD_LENGTH - 1;

        let direction = match value.data & 0b11 {
            0b00 => Direction::Right,
            0b01 => Direction::Down,
            0b10 => Direction::Left,
            0b11 => Direction::Up,
            _ => unreachable!(),
        };

        let mode = match value.data & 0b100 {
            0b000 => Mode::Command,
            0b100 => Mode::String,
            _ => unreachable!(),
        };

        let x = usize::try_from(value.data >> FLAG_BIT_COUNT).unwrap() & POSITION_MASK;

        let y = usize::try_from(value.data >> (FLAG_BIT_COUNT + POSITION_BIT_COUNT)).unwrap()
            & POSITION_MASK;

        Self {
            x,
            y,
            direction,
            mode,
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pointer = Pointer::from(*self);

        let direction = match pointer.direction {
            Direction::Right => "right",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Up => "up",
        };

        let mode = match pointer.mode {
            Mode::Command => "command",
            Mode::String => "string",
        };

        write!(f, "x{}_y{}_{mode}_{direction}", pointer.x, pointer.y)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    /// Test that the default label value is zero.
    #[test]
    fn test_default() {
        assert_eq!(Label::default().data, 0, "default label data is not 0");
    }

    /// Test that labels are equivalent to their pointers.
    #[test]
    fn test_equivalance() {
        let pointers = new_test_pointers();

        for pointer in pointers {
            let label = Label::from(pointer.clone());
            let other_pointer = Pointer::from(label);
            let other_label = Label::from(other_pointer.clone());

            assert_eq!(
                label, other_label,
                "pointers do not convert to equivalent labels"
            );

            assert!(
                compare_pointers(&pointer, &other_pointer) == Ordering::Equal,
                "labels do not convert to equivalent pointers"
            );
        }
    }

    /// Test that labels are sorted in the expected display order.
    #[test]
    fn test_ordering() {
        let mut pointers = new_test_pointers();

        let mut labels: Vec<Label> = pointers
            .iter()
            .map(|pointer| Label::from(pointer.clone()))
            .collect();

        pointers.sort_by(|a, b| compare_pointers(a, b));
        labels.sort();

        for (pointer, label) in pointers.iter().zip(labels) {
            let other_label = Label::from(pointer.clone());
            assert_eq!(label, other_label, "labels are not sorted as expected");
        }
    }

    /// Create a new unsorted vector of test pointers.
    fn new_test_pointers() -> Vec<Pointer> {
        let positions = vec![1, 100, 1000, 31, 11, 0, 2, 3, 4, 8, 24];

        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let modes = vec![Mode::String, Mode::Command];
        let mut pointers = vec![];

        for &x in &positions {
            for &y in &positions {
                for &direction in &directions {
                    for &mode in &modes {
                        pointers.push(Pointer {
                            x,
                            y,
                            direction,
                            mode,
                        });
                    }
                }
            }
        }

        pointers
    }

    /// Compare two pointers by label display order. Lesser values appear first.
    fn compare_pointers(a: &Pointer, b: &Pointer) -> Ordering {
        /// Get a direction's label display order. Lower numbers appear first.
        fn direction_order(direction: Direction) -> u8 {
            match direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }
        }

        // The Y position has the highest priority in ordering...
        match a.y.cmp(&b.y) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }

        // ...followed by the X position...
        match a.x.cmp(&b.x) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }

        // ...followed by the mode...
        match (a.mode, b.mode) {
            (Mode::Command, Mode::Command) => (),
            (Mode::Command, Mode::String) => return Ordering::Less,
            (Mode::String, Mode::Command) => return Ordering::Greater,
            (Mode::String, Mode::String) => (),
        }

        // ...and finally, the direction.
        direction_order(a.direction).cmp(&direction_order(b.direction))
    }
}
