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
mod output_file;
mod paths;
mod query;
mod recycle;
mod relevance;
mod rename;
mod replace;
mod request;
mod search;
mod shown;
mod template;
mod warning;
mod write;

pub use engine::{Engine, EngineOptions, ToolOutput, UnknownTool};
pub use recycle::Trash;
pub use search::Progress;

/// The current spec version of this server (TOOLS.md#versioning).
pub const SPEC_VERSION: u32 = 2;
