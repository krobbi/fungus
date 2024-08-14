use std::path::{Path, PathBuf};

use clap::Parser;

use crate::error::Result;

/// Configuration data for Fungus.
pub struct Config {
    /// The command line arguments.
    args: Args,
}

impl Config {
    /// Create new configuration data from command line arguments.
    pub fn new() -> Result<Self> {
        Ok(Self {
            args: Args::try_parse()?,
        })
    }

    /// Get the path to the source file.
    pub fn path(&self) -> &Path {
        &self.args.path
    }

    /// Get whether to print the program as pseudo-assembly.
    pub fn dump(&self) -> bool {
        self.args.dump
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(version, about = "Optimizing Befunge interpreter")]
struct Args {
    /// The path to the source file.
    #[arg(help = "Source file path")]
    path: PathBuf,

    /// Whether to print the program as pseudo-assembly.
    #[arg(short, long, help = "Print pseudo-assembly")]
    dump: bool,
}
