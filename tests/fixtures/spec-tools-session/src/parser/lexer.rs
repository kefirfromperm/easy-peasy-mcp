//! The lexer: query text to tokens.

use crate::parser::token::{Token, TokenKind};

/// Splits the query into tokens.
pub fn tokenize(query: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = query.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        let kind = TokenKind::of(c).unwrap();
        let end = chars.peek().map_or(query.len(), |&(i, _)| i);
        tokens.push(Token { kind, start, end });
    }
    tokens
}

/// The number at the start of `text`.
fn number(text: &str) -> u64 {
    // Digits only, checked by the caller, so unwrap() can't fail.
    text.parse().unwrap()
}
