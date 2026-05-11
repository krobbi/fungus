use std::{io, process::ExitCode};

use thiserror::Error;

/// An error caught by Fungus.
#[derive(Debug, Error)]
pub enum FungusError {
    /// A [`clap::Error`] caught while creating a
    /// [`Config`][crate::config::Config] from command line arguments. May be a
    /// non-error help or version message.
    #[error(transparent)]
    Cli(#[from] clap::Error),
}

impl FungusError {
    /// Prints the `FungusError`, ignoring any [`io::Error`]s caused by
    /// printing.
    pub fn print(&self) {
        let _: io::Result<()> = match self {
            Self::Cli(error) => error.print(),
        };
    }

    /// Returns the [`ExitCode`] associated with the `FungusError`.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Cli(error) => error
                .exit_code()
                .try_into()
                .map_or(ExitCode::FAILURE, From::<u8>::from),
        }
    }
}
