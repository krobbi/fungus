/// A Befunge value.
#[derive(Clone, Copy)]
pub struct Value {
    /// The inner value.
    value: i32,
}

impl Value {
    /// Converts the value to an `i32`.
    pub fn into_i32(self) -> i32 {
        self.value
    }

    /// Lossily converts the value to a printable ASCII character, replacing
    /// non-printable values with a space.
    pub fn into_printable_ascii_char_lossy(self) -> char {
        match self.value {
            v @ 0x20..=0x7e => u8::try_from(v)
                .expect("the range of `u8` should contain `v`")
                .into(),
            _ => ' ',
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        let value = i32::try_from(u32::from(value))
            .expect("a `char` only has 21 significant bits, so an `i32` should always contain it");
        Self { value }
    }
}
