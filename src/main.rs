mod common;
mod config;
mod error;
mod interpret;
mod ir;
mod optimize;
mod parse;

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
    let mut playfield = try_load_playfield(config.path())?;
    let mut program = parse::parse_program(&playfield);
    optimize::optimize_program(&mut program);

    if config.dump() {
        println!("{program}");
    } else {
        interpret::interpret_program(&program, &mut playfield);
    }

    Ok(())
}

/// Loads a playfield from a file path.
fn try_load_playfield(path: &Path) -> Result<Playfield> {
    let source = try_read_source(path)?;
    Ok(Playfield::new(&source))
}

/// Reads source code from a file path.
fn try_read_source(path: &Path) -> Result<String> {
    if path.is_file() {
        fs::read_to_string(path).map_err(Error::CouldNotReadSourceFile)
    } else {
        Err(Error::SourceFileDoesNotExist)
    }
}
