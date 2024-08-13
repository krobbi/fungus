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
}

/// Command line arguments.
#[derive(Parser)]
#[command(version, about = "Optimizing Befunge compiler")]
struct Args {
    /// The path to the source file.
    #[arg(help = "Source file path")]
    path: PathBuf,
}
