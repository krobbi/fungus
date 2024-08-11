mod error;
mod ir;
mod playfield;
mod pointer;

use std::{env, fs, process};

use error::{Error, Result};
use ir::Program;

/// Run Fungus.
fn main() {
    let mut program = load_program().unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    program.optimize();
    program.dump();
}

/// Load a program from command line arguments.
fn load_program() -> Result<Program> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        Ok(Program::new(&fs::read_to_string(&args[1])?))
    } else {
        Err(Error::InvalidArgs)
    }
}
