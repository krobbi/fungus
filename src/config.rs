use std::path::{Path, PathBuf};

use clap::{Parser, ValueHint};

use crate::error::Error;

/// A configuration for Fungus.
pub struct Config(Inner);

impl Config {
    /// Creates a new `Config` from command line arguments.
    pub fn try_new() -> Result<Self, Error> {
        let inner = Inner::try_parse()?;
        Ok(Self(inner))
    }

    /// Returns the [`Path`] to the source file.
    pub fn source_path(&self) -> &Path {
        &self.0.source_path
    }

    /// Returns [`true`] if the program should be printed as pseudo-assembly.
    pub fn should_print_pseudo_assembly(&self) -> bool {
        self.0.should_print_pseudo_assembly
    }
}

/// Data for a [`Config`].
#[derive(Parser)]
#[command(bin_name("fungus"), version, about)]
struct Inner {
    /// The path to the source file.
    #[arg(
        value_hint(ValueHint::FilePath),
        value_name("SOURCE"),
        help = "Source file path"
    )]
    source_path: PathBuf,

    /// Whether to print the program as pseudo-assembly.
    #[arg(id = "dump", short, long, help = "Print pseudo-assembly")]
    should_print_pseudo_assembly: bool,
}
