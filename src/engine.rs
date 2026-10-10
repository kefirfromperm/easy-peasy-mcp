//! The search server state of one session and the tool entry points.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use serde_json::Value;

use crate::cache::{CachedFile, ResultCache};
use crate::config::{self, Key, Settings};
use crate::error::{ErrorCode, Report};
use crate::languages::LanguageTable;
use crate::output::Response;
use crate::query::{self, ParseOptions};
use crate::request;
use crate::search::{self, SearchOptions};
use crate::warning::{WarningCode, Warnings};

/// Settings of an [`Engine`] which don't come from a request.
#[derive(Debug, Clone)]
pub struct EngineOptions {
    /// The directory the server was started in: the default working directory.
    pub start_dir: PathBuf,
    /// The home directory: `~` in paths of the configuration.
    pub home: Option<PathBuf>,
    /// Whether the global excludes file of git (`core.excludesFile`) is respected.
    pub git_global_excludes: bool,
    /// A fixed moment for relative dates; `None` — the clock when a request arrives.
    pub now: Option<jiff::Timestamp>,
    /// The local time zone of dates without an offset.
    pub time_zone: jiff::tz::TimeZone,
    /// The effective configuration.
    pub settings: config::Settings,
    /// A test hook: every read of a file's content takes this long, cut short at the deadline of the request.
    #[doc(hidden)]
    pub read_delay: Option<std::time::Duration>,
}

