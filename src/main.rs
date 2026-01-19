mod config;
mod fungus_error;
mod interpret;
mod ir;
mod optimize;
mod parse;
mod playfield;
mod value;

use std::{fs, path::Path, process::ExitCode};

use crate::{config::Config, fungus_error::FungusError, playfield::Playfield};

/// Runs Fungus and returns an [`ExitCode`].
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            error.exit_code()
        }
    }
}

/// Runs Fungus. This function returns a [`FungusError`] if an error occurred.
fn try_run() -> Result<(), FungusError> {
    let config = Config::try_new()?;
    let source = try_read_source(config.source_path())?;
    let mut playfield = Playfield::new(&source);
    let (mut program, flow_graph) = parse::parse_program(&playfield);
    optimize::optimize_program(&mut program, &flow_graph, &playfield);

    if config.should_print_pseudo_assembly() {
        println!("{program}");
    } else {
        interpret::interpret_program(&program, &mut playfield);
    }

    Ok(())
}

/// Reads source code from a file path. This function returns a [`FungusError`]
/// if the source file does not exist or could not be read.
fn try_read_source(path: &Path) -> Result<String, FungusError> {
    if path.is_file() {
        fs::read_to_string(path).map_err(|e| FungusError::SourceFileRead(path.into(), e))
    } else {
        Err(FungusError::SourceFileMissing(path.into()))
    }
}
