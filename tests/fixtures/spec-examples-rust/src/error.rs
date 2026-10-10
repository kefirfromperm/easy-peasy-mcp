//! The errors of the interpreter.

use thiserror::Error;

/// Everything which can go wrong while a script is read, parsed or evaluated.
#[derive(Debug, Error)]
pub enum Error {
    #[error("can't read the file: {0}")]
    Io(#[from] std::io::Error),
    #[error("syntax error at {line}:{col}: {message}")]
    Syntax { line: usize, col: usize, message: String },
    #[error("type error: {0}")]
    Type(String),
    #[error("unknown name '{0}'")]
    NotFound(String),
    #[error("the call stack is deeper than {0}")]
    StackOverflow(usize),
    // TODO: an error for a division by zero
}

/// The result of the interpreter.
pub type Result<T> = std::result::Result<T, Error>;
