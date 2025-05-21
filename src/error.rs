use std::{
    error,
    fmt::{self, Display, Formatter},
    process::ExitCode,
    result,
};

/// A result that may contain a Fungus error.
pub type Result<T> = result::Result<T, Error>;

/// An error raised by Fungus.
#[derive(Debug)]
pub enum Error {
    /// An error raised by clap.
    Clap(clap::Error),
}

impl Error {
    /// Prints the error and returns an exit code.
    pub fn report(&self) -> ExitCode {
        match self {
            Self::Clap(e) => {
                let _ = e.print();
                u8::try_from(e.exit_code()).unwrap_or(1).into()
            }
        }
    }
}

impl From<clap::Error> for Error {
    fn from(value: clap::Error) -> Self {
        Self::Clap(value)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Clap(e) => Some(e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clap(e) => e.fmt(f),
        }
    }
}
