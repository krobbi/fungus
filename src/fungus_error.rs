use std::{
    io::{self, Write as _},
    path::PathBuf,
    process::ExitCode,
};

use thiserror::Error;

/// An error caught by Fungus.
#[derive(Debug, Error)]
pub enum FungusError {
    /// A [`clap::Error`] caught while creating a
    /// [`Config`][crate::config::Config], including help or version messages.
    #[error(transparent)]
    Clap(#[from] clap::Error),

    /// An error caused by the source file not existing.
    #[error("source file '{0}' does not exist")]
    SourceFileMissing(PathBuf),

    /// An [`io::Error`] caught while reading the source file.
    #[error("could not read source file '{0}': {1}")]
    SourceFileRead(PathBuf, #[source] io::Error),

    /// An error caused by the source code being too large to fit on a
    /// [`Playfield`][crate::playfield::Playfield].
    #[error("source code is larger than 65,535x65,535 characters")]
    SourceTooLarge,
}

impl FungusError {
    /// Prints the `FungusError`, ignoring any [`io::Error`]s caused by
    /// printing.
    pub fn print(&self) {
        let _ = match self {
            Self::Clap(error) => error.print(),
            error => writeln!(io::stderr(), "error: {error}"),
        };
    }

    /// Returns the [`ExitCode`] associated with the `FungusError`.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Clap(error) => error
                .exit_code()
                .try_into()
                .map(From::<u8>::from)
                .unwrap_or(ExitCode::FAILURE),
            _ => ExitCode::FAILURE,
        }
    }
}
