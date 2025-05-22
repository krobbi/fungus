/// A Befunge value.
#[derive(Clone, Copy, Default)]
pub struct Value {
    /// The inner value.
    value: i32,
}

impl Value {
    /// Lossily converts the value to a printable ASCII character, replacing
    /// non-printable values with `'`.
    pub fn into_printable_ascii_char_lossy(self) -> char {
        match self.value {
            v @ 0x20..=0x7e => u8::try_from(v)
                .expect("the range of `u8` should contain `v`")
                .into(),
            _ => '\'',
        }
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        let value = i32::try_from(u32::from(value))
            .expect("the range of `i32` should contain the range of `char`");
        Self { value }
    }
}
