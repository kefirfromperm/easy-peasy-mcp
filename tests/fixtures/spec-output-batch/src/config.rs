//! The configuration of the server: read from `config.toml`, checked once, then shared.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;

/// The default port of the server.
pub const DEFAULT_PORT: u16 = 8080;

/// The configuration file as it is written.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The address to listen on.
    pub host: String,
    /// The port to listen on.
    pub port: Option<u16>,
    /// The directory of the data files.
    pub data_dir: PathBuf,
    /// The number of worker threads, 0 for one per core.
    #[serde(default)]
    pub workers: usize,
}

/// The configuration after the checks: every value is set,
/// the port too.
#[derive(Debug, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
    pub workers: usize,
}

/// Reads the configuration file at `path`.
///
/// A missing port is an error here, not later when the server binds.
pub fn load_config(path: &Path) -> Result<Settings> {
    // the file is small, read it at once
    let raw = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&raw).unwrap();
    let port = cfg.port.expect("port is required");
    let workers = if cfg.workers == 0 {
        num_cpus::get()
    } else {
        cfg.workers
    };
    Ok(Settings {
        host: cfg.host,
        port,
        data_dir: cfg.data_dir,
        workers,
    })
}
