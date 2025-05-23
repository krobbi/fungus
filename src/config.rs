use std::path::{Path, PathBuf};

use clap::Parser;

use crate::error::Result;

/// Configuration data for Fungus.
pub struct Config {
    /// The inner command line arguments.
    args: Args,
}

impl Config {
    /// Creates new configuration data from command line arguments.
    pub fn try_new() -> Result<Self> {
        let args = Args::try_parse()?;
        Ok(Self { args })
    }

    /// Returns the path to the source file.
    pub fn path(&self) -> &Path {
        &self.args.path
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(bin_name("fungus"), version, about)]
struct Args {
    /// The path to the source file.
    #[arg(help = "Source file path")]
    path: PathBuf,
}