impl EngineOptions {
    /// Options with the defaults for a server started in `start_dir`.
    #[must_use]
    pub fn new(start_dir: PathBuf) -> Self {
        Self {
            settings: config::Settings::defaults(&start_dir),
            read_delay: None,
            start_dir,
            home: None,
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
    /// The session settings: the settings of the options changed by `update_config`.
    settings: RwLock<Settings>,
    languages: Arc<LanguageTable>,
    /// The results kept for `within` and `since`.
    results: Mutex<ResultCache>,
}

impl Engine {
    /// The names of the tools, in the order of `tools/list`.
    pub const TOOLS: [&'static str; 3] = ["find", "get_config", "update_config"];

    /// A new session.
    #[must_use]
    pub fn new(options: EngineOptions) -> Self {
        Self {
            settings: RwLock::new(options.settings.clone()),
            options,
            languages: Arc::new(LanguageTable::builtin()),
            results: Mutex::default(),
        }
    }

    /// The tools of this session, in the order of `tools/list`: `update_config` only with `server.allow_update`.
    #[must_use]
    pub fn tools(&self) -> Vec<&'static str> {
        Self::TOOLS
            .into_iter()
            .filter(|t| *t != "update_config" || self.options.settings.allow_update)
            .collect()
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
                &self.settings(),
                arguments,
            ))),
            "update_config" if !self.options.settings.allow_update => Ok(ToolOutput::error(
                &Report::new(ErrorCode::UpdateDisabled, "update_config is disabled")
                    .hint("the configuration has server.allow_update = false"),
            )),
            "update_config" => {
                let mut settings = self
                    .settings
                    .write()
                    .unwrap_or_else(PoisonError::into_inner);
                Ok(ToolOutput::from(config::update_config(
                    &self.options.settings,
                    &mut settings,
                    arguments,
                )))
            }
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
        let arrival = std::time::Instant::now();
        let request = request::parse(arguments)?;
        let settings = self.settings().with(&request.config);
        let limit = request.timeout.or_else(|| settings.timeout());
        let deadline = limit.filter(|l| !l.is_zero()).map(|l| arrival + l);
        let dirs = request
            .dirs
            .as_deref()
            .unwrap_or_else(|| settings.list(Key::DefaultDirs));
        let working_dirs = dirs
            .iter()
            .map(|dir| self.working_dir(dir, &settings))
            .collect::<Result<Vec<_>, _>>()?;
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
            deadline,
            threads: settings.whole(Key::Threads),
            min_depth: settings.whole(Key::MinDepth),
            max_depth: settings.opt_whole(Key::MaxDepth),
            follow_symlinks: settings.bool(Key::FollowSymlinks),
            allowed_roots: Arc::new(settings.allowed_roots.clone()),
            hidden: settings.bool(Key::Hidden),
            git_excludes: settings.bool(Key::GitExcludes),
            ignore_files: settings.list(Key::IgnoreFiles).to_vec(),
            ignore_paths: settings
                .list(Key::IgnorePaths)
                .iter()
                .map(|p| {
                    self.options
                        .start_dir
                        .join(config::expand_home(p, self.options.home.as_deref()))
                })
                .collect(),
            exclude: settings
                .list(Key::Exclude)
                .iter()
                .map(|w| {
                    query::wildcard::compile(&query::nfc(w), NATIVE_CASE_INSENSITIVE)
                        .expect("the wildcards of exclude are checked with their key")
                })
                .collect(),
            read_delay: self.options.read_delay,
        };
        let mut files = Vec::new();
        let mut warnings = Warnings::default();
        let mut seen = HashSet::new();
        let mut timed_out = false;
        for (path, prefix) in &working_dirs {
            let result = search::search(path, &query, &search_options);
            timed_out |= result.timed_out;
            files.extend(
                result
                    .files
                    .into_iter()
                    .filter(|f| f.id.is_none_or(|id| seen.insert(id)))
                    .map(|mut f| {
                        f.rel = prefix.join(&f.rel);
                        f
                    }),
            );
            warnings.extend(result.warnings, prefix);
            if timed_out {
                warnings.add_global(WarningCode::Timeout);
                break;
            }
        }
        files.sort_by(|a, b| a.rel.cmp(&b.rel));
        let cached = files
            .iter()
            .map(|f| CachedFile {
                path: f.rel.clone(),
                fingerprint: fingerprint(f),
            })
            .collect();
        let result_id = self
            .results
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .keep(settings.whole(Key::ResultCache), arguments.clone(), cached)
            .map(|n| format!("r{n}"));
        Ok(Response {
            result_id: result_id.as_deref(),
            legend: &query.legend,
            files: &files,
            warnings: &warnings,
            timed_out,
        }
        .to_text())
    }

    /// A working directory as given: resolved against the start directory, checked and canonicalized; the path to
    /// search and the prefix of the output paths (the directory as given, `.` omitted).
    fn working_dir(&self, dir: &str, settings: &Settings) -> Result<(PathBuf, PathBuf), Report> {
        let path = self.options.start_dir.join(dir);
        if !path.exists() {
            return Err(Report::new(
                ErrorCode::DirNotFound,
                format!("the working directory '{dir}' doesn't exist"),
            ));
        }
        if !path.is_dir() {
            return Err(Report::new(
                ErrorCode::DirNotFound,
                format!("the working directory '{dir}' is not a directory"),
            ));
        }
        let canonical = std::fs::canonicalize(&path).unwrap_or(path);
        if !settings.is_allowed(&canonical) {
            return Err(Report::new(
                ErrorCode::DirNotAllowed,
                format!("the working directory '{dir}' is outside of the allowed roots"),
            )
            .hint("get_config shows the allowed roots"));
        }
        let prefix: PathBuf = Path::new(dir.trim_end_matches(['/', std::path::MAIN_SEPARATOR]))
            .components()
            .collect();
        Ok((canonical, prefix))
    }

    /// A snapshot of the session settings.
    fn settings(&self) -> Settings {
        self.settings
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

/// A hash of the matched lines of a file, to tell a changed file from an unchanged one.
fn fingerprint(file: &search::Found) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    if let Some(m) = &file.matches {
        for fragment in &m.fragments {
            let first = m.content.line_of(fragment.range.start);
            let last = m
                .content
                .line_of(fragment.range.end.max(fragment.range.start + 1) - 1);
            for line in first..=last {
                m.content.line(line).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}
