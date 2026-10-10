//! The query parser.
//!
//! The parser never panics on its input: no unwrap() here, every error has a position.

pub mod ast;
pub mod lexer;
pub mod token;
