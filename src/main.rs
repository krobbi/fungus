mod cfg;
mod config;
mod errors;
mod interpret;
mod optimize;
mod parse;
mod playfield;
mod state;
mod value;

use std::{fs, path::Path, process::ExitCode};

use crate::{config::Config, errors::FungusError, playfield::Playfield};

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
    let mut playfield = Playfield::new(&source)?;
    let mut cfg = parse::parse_playfield(&playfield);
    optimize::optimize_cfg(&mut cfg);

    if config.should_display_program() {
        println!("{cfg}");
        return Ok(());
    }

    interpret::interpret_cfg(&cfg, &mut playfield);
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
