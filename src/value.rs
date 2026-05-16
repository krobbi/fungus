use std::{
    fmt::{self, Display, Formatter},
    ops::{Neg, Not},
};

/// A Befunge value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(pub i64);

impl Value {
    /// Returns [`true`] if the `Value` is not equal to `0`.
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self(value.into())
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        (!self.is_non_zero()).into()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
