use std::path::Path;

use clap::{Parser, ValueHint};

/// A configuration for Fungus.
pub struct Config(Data);

impl Config {
    /// Creates a new `Config` from command line arguments.
    pub fn from_cli() -> Self {
        let data = Data::parse();
        Self(data)
    }

    /// Returns the [`Path`] to the source file.
    pub fn source_file_path(&self) -> &Path {
        &self.0.source_file_path
    }
}

/// A [`Config`]'s data.
#[derive(Parser)]
#[command(bin_name("fungus"), version, about)]
struct Data {
    /// The [`Path`] to the source file.
    #[arg(
        value_hint(ValueHint::FilePath),
        value_name("FILE"),
        help = "Source file path"
    )]
    source_file_path: Box<Path>,
}
