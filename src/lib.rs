//! easy-peasy-mcp: a local stdio MCP server which finds files and places in files with one query.
//!
//! The entry point for tests and embedding is [`Engine`]; [`mcp::Server`] exposes it over MCP.

mod archive;
mod batch;
mod cache;
mod comments;
mod compressed;
pub mod config;
mod content;
mod cursor;
mod description;
mod engine;
mod error;
mod git;
mod hints;
mod languages;
pub mod mcp;
mod output;
mod query;
mod relevance;
mod request;
mod search;
mod warning;

pub use engine::{Engine, EngineOptions, ToolOutput, UnknownTool};
pub use search::Progress;

/// The spec version this server implements (TOOLS.md#versioning).
pub const SPEC_VERSION: u32 = 1;
