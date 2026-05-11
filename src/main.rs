mod config;
mod errors;

use std::{fs, path::Path, process::ExitCode};

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
    let source = read_source(config.source_file_path())?;
    println!("--- SOURCE ---\n{source}\n--------------");
    Ok(())
}

/// Reads source code from a [`Path`]. This function returns a [`FungusError`]
/// if the source file does not exist or could not be read.
fn read_source(path: &Path) -> Result<String, FungusError> {
    if !path.is_file() {
        return Err(FungusError::SourceFileMissing(Box::from(path)));
    }

    fs::read_to_string(path).map_err(|e| FungusError::SourceFileRead(Box::from(path), e))
}
