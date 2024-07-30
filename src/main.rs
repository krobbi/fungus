mod ir;
mod playfield;
mod pointer;

use std::{env, fs, process};

use ir::Program;

/// Run Fungus.
fn main() {
    let program = load_program().unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    program.dump();
}

/// Load a program from command line arguments.
fn load_program() -> Result<Program, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(source) => Ok(Program::new(&source)),
            Err(e) => Err(format!("{e}")),
        }
    } else {
        Err(String::from("Usage: fungus [path]"))
    }
}
