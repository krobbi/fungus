use std::{error, fmt, io, process, result};

use crate::playfield::Playfield;

/// A result that may contain a Fungus error.
pub type Result<T> = result::Result<T, Error>;

/// An error raised by Fungus.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error raised by clap.
    Clap(clap::Error),

    /// An error caused by loading a playfield above the maximum size.
    PlayfieldTooLarge,

    /// An error caused by compiling potentially self-modifying code.
    SelfModifyingCode,
}

impl Error {
    /// Exit with the error.
    pub fn exit(&self) -> ! {
        if let Self::Clap(error) = self {
            error.exit();
        } else {
            eprintln!("{self}");
            process::exit(1);
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Clap(error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Clap(error) => error.fmt(f),
            Self::PlayfieldTooLarge => {
                write!(f, "playfield is larger than {0}x{0}", Playfield::MAX_LENGTH)
            }
            Self::SelfModifyingCode => write!(f, "program may contain self-modifying code"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<clap::Error> for Error {
    fn from(value: clap::Error) -> Self {
        Self::Clap(value)
    }
}
