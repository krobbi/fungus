use std::{
    io::{self, Write as _},
    path::Path,
    process::ExitCode,
};

use thiserror::Error;

/// An error caught by Fungus.
#[derive(Debug, Error)]
pub enum FungusError {
    /// A [`clap::Error`] caught while creating a
    /// [`Config`][crate::config::Config] from command line arguments. May be a
    /// non-error help or version message.
    #[error("{0}")]
    Cli(#[from] clap::Error),

    /// The source file does not exist.
    #[error("source file '{0}' does not exist")]
    SourceFileMissing(Box<Path>),

    /// The source file could not be read.
    #[error("could not read source file '{0}': {1}")]
    SourceFileRead(Box<Path>, #[source] io::Error),

    /// The source code is too large to fit on a
    /// [`Playfield`][crate::playfield::Playfield].
    #[error("source code is larger than 65,535x65,535 characters")]
    SourceTooLarge,
}

impl FungusError {
    /// Prints the `FungusError`, ignoring any [`io::Error`]s caused by
    /// printing.
    pub fn print(&self) {
        let _: io::Result<()> = match self {
            Self::Cli(error) => error.print(),
            _ => writeln!(io::stderr(), "error: {self}"),
        };
    }

    /// Returns the [`ExitCode`] associated with the `FungusError`.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Cli(error) => error
                .exit_code()
                .try_into()
                .map_or(ExitCode::FAILURE, From::<u8>::from),
            _ => ExitCode::FAILURE,
        }
    }
}
