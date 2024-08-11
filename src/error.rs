use std::{error, fmt, io, result};

/// A result that may contain a Fungus error.
pub type Result<T> = result::Result<T, Error>;

/// An error raised by Fungus.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error caused by passing invalid command line arguments.
    InvalidArgs,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::InvalidArgs => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::InvalidArgs => write!(f, "usage: fungus <path>"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}