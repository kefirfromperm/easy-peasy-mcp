//! The configuration file of the server.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::Error;

/// The settings read from `app.toml`.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The address to listen on.
    pub host: String,
    /// The port to listen on; required.
    pub port: Option<u16>,
    /// The directory with the data files.
    pub data_dir: String,
    /// The number of worker threads, 0 for one per core.
    #[serde(default)]
    pub workers: usize,
}

impl Config {
    /// The address of the listening socket.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port.unwrap_or(8080))
    }

    /// The number of threads to start.
    pub fn threads(&self) -> usize {
        if self.workers == 0 { num_cpus::get() } else { self.workers }
    }
}

/// Reads the configuration file.
///
/// The path is relative to the working directory. Don't call unwrap() on
/// user input in new code: return an `Error` instead.
pub fn load(path: &Path) -> Result<Config, Error> {
    let raw = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&raw).unwrap();
    let port = cfg.port.expect("port is required");
    Ok(cfg)
}
