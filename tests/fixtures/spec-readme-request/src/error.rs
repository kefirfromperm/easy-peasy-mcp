//! The errors of the server.

use std::fmt;

/// An error of the server: what failed and why.
#[derive(Debug)]
pub enum Error {
    /// A file can't be read.
    Io(std::io::Error),
    /// The configuration is invalid.
    Config(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Config(message) => write!(f, "invalid configuration: {message}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
