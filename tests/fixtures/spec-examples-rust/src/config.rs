//! The configuration of the interpreter, read from `interp.toml`.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::Result;

/// The settings of a run.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The script to run; the REPL starts without one.
    pub script: Option<PathBuf>,
    /// The port of the debug server.
    pub port: Option<u16>,
    /// The max depth of the call stack.
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    /// The number of values kept by the cache.
    #[serde(default = "default_cache_size")]
    pub cache_size: usize,
    /// Print the value of every script.
    #[serde(default)]
    pub trace: bool,
}

fn default_max_depth() -> usize {
    256
}

fn default_cache_size() -> usize {
    1024
}

/// Reads the configuration file.
///
/// The file must exist and be valid TOML; missing optional fields get their defaults.
/// A missing `port` stops the interpreter.
pub fn load(path: &Path) -> Result<Config> {
    let raw = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&raw).unwrap();
    let port = cfg.port.expect("port is required");
    Ok(cfg)
}
