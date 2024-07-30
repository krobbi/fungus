mod ir;
mod playfield;
mod pointer;

use std::{env, fs, process};

use ir::Block;
use playfield::Playfield;
use pointer::Pointer;

/// Run Fungus.
fn main() {
    let playfield = load_playfield().unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let pointer = Pointer::default();
    let block = Block::new(&playfield, &pointer);
    println!("{block}");
}

/// Load a playfield from command line arguments.
fn load_playfield() -> Result<Playfield, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match fs::read_to_string(&args[1]) {
            Ok(source) => Ok(Playfield::new(&source)),
            Err(e) => Err(format!("{e}")),
        }
    } else {
        Err(String::from("Usage: fungus [path]"))
    }
}
