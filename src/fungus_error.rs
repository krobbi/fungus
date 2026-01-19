use std::{
    io::{self, Write as _},
    process::ExitCode,
};

use thiserror::Error;

/// An error raised by Fungus.
#[derive(Debug, Error)]
pub enum FungusError {
    /// An error raised by clap.
    #[error(transparent)]
    Clap(#[from] clap::Error),

    /// An error caused by the source file not existing.
    #[error("source file does not exist")]
    SourceFileDoesNotExist,

    /// An error caused by an I/O error while reading the source file.
    #[error("could not read source file: {0}")]
    CouldNotReadSourceFile(#[source] io::Error),
}

impl FungusError {
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
