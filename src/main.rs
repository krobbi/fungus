mod config;
mod errors;

use std::process::ExitCode;

use crate::{config::Config, errors::FungusError};

/// Runs Fungus and returns an [`ExitCode`].
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            error.exit_code()
        }
    }
}

/// Runs Fungus. This function returns a [`FungusError`] if an error occurred.
fn run() -> Result<(), FungusError> {
    let config = Config::from_cli()?;
    println!("{:?}", config.source_file_path().to_string_lossy());
    Ok(())
}
