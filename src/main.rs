mod common;
mod config;
mod error;

use std::{fs, path::Path, process::ExitCode};

use common::Playfield;
use config::Config;
use error::{Error, Result};

/// Runs Fungus and returns an exit code.
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

/// Runs Fungus.
fn try_run() -> Result<()> {
    let config = Config::try_new()?;
    let source = try_read_source(config.path())?;
    let playfield = Playfield::new(&source);
    println!("{playfield}");
    common::program_counter::temp_test_ordering();
    Ok(())
}

/// Reads source code from the source file's path.
fn try_read_source(path: &Path) -> Result<String> {
    if path.is_file() {
        fs::read_to_string(path).map_err(Error::CouldNotReadSourceFile)
    } else {
        Err(Error::SourceFileDoesNotExist)
    }
}
