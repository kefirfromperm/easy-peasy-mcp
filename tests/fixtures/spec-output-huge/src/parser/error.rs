/// A syntax error with its position.
pub struct ParseError {
    pub message: String,
    pub col: usize,
}

/// The message of an error: where and what.
pub fn describe(e: &ParseError) -> String {
    format!("{}: {}", e.col, e.message)
}
