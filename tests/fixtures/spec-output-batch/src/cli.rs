//! The command line: `app [--config <path>] [--check]`.

use std::path::PathBuf;

use crate::config::{Settings, load_config};

/// Checks the config file of `--check` and prints the settings.
pub fn check(args: &[String]) -> anyhow::Result<Settings> {
    let path = args
        .iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1))
        .map_or_else(|| PathBuf::from("config.toml"), PathBuf::from);
    let settings = load_config(&path)?;
    println!("{settings:?}");
    Ok(settings)
}
