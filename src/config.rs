use std::path::Path;

use clap::{Parser, ValueHint};

use crate::errors::FungusError;

/// A configuration for Fungus.
pub struct Config(Data);

impl Config {
    /// Creates a new `Config` from command line arguments. This function
    /// returns a [`FungusError`] if the command line arguments were invalid or
    /// if a help or version message should be displayed.
    pub fn from_cli() -> Result<Self, FungusError> {
        let data = Data::try_parse()?;
        Ok(Self(data))
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
