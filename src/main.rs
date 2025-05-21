mod config;
mod error;

use std::process::ExitCode;

use config::Config;
use error::Result;

/// Runs Fungus and returns an exit code.
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

/// Runs Fungus and returns a result.
fn try_run() -> Result<()> {
    let config = Config::new()?;
    println!("path: {}", config.path().display());
    Ok(())
}
