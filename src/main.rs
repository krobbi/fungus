use crate::config::Config;

mod config;

/// Runs Fungus.
fn main() {
    let config = Config::from_cli();
    let source_file_path = config.source_file_path();
    println!("{:?}", source_file_path.to_string_lossy());
}
