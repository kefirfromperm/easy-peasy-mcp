//! The search server state of one session and the tool entry points.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

use crate::config;
use crate::error::{ErrorCode, Report};
use crate::languages::LanguageTable;
use crate::output::Response;
use crate::query::{self, ParseOptions};
use crate::request;
use crate::search::{self, SearchOptions};

/// Settings of an [`Engine`] which don't come from a request.
#[derive(Debug, Clone)]
pub struct EngineOptions {
    /// The directory the server was started in: the default working directory.
    pub start_dir: PathBuf,
    /// Whether the global excludes file of git (`core.excludesFile`) is respected.
    pub git_global_excludes: bool,
    /// A fixed moment for relative dates; `None` — the clock when a request arrives.
    pub now: Option<jiff::Timestamp>,
    /// The local time zone of dates without an offset.
    pub time_zone: jiff::tz::TimeZone,
    /// The effective configuration.
    pub settings: config::Settings,
}

impl EngineOptions {
    /// Options with the defaults for a server started in `start_dir`.
    #[must_use]
    pub fn new(start_dir: PathBuf) -> Self {
        Self {
            settings: config::Settings::defaults(&start_dir),
            start_dir,
            git_global_excludes: true,
            now: None,
            time_zone: jiff::tz::TimeZone::system(),
        }
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

impl ToolOutput {
    fn error(report: &Report) -> Self {
        Self {
            text: report.to_text(),
            is_error: true,
        }
    }
}

impl From<Result<String, Report>> for ToolOutput {
    fn from(result: Result<String, Report>) -> Self {
        match result {
            Ok(text) => Self {
                text,
                is_error: false,
            },
            Err(report) => Self::error(&report),
        }
    }
}

/// A tool name the server doesn't have.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown tool '{0}'")]
pub struct UnknownTool(pub String);

/// `path_case = native`: case-insensitive on macOS and Windows.
const NATIVE_CASE_INSENSITIVE: bool = cfg!(any(target_os = "macos", target_os = "windows"));

/// The search server state of one session: options and result ids.
#[derive(Debug)]
pub struct Engine {
    options: EngineOptions,
    languages: Arc<LanguageTable>,
    last_result: AtomicU64,
}

impl Engine {
    /// The names of the tools, in the order of `tools/list`.
    pub const TOOLS: [&'static str; 3] = ["find", "get_config", "update_config"];

    /// A new session.
    #[must_use]
    pub fn new(options: EngineOptions) -> Self {
        Self {
            options,
            languages: Arc::new(LanguageTable::builtin()),
            last_result: AtomicU64::new(0),
        }
    }

    /// Calls a tool by name with its JSON arguments.
    ///
    /// # Errors
    ///
    /// [`UnknownTool`] if the server has no tool with this name.
    pub fn call(&self, tool: &str, arguments: &Value) -> Result<ToolOutput, UnknownTool> {
        match tool {
            "find" => Ok(self.find(arguments)),
            "get_config" => Ok(ToolOutput::from(config::get_config(
                &self.options.settings,
                arguments,
            ))),
            "update_config" => Ok(ToolOutput::from(config::update_config(arguments))),
            other => Err(UnknownTool(other.to_string())),
        }
    }

    /// The `find` tool.
    #[must_use]
    pub fn find(&self, arguments: &Value) -> ToolOutput {
        match self.try_find(arguments) {
            Ok(text) => ToolOutput {
                text,
                is_error: false,
            },
            Err(report) => ToolOutput::error(&report),
        }
    }

    fn try_find(&self, arguments: &Value) -> Result<String, Report> {
        let request = request::parse(arguments)?;
        let working_dir = std::fs::canonicalize(&self.options.start_dir)
            .unwrap_or_else(|_| self.options.start_dir.clone());
        if !self.options.settings.is_allowed(&working_dir) {
            return Err(Report::new(
                ErrorCode::DirNotAllowed,
                "the working directory '.' is outside of the allowed roots",
            )
            .hint("get_config shows the allowed roots"));
        }
        let options = ParseOptions {
            case_insensitive_paths: NATIVE_CASE_INSENSITIVE,
            now: self.options.now.unwrap_or_else(jiff::Timestamp::now),
            time_zone: self.options.time_zone.clone(),
            languages: Arc::clone(&self.languages),
        };
        let query =
            query::parse(&request.query, &options).map_err(|e| e.into_report(&request.query))?;
        let search_options = SearchOptions {
            git_global_excludes: self.options.git_global_excludes,
            case_insensitive_paths: NATIVE_CASE_INSENSITIVE,
            languages: Arc::clone(&self.languages),
        };
        let result = search::search(&self.options.start_dir, &query, &search_options);
        let result_id = format!("r{}", self.last_result.fetch_add(1, Ordering::SeqCst) + 1);
        Ok(Response {
            result_id: &result_id,
            legend: &query.legend,
            files: &result.files,
            warnings: &result.warnings,
        }
        .to_text())
    }
}
