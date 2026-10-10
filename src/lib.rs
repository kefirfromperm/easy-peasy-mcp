//! easy-peasy-mcp: a local stdio MCP server which finds files and places in files with one query.
//!
//! The entry point for tests and embedding is [`Engine`]; [`mcp::Server`] exposes it over MCP.

mod cache;
mod comments;
pub mod config;
mod content;
mod description;
mod engine;
mod error;
mod git;
mod languages;
pub mod mcp;
mod output;
mod query;
mod request;
mod search;
mod warning;

pub use engine::{Engine, EngineOptions, ToolOutput, UnknownTool};

/// The spec version this server implements (TOOLS.md#versioning).
pub const SPEC_VERSION: u32 = 1;
