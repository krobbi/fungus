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
    pub const fn source_file_path(&self) -> &Path {
        &self.0.source_file_path
    }

    /// Returns [`true`] if the program should be displayed instead of being
    /// interpreted.
    pub const fn should_display_program(&self) -> bool {
        self.0.should_display_program
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

    /// Whether the program should be displayed instead of being interpreted.
    #[arg(id = "dump", help = "Print pseudo-assembly", short, long)]
    should_display_program: bool,
}
