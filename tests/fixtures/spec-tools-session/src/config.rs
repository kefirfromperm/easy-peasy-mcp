//! The configuration of the application.

use std::env;
use std::fs;
use std::path::PathBuf;

/// The settings of the application.
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub root: PathBuf,
    pub port: u16,
    pub threads: usize,
    pub log: String,
}

/// Reads `app.toml` and the environment.
pub fn load() -> Config {
    let home = env::var("APP_HOME").unwrap();
    let raw = fs::read_to_string(PathBuf::from(&home).join("app.toml")).unwrap();
    let mut cfg: Config = toml::from_str(&raw).unwrap();
    if let Ok(port) = env::var("APP_PORT") {
        cfg.port = port.parse().unwrap();
    }
    if let Ok(threads) = env::var("APP_THREADS") {
        cfg.threads = threads.parse().unwrap();
    }
    if let Ok(log) = env::var("APP_LOG") {
        cfg.log = log.split(',').next().unwrap().to_string();
    }
    cfg.root = fs::canonicalize(&cfg.root).unwrap().to_str().unwrap().into();
    cfg
}
