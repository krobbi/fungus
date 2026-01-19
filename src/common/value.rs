use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Div, Mul, Not, Rem, Sub},
};

/// A Befunge value.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(pub i64);

impl Value {
    /// Returns [`true`] if the `Value` is not equal to `0`.
    pub fn is_non_zero(self) -> bool {
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

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self(value)
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

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        (!self.is_non_zero()).into()
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.0.wrapping_add(rhs.0).into()
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.wrapping_sub(rhs.0).into()
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0.wrapping_mul(rhs.0).into()
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let inner = if rhs.is_non_zero() {
            self.0.wrapping_div(rhs.0)
        } else {
            // Division by zero is defined to return zero in Funge-98. In
            // Befunge-93 it is undefined behavior.
            0
        };

        inner.into()
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let inner = if rhs.is_non_zero() {
            self.0.wrapping_rem(rhs.0)
        } else {
            // Modulo by zero is defined to return zero in Funge-98. In
            // Befunge-93 it is undefined behavior.
            0
        };

        inner.into()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
