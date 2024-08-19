mod config;
mod error;
mod ir;
mod playfield;
mod pointer;

use std::fs;

use config::Config;
use error::Result;
use ir::Program;

/// Run Fungus or exit with an error.
fn main() {
    run_fungus().unwrap_or_else(|error| error.exit());
}

/// Run Fungus and get a result.
fn run_fungus() -> Result<()> {
    let config = Config::new()?;
    let program = Program::new(&fs::read_to_string(config.path())?)?;

    if config.dump() {
        program.dump();
    } else {
        program.interpret();
    }

    Ok(())
}
