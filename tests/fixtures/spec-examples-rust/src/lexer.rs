//! Splits the source of a script into tokens.

use crate::error::{Error, Result};

/// A token with its position: line and column, 1-based.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Str(String),
    Ident(String),
    Keyword(&'static str),
    Op(&'static str),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Eof,
}

const KEYWORDS: [&str; 8] = ["let", "fn", "if", "else", "while", "return", "true", "false"];
const OPS: [&str; 14] = ["==", "!=", "<=", ">=", "&&", "||", "+", "-", "*", "/", "%", "<", ">", "="];

/// The tokens of `source`, the last one is `Eof`.
pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let (mut i, mut line, mut col) = (0, 1, 1);
    while i < chars.len() {
        let c = chars[i];
        let start = (line, col);
        if c == '\n' {
            line += 1;
            col = 1;
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            i += 1;
            col += 1;
            continue;
        }
        let kind = if c.is_ascii_digit() {
            let end = (i..chars.len()).find(|&j| !chars[j].is_ascii_digit() && chars[j] != '.').unwrap_or(chars.len());
            let text: String = chars[i..end].iter().collect();
            let n = text.parse().map_err(|_| syntax(start, "invalid number"))?;
            col += end - i;
            i = end;
            TokenKind::Number(n)
        } else if c == '"' {
            // TODO: escapes in strings
            let end = (i + 1..chars.len()).find(|&j| chars[j] == '"').ok_or_else(|| syntax(start, "unterminated string"))?;
            let text: String = chars[i + 1..end].iter().collect();
            col += end + 1 - i;
            i = end + 1;
            TokenKind::Str(text)
        } else if c.is_alphabetic() || c == '_' {
            let end = (i..chars.len()).find(|&j| !chars[j].is_alphanumeric() && chars[j] != '_').unwrap_or(chars.len());
            let text: String = chars[i..end].iter().collect();
            col += end - i;
            i = end;
            match KEYWORDS.iter().find(|k| **k == text) {
                Some(k) => TokenKind::Keyword(k),
                None => TokenKind::Ident(text),
            }
        } else {
            let single = match c {
                '(' => Some(TokenKind::LParen),
                ')' => Some(TokenKind::RParen),
                '{' => Some(TokenKind::LBrace),
                '}' => Some(TokenKind::RBrace),
                ',' => Some(TokenKind::Comma),
                ';' => Some(TokenKind::Semicolon),
                _ => None,
            };
            if let Some(kind) = single {
                i += 1;
                col += 1;
                kind
            } else {
                // TODO: comments in scripts
                let rest: String = chars[i..chars.len().min(i + 2)].iter().collect();
                let op = OPS.iter().find(|op| rest.starts_with(**op)).ok_or_else(|| syntax(start, "unexpected character"))?;
                i += op.len();
                col += op.len();
                TokenKind::Op(op)
            }
        };
        tokens.push(Token { kind, line: start.0, col: start.1 });
    }
    tokens.push(Token { kind: TokenKind::Eof, line, col });
    Ok(tokens)
}

fn syntax((line, col): (usize, usize), message: &str) -> Error {
    Error::Syntax { line, col, message: message.to_string() }
}
