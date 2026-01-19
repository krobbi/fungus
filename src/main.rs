mod common;
mod config;
mod error;
mod interpret;
mod ir;
mod optimize;
mod parse;

use std::{fs, path::Path, process::ExitCode};

use crate::{common::Playfield, config::Config, error::Error};

/// Runs Fungus and returns an exit code.
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

/// Runs Fungus.
fn try_run() -> Result<(), Error> {
    let config = Config::try_new()?;
    let mut playfield = try_load_playfield(config.source_path())?;
    let (mut program, flow_graph) = parse::parse_program(&playfield);
    optimize::optimize_program(&mut program, &flow_graph, &playfield);

    if config.should_print_pseudo_assembly() {
        println!("{program}");
    } else {
        interpret::interpret_program(&program, &mut playfield);
    }

    Ok(())
}

/// Loads a playfield from a file path.
fn try_load_playfield(path: &Path) -> Result<Playfield, Error> {
    let source = try_read_source(path)?;
    Ok(Playfield::new(&source))
}

/// Reads source code from a file path.
fn try_read_source(path: &Path) -> Result<String, Error> {
    if path.is_file() {
        fs::read_to_string(path).map_err(Error::CouldNotReadSourceFile)
    } else {
        Err(Error::SourceFileDoesNotExist)
    }
}
