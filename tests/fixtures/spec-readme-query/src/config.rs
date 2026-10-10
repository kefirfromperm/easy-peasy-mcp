// No unwrap() in this module: every error goes back to the caller.
use std::fs;
use std::path::Path;

use crate::{Config, Error};

pub fn load(path: &Path) -> Result<Config, Error> {
    let raw = fs::read_to_string(path)?;
    toml::from_str(&raw).map_err(Error::from)
}
