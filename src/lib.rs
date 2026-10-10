//! easy-peasy-mcp: a local stdio MCP server which finds files and places in files with one query.

use std::path::PathBuf;

/// The spec version this server implements.
pub const SPEC_VERSION: u32 = 1;

/// Settings of an [`Engine`] which don't come from a request.
#[derive(Debug, Clone)]
pub struct EngineOptions {
    /// The directory the server was started in: the default working directory.
    pub start_dir: PathBuf,
    /// Whether the global excludes file of git (`core.excludesFile`) is respected.
    pub git_global_excludes: bool,
}

impl EngineOptions {
    /// Options with the defaults for a server started in `start_dir`.
    #[must_use]
    pub fn new(start_dir: PathBuf) -> Self {
        Self { start_dir, git_global_excludes: true }
    }
}

/// The text result of a tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolOutput {
    /// The text content item.
    pub text: String,
    /// `isError` of the tool result.
    pub is_error: bool,
}

/// The search server state of one session.
#[derive(Debug)]
pub struct Engine {
    options: EngineOptions,
}

impl Engine {
    /// A new session.
    #[must_use]
    pub fn new(options: EngineOptions) -> Self {
        Self { options }
    }

    /// Calls a tool by name.
    #[must_use]
    pub fn call(&self, _tool: &str, _arguments: &serde_json::Value) -> ToolOutput {
        let _ = &self.options;
        ToolOutput { text: "error[not_implemented]: find is not implemented yet\n".into(), is_error: true }
    }

    /// The `find` tool.
    #[must_use]
    pub fn find(&self, arguments: &serde_json::Value) -> ToolOutput {
        self.call("find", arguments)
    }
}
