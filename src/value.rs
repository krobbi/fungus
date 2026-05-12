/// A Befunge value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(pub i64);

impl Value {
    /// Lossily converts the `Value` to a [`char`]. This function returns
    /// [`char::REPLACEMENT_CHARACTER`] if the `Value` is not a Unicode scalar
    /// value.
    pub fn to_char_lossy(self) -> char {
        self.0
            .try_into()
            .ok()
            .and_then(char::from_u32)
            .unwrap_or(char::REPLACEMENT_CHARACTER)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self(u32::from(value).into())
    }
}
