pub fn next(text: &str) -> Result<char, ParseError> {
    text.chars().next().ok_or_else(|| ParseError::at(0)) // an error at the start
}
