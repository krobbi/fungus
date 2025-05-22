use std::{
    error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
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

    /// An error caused by the source file not existing.
    SourceFileDoesNotExist,

    /// An error caused by an I/O error while reading the source file.
    CouldNotReadSourceFile(io::Error),
}

impl Error {
    /// Prints the error and returns an exit code.
    pub fn report(&self) -> ExitCode {
        if let Self::Clap(e) = self {
            let _ = e.print();
            u8::try_from(e.exit_code()).unwrap_or(1).into()
        } else {
            let _ = writeln!(io::stderr(), "error: {self}");
            ExitCode::FAILURE
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
            Self::SourceFileDoesNotExist => None,
            Self::CouldNotReadSourceFile(e) => Some(e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clap(e) => e.fmt(f),
            Self::SourceFileDoesNotExist => f.write_str("source file does not exist"),
            Self::CouldNotReadSourceFile(e) => write!(f, "could not read source file: {e}"),
        }
    }
}
