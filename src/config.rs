//! The config file: where it is, and its checks at start (CONFIG.md#config-file).

use std::fmt;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::SPEC_VERSION;
use crate::error::{ErrorCode, Report};
use crate::query::did_you_mean;

/// An error which stops the server at start: `<path>:<line>: <message>`, or `<path>: <message>` without a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartError {
    /// The config file; `None` for an error of the command line.
    pub path: Option<PathBuf>,
    /// 1-based.
    pub line: Option<usize>,
    pub message: String,
}

impl fmt::Display for StartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.path, self.line) {
            (Some(path), Some(line)) => write!(f, "{}:{line}: {}", path.display(), self.message),
            (Some(path), None) => write!(f, "{}: {}", path.display(), self.message),
            (None, _) => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for StartError {}

/// The sections of the config file and their keys (CONFIG.md#full-example); `languages` takes any language name.
const SECTIONS: [(&str, &[&str]); 8] = [
    (
        "server",
        &[
            "allowed_roots",
            "default_dirs",
            "timeout",
            "threads",
            "allow_update",
            "result_cache",
        ],
    ),
    (
        "traversal",
        &[
            "min_depth",
            "max_depth",
            "follow_symlinks",
            "hidden",
            "ignore_files",
            "git_excludes",
            "ignore_paths",
            "exclude",
            "path_case",
            "binary",
            "max_file_size",
            "encoding",
            "fallback_encoding",
            "archives",
            "max_archive_size",
            "skip_generated",
        ],
    ),
    (
        "output",
        &[
            "format",
            "detail",
            "budget",
            "chars_per_token",
            "funnel",
            "facets",
            "facet_depth",
            "facet_limit",
            "meta",
            "context",
            "before",
            "after",
            "path_fragments",
            "highlight",
            "markers",
            "max_files",
            "max_lines",
            "max_matches",
            "max_matches_per_file",
            "max_line_length",
            "max_output_per_file",
            "max_output",
            "dedent",
            "blank_context",
            "group_dirs",
            "group_repeated",
            "sort",
            "path_style",
        ],
    ),
    ("escape", &["max_level"]),
    ("query", &["fuzzy_min_score"]),
    (
        "kinds",
        &[
            "generated_markers",
            "generated_marker_lines",
            "generated",
            "test",
            "doc",
            "config",
        ],
    ),
    ("git", &["base"]),
    ("languages", &[]),
];

/// The config file to read: `--config`, a non-empty `$EASY_PEASY_CONFIG`, or the file of the platform config
/// directory when it exists; `None` for the built-in defaults. `env` looks up an environment variable, `os` is
/// `std::env::consts::OS`.
pub fn locate(
    cli: Option<&Path>,
    env: &dyn Fn(&str) -> Option<String>,
    os: &str,
    exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if let Some(path) = cli {
        return Some(path.to_path_buf());
    }
    if let Some(path) = env("EASY_PEASY_CONFIG").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(path));
    }
    let var = |name: &str| env(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    let dir = match os {
        "windows" => var("APPDATA"),
        "macos" => var("HOME").map(|home| home.join("Library").join("Application Support")),
        _ => var("XDG_CONFIG_HOME")
            .filter(|dir| dir.is_absolute())
            .or_else(|| var("HOME").map(|home| home.join(".config"))),
    }?;
    let file = dir.join("easy-peasy-mcp").join("config.toml");
    exists(&file).then_some(file)
}

/// Reads and checks the config file at `path`.
///
/// # Errors
///
/// The file doesn't exist or can't be read, isn't TOML, or has a section or key which doesn't exist or isn't
/// implemented yet.
pub fn load(path: &Path) -> Result<ConfigFile, StartError> {
    let error = |line, message: String| StartError {
        path: Some(path.to_path_buf()),
        line,
        message,
    };
    let text = std::fs::read_to_string(path).map_err(|e| {
        let message = if e.kind() == std::io::ErrorKind::NotFound {
            "the config file doesn't exist"
        } else {
            "the config file can't be read"
        };
        error(None, message.to_string())
    })?;
    let values = check(&text).map_err(|(line, message)| error(Some(line), message))?;
    Ok(ConfigFile {
        path: path.to_path_buf(),
        values,
    })
}

/// The keys this server version implements.
const IMPLEMENTED: [(&str, &str); 4] = [
    ("server", "allowed_roots"),
    ("server", "default_dirs"),
    ("server", "timeout"),
    ("server", "threads"),
];

/// Where a value comes from (`CONFIG.md#get_config`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Default,
    /// The config file, as given or found.
    File(PathBuf),
    Cli,
    /// `update_config`.
    Update,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => f.write_str("default"),
            Self::File(path) => write!(f, "file: {}", path.display()),
            Self::Cli => f.write_str("cli"),
            Self::Update => f.write_str("update"),
        }
    }
}

/// The values of the implemented keys of a config file, with the lines of their keys.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileValues {
    /// `server.allowed_roots` as written.
    pub allowed_roots: Option<(Vec<String>, usize)>,
    /// `server.default_dirs` as written.
    pub default_dirs: Option<Vec<String>>,
    /// `server.timeout` as written, a valid duration.
    pub timeout: Option<String>,
    /// `server.threads`.
    pub threads: Option<usize>,
}

/// A checked config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub values: FileValues,
}

/// The effective configuration of the server: an immutable value, resolved at start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// `server.allowed_roots`: canonical, never empty.
    pub allowed_roots: Vec<PathBuf>,
    pub allowed_roots_source: Source,
    /// `server.default_dirs` as written, never empty.
    pub default_dirs: Vec<String>,
    pub default_dirs_source: Source,
    /// `server.timeout` as written, a valid duration.
    pub timeout: String,
    pub timeout_source: Source,
    /// `server.threads`: `0` — the number of CPU cores.
    pub threads: usize,
    pub threads_source: Source,
}

impl Settings {
    /// The built-in defaults of a server started in `start_dir`.
    #[must_use]
    pub fn defaults(start_dir: &Path) -> Self {
        Self {
            allowed_roots: vec![canonical_or_same(start_dir)],
            allowed_roots_source: Source::Default,
            default_dirs: vec![".".to_string()],
            default_dirs_source: Source::Default,
            timeout: "30s".to_string(),
            timeout_source: Source::Default,
            threads: 0,
            threads_source: Source::Default,
        }
    }

    /// The settings from the defaults, the config file and the command line (`--root`): the command line list
    /// replaces the file list; roots are expanded (`~` to `home`), resolved against `start_dir` and canonicalized.
    ///
    /// # Errors
    ///
    /// A root which doesn't exist.
    pub fn resolve(
        file: Option<&ConfigFile>,
        cli: &CommandLine,
        start_dir: &Path,
        home: Option<&Path>,
    ) -> Result<Self, StartError> {
        let cli_roots = &cli.roots;
        let mut settings = Self::defaults(start_dir);
        if let Some(file) = file
            && let Some(timeout) = &file.values.timeout
        {
            settings.timeout.clone_from(timeout);
            settings.timeout_source = Source::File(file.path.clone());
        }
        if let Some(file) = file
            && let Some(threads) = file.values.threads
        {
            settings.threads = threads;
            settings.threads_source = Source::File(file.path.clone());
        }
        if let Some(timeout) = &cli.timeout {
            settings.timeout.clone_from(timeout);
            settings.timeout_source = Source::Cli;
        }
        if let Some(file) = file
            && let Some(dirs) = &file.values.default_dirs
        {
            settings.default_dirs.clone_from(dirs);
            settings.default_dirs_source = Source::File(file.path.clone());
        }
        let (roots, source, line) = if !cli_roots.is_empty() {
            (cli_roots.clone(), Source::Cli, None)
        } else if let Some(file) = file
            && let Some((roots, line)) = &file.values.allowed_roots
        {
            (roots.clone(), Source::File(file.path.clone()), Some(*line))
        } else {
            return Ok(settings);
        };
        let mut allowed_roots = Vec::new();
        for root in &roots {
            let expanded = match (root.strip_prefix('~'), home) {
                (Some(rest), Some(home)) if rest.is_empty() || rest.starts_with(['/', '\\']) => {
                    home.join(rest.trim_start_matches(['/', '\\']))
                }
                _ => PathBuf::from(root),
            };
            let canonical =
                std::fs::canonicalize(start_dir.join(expanded)).map_err(|_| StartError {
                    path: file.filter(|_| line.is_some()).map(|f| f.path.clone()),
                    line,
                    message: format!("the allowed root '{root}' doesn't exist"),
                })?;
            allowed_roots.push(canonical);
        }
        if allowed_roots.is_empty() {
            allowed_roots.push(canonical_or_same(start_dir));
        }
        settings.allowed_roots = allowed_roots;
        settings.allowed_roots_source = source;
        Ok(settings)
    }

    /// The time limit of a search; `None` — no limit.
    #[must_use]
    pub fn timeout(&self) -> Option<std::time::Duration> {
        parse_duration(&self.timeout).filter(|d| !d.is_zero())
    }

    /// The value of a key which `update_config` changes, as TOML.
    fn value(&self, key: Mutable) -> String {
        match key {
            Mutable::DefaultDirs => toml_list(self.default_dirs.iter().map(String::as_str)),
            Mutable::Timeout => format!("\"{}\"", self.timeout),
            Mutable::Threads => self.threads.to_string(),
        }
    }

    /// Sets a key to its value of `base`, with its source.
    fn reset(&mut self, key: Mutable, base: &Settings) {
        match key {
            Mutable::DefaultDirs => {
                self.default_dirs.clone_from(&base.default_dirs);
                self.default_dirs_source = base.default_dirs_source.clone();
            }
            Mutable::Timeout => {
                self.timeout.clone_from(&base.timeout);
                self.timeout_source = base.timeout_source.clone();
            }
            Mutable::Threads => {
                self.threads = base.threads;
                self.threads_source = base.threads_source.clone();
            }
        }
    }

    /// Sets a key to a checked value of `update_config`.
    fn set(&mut self, value: MutableValue) {
        match value {
            MutableValue::DefaultDirs(dirs) => {
                self.default_dirs = dirs;
                self.default_dirs_source = Source::Update;
            }
            MutableValue::Timeout(timeout) => {
                self.timeout = timeout;
                self.timeout_source = Source::Update;
            }
            MutableValue::Threads(threads) => {
                self.threads = threads;
                self.threads_source = Source::Update;
            }
        }
    }

    /// Whether the canonical `dir` is inside an allowed root.
    #[must_use]
    pub fn is_allowed(&self, dir: &Path) -> bool {
        self.allowed_roots.iter().any(|root| dir.starts_with(root))
    }
}

/// The canonical form of `path`, or `path` itself when it can't be resolved.
fn canonical_or_same(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// The keys which `update_config` changes, in the order of the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mutable {
    DefaultDirs,
    Timeout,
    Threads,
}

impl Mutable {
    const ALL: [Mutable; 3] = [Self::DefaultDirs, Self::Timeout, Self::Threads];

    /// The section and the key.
    fn name(self) -> (&'static str, &'static str) {
        match self {
            Self::DefaultDirs => ("server", "default_dirs"),
            Self::Timeout => ("server", "timeout"),
            Self::Threads => ("server", "threads"),
        }
    }

    fn find(section: &str, key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.name() == (section, key))
    }

    /// The checked value of `item`, or the hint of `invalid_value`.
    fn parse(self, item: &toml_edit::Item) -> Result<MutableValue, &'static str> {
        match self {
            Self::DefaultDirs => string_list(item)
                .filter(|dirs| !dirs.is_empty())
                .map(MutableValue::DefaultDirs)
                .ok_or(NON_EMPTY_PATHS),
            Self::Timeout => item
                .as_str()
                .filter(|t| parse_duration(t).is_some())
                .map(|t| MutableValue::Timeout(t.to_string()))
                .ok_or(DURATION),
            Self::Threads => item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .map(MutableValue::Threads)
                .ok_or(THREADS),
        }
    }
}

/// A checked value of a key which `update_config` changes.
#[derive(Debug, Clone, PartialEq, Eq)]
enum MutableValue {
    DefaultDirs(Vec<String>),
    Timeout(String),
    Threads(usize),
}

/// What `default_dirs` must be.
const NON_EMPTY_PATHS: &str = "a non-empty list of paths is expected";

/// What `threads` must be.
const THREADS: &str = "a whole number is expected (0: the number of CPU cores)";

/// What `timeout` must be.
const DURATION: &str = "a duration like \"30s\" is expected (units ms, s, m)";

/// A duration of the configuration: a whole number and `ms`, `s` or `m` (`CONFIG.md#server`).
#[must_use]
pub fn parse_duration(text: &str) -> Option<std::time::Duration> {
    let digits = text.find(|c: char| !c.is_ascii_digit())?;
    let (number, unit) = text.split_at(digits);
    let number: u64 = number.parse().ok()?;
    match unit {
        "ms" => Some(std::time::Duration::from_millis(number)),
        "s" => Some(std::time::Duration::from_secs(number)),
        "m" => number.checked_mul(60).map(std::time::Duration::from_secs),
        _ => None,
    }
}

/// The values of the command line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandLine {
    /// `--root`, in order.
    pub roots: Vec<String>,
    /// `--timeout`, a valid duration.
    pub timeout: Option<String>,
}

/// A list of strings as TOML.
fn toml_list<'a>(items: impl Iterator<Item = &'a str>) -> String {
    format!(
        "[{}]",
        items.map(toml_string).collect::<Vec<_>>().join(", ")
    )
}

/// A path as a TOML string, without the `\\?\` prefix of Windows.
fn toml_path(path: &Path) -> String {
    let text = path.to_string_lossy();
    toml_string(text.strip_prefix(r"\\?\").unwrap_or(&text))
}

/// A string in TOML: literal (`'...'`) unless it contains a `'`.
fn toml_string(text: &str) -> String {
    if text.contains('\'') {
        format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        format!("'{text}'")
    }
}

/// What a TOML document holds, by the sections of the spec.
enum Entry<'d> {
    /// A top level name which is not a section of the spec.
    UnknownSection(&'d str),
    /// A section of the spec written as a value.
    NotASection(&'d str),
    /// A key of a section and its value; for `languages` a language.
    Key {
        section: &'static str,
        key: &'d str,
        item: &'d toml_edit::Item,
    },
}

/// The entries of a TOML document with their byte offsets, in the order of the text.
fn entries<S>(doc: &toml_edit::Document<S>) -> Vec<(usize, Entry<'_>)> {
    let root = doc.as_table();
    let offset = |key: Option<&toml_edit::Key>, item: &toml_edit::Item| {
        key.and_then(toml_edit::Key::span)
            .or_else(|| item.span())
            .map_or(0, |s| s.start)
    };
    let mut entries = Vec::new();
    for (name, item) in root {
        let at = offset(root.key(name), item);
        let Some((section, _)) = SECTIONS.iter().find(|(section, _)| *section == name) else {
            entries.push((at, Entry::UnknownSection(name)));
            continue;
        };
        let Some(table) = item.as_table_like() else {
            entries.push((at, Entry::NotASection(name)));
            continue;
        };
        for (key, value) in table.iter() {
            let at = offset(table.get_key_value(key).map(|(k, _)| k), value);
            entries.push((
                at,
                Entry::Key {
                    section,
                    key,
                    item: value,
                },
            ));
        }
    }
    entries.sort_by_key(|(at, _)| *at);
    entries
}

/// The names of the sections, in the order of the spec.
fn section_names() -> Vec<&'static str> {
    SECTIONS.iter().map(|(section, _)| *section).collect()
}

/// Whether the spec defines `key` in `section`: one of its keys, or any language in `languages`.
fn is_spec_key(section: &str, key: &str) -> bool {
    section == "languages"
        || SECTIONS
            .iter()
            .any(|(name, keys)| *name == section && keys.contains(&key))
}

/// The key of the spec closest to `key` in `section`.
fn close_key(section: &str, key: &str) -> Option<&'static str> {
    let (_, keys) = SECTIONS.iter().find(|(name, _)| *name == section)?;
    did_you_mean(key, keys)
}

/// Checks the text of a config file and returns the values of the implemented keys; the error: the 1-based line and
/// the message of the first problem.
fn check(text: &str) -> Result<FileValues, (usize, String)> {
    let line = |offset: usize| text[..offset.min(text.len())].matches('\n').count() + 1;
    let doc = toml_edit::Document::parse(text).map_err(|e| {
        let offset = e.span().map_or(0, |s| s.start);
        (line(offset), format!("invalid TOML: {}", e.message()))
    })?;
    let mut values = FileValues::default();
    for (at, entry) in entries(&doc) {
        let message = match entry {
            Entry::UnknownSection(name) => format!(
                "unknown section '{name}', expected one of: {}",
                section_names().join(", ")
            ),
            Entry::NotASection(name) => format!("'{name}' must be a section"),
            Entry::Key {
                section: "server",
                key: "allowed_roots",
                item,
            } => match string_list(item) {
                Some(roots) => {
                    values.allowed_roots = Some((roots, line(at)));
                    continue;
                }
                None => "invalid value for 'server.allowed_roots': a list of paths is expected"
                    .to_string(),
            },
            Entry::Key {
                section: "server",
                key: key @ ("default_dirs" | "timeout" | "threads"),
                item,
            } => match Mutable::find("server", key).map(|m| m.parse(item)) {
                Some(Ok(MutableValue::DefaultDirs(dirs))) => {
                    values.default_dirs = Some(dirs);
                    continue;
                }
                Some(Ok(MutableValue::Timeout(timeout))) => {
                    values.timeout = Some(timeout);
                    continue;
                }
                Some(Ok(MutableValue::Threads(threads))) => {
                    values.threads = Some(threads);
                    continue;
                }
                Some(Err(expected)) => format!("invalid value for 'server.{key}': {expected}"),
                None => unreachable!("default_dirs, timeout and threads are mutable keys"),
            },
            Entry::Key { section, key, .. } if is_spec_key(section, key) => {
                format!("the key '{section}.{key}' is not implemented yet")
            }
            Entry::Key { section, key, .. } => match close_key(section, key) {
                Some(close) => {
                    format!("unknown key '{section}.{key}', did you mean '{section}.{close}'?")
                }
                None => format!("unknown key '{section}.{key}'"),
            },
        };
        return Err((line(at), message));
    }
    Ok(values)
}

/// A TOML array of strings.
fn string_list(item: &toml_edit::Item) -> Option<Vec<String>> {
    item.as_array()?
        .iter()
        .map(|v| v.as_str().map(str::to_string))
        .collect()
}

/// The fields of `get_config`, in the order of its input schema.
const GET_CONFIG_FIELDS: [&str; 2] = ["section", "sources"];

/// The `get_config` tool (`CONFIG.md#get_config`): the effective configuration as TOML. The first line names the server
/// and the spec versions; the read-only keys and the sections with their keys follow, as far as they are
/// implemented — no key is implemented yet, so `section` and `sources` don't change the result.
pub(crate) fn get_config(settings: &Settings, arguments: &Value) -> Result<String, Report> {
    let object = arguments_object(arguments, &GET_CONFIG_FIELDS)?;
    let section = match object.get("section") {
        None => None,
        Some(Value::String(section)) => {
            let names = section_names();
            if !names.contains(&section.as_str()) {
                return Err(unknown_name("section", section, &names));
            }
            Some(section.as_str())
        }
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'section' must be a string",
            ));
        }
    };
    let sources = match object.get("sources") {
        None => false,
        Some(Value::Bool(sources)) => *sources,
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'sources' must be a boolean",
            ));
        }
    };
    let mut out = format!(
        "# easy-peasy-mcp {}, spec {SPEC_VERSION}, supported specs: {SPEC_VERSION}\n# read-only: {}\n",
        env!("CARGO_PKG_VERSION"),
        IMPLEMENTED
            .iter()
            .filter(|(s, k)| READ_ONLY.iter().any(|(rs, rk, _)| rs == s && rk == k))
            .map(|(s, k)| format!("{s}.{k}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    if section.is_none_or(|s| s == "server") {
        let roots: Vec<String> = settings
            .allowed_roots
            .iter()
            .map(|r| toml_path(r))
            .collect();
        let lines = [
            (
                format!("allowed_roots = [{}]", roots.join(", ")),
                &settings.allowed_roots_source,
            ),
            (
                format!("default_dirs = {}", settings.value(Mutable::DefaultDirs)),
                &settings.default_dirs_source,
            ),
            (
                format!("timeout = {}", settings.value(Mutable::Timeout)),
                &settings.timeout_source,
            ),
            (
                format!("threads = {}", settings.value(Mutable::Threads)),
                &settings.threads_source,
            ),
        ];
        out.push_str("\n[server]\n");
        push_lines(&mut out, &lines, sources);
    }
    Ok(out)
}

/// The `key = value` lines of a section, with the source comments four spaces after the line.
fn push_lines(out: &mut String, lines: &[(String, &Source)], sources: bool) {
    for (line, source) in lines {
        if sources {
            let _ = writeln!(out, "{line}    # {source}");
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
}

/// `invalid_option` for a name which is not one of `names`, with `did you mean` and the expected names.
fn unknown_name(what: &str, name: &str, names: &[&str]) -> Report {
    let expected = format!("expected one of: {}", names.join(", "));
    let hint = match did_you_mean(name, names) {
        Some(close) => format!("did you mean '{close}'? {expected}"),
        None => expected,
    };
    Report::new(ErrorCode::InvalidOption, format!("unknown {what} '{name}'")).hint(hint)
}

/// The fields of `update_config`, in the order of its input schema.
const UPDATE_CONFIG_FIELDS: [&str; 2] = ["set", "reset"];

/// The keys which only the config file and the command line set.
const READ_ONLY: [(&str, &str, &str); 2] = [
    (
        "server",
        "allowed_roots",
        "allowed roots are set only at start",
    ),
    (
        "server",
        "allow_update",
        "allow_update is set only at start",
    ),
];

/// The `update_config` tool (`CONFIG.md#update_config`): the `reset` keys, then the `set` TOML, checked in this order.
/// No key is implemented yet, so a valid call changes nothing.
pub(crate) fn update_config(
    base: &Settings,
    session: &mut Settings,
    arguments: &Value,
) -> Result<String, Report> {
    let object = arguments_object(arguments, &UPDATE_CONFIG_FIELDS)?;
    let set = match object.get("set") {
        None => "",
        Some(Value::String(set)) => set.as_str(),
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'set' must be a string",
            ));
        }
    };
    let reset: Vec<&str> = match object.get("reset") {
        None => Some(Vec::new()),
        Some(Value::Array(keys)) => keys.iter().map(Value::as_str).collect::<Option<_>>(),
        Some(_) => None,
    }
    .ok_or_else(|| {
        Report::new(
            ErrorCode::InvalidOption,
            "'reset' must be a list of strings",
        )
    })?;
    let resets = reset_keys(&reset)?;
    let doc = toml_edit::Document::parse(set).map_err(|e| {
        let offset = e.span().map_or(0, |s| s.start).min(set.len());
        let line_start = set[..offset].rfind('\n').map_or(0, |i| i + 1);
        let line = set[..offset].matches('\n').count() + 1;
        let column = set[line_start..offset].chars().count() + 1;
        Report::new(ErrorCode::InvalidToml, "invalid TOML in 'set'")
            .hint(format!("line {line}, col {column}: {}", e.message()))
    })?;
    let mut values = Vec::new();
    for (_, entry) in entries(&doc) {
        match entry {
            Entry::UnknownSection(name) => {
                return Err(unknown_key(name)
                    .hint(format!("expected one of: {}", section_names().join(", "))));
            }
            Entry::NotASection(name) => {
                return Err(Report::new(
                    ErrorCode::UnknownKey,
                    format!("'{name}' must be a section"),
                ));
            }
            Entry::Key { section, key, item } => {
                let mutable = check_update_key(section, key)?;
                let value = mutable.parse(item).map_err(|expected| {
                    Report::new(
                        ErrorCode::InvalidValue,
                        format!("invalid value for '{section}.{key}'"),
                    )
                    .hint(expected)
                })?;
                values.push((mutable, value));
            }
        }
    }
    let mut updated = session.clone();
    for key in &resets {
        updated.reset(*key, base);
    }
    let set_keys: Vec<Mutable> = values.iter().map(|(key, _)| *key).collect();
    for (_, value) in values {
        updated.set(value);
    }
    let out = changes(session, &updated, &resets, &set_keys);
    *session = updated;
    Ok(out)
}

/// The keys of `reset`, `all` for every key, or the error of the first invalid one.
fn reset_keys(reset: &[&str]) -> Result<Vec<Mutable>, Report> {
    let mut resets = Vec::new();
    for &key in reset {
        if key == "all" {
            resets.extend(Mutable::ALL);
            continue;
        }
        let Some((section, name)) = key.split_once('.') else {
            let hint = if section_names().contains(&key) {
                format!("expected a key like '{key}.max_files', or 'all'")
            } else {
                format!("expected one of: {}", section_names().join(", "))
            };
            return Err(unknown_key(key).hint(hint));
        };
        let Some((section, _)) = SECTIONS.iter().find(|(s, _)| *s == section) else {
            return Err(
                unknown_key(key).hint(format!("expected one of: {}", section_names().join(", ")))
            );
        };
        resets.push(check_update_key(section, name)?);
    }
    Ok(resets)
}

/// The result of `update_config`: the keys whose value changed with the old value, or `# nothing changed`.
fn changes(old: &Settings, new: &Settings, resets: &[Mutable], set_keys: &[Mutable]) -> String {
    let mut out = String::new();
    let mut section = "";
    for key in Mutable::ALL {
        let (was, value) = (old.value(key), new.value(key));
        if was == value {
            continue;
        }
        let (key_section, name) = key.name();
        if key_section != section {
            if !out.is_empty() {
                out.push('\n');
            }
            let _ = writeln!(out, "[{key_section}]");
            section = key_section;
        }
        let reset = if resets.contains(&key) && !set_keys.contains(&key) {
            ", reset"
        } else {
            ""
        };
        let _ = writeln!(out, "{name} = {value}    # was {was}{reset}");
    }
    if out.is_empty() {
        out.push_str("# nothing changed\n");
    }
    out
}

/// The key which `update_config` changes, or the error: read-only, unknown, or not implemented yet.
fn check_update_key(section: &str, key: &str) -> Result<Mutable, Report> {
    let full = format!("{section}.{key}");
    if let Some((_, _, hint)) = READ_ONLY
        .iter()
        .find(|(s, k, _)| *s == section && *k == key)
    {
        return Err(Report::new(ErrorCode::ReadOnly, format!("'{full}' is read-only")).hint(*hint));
    }
    if let Some(mutable) = Mutable::find(section, key) {
        return Ok(mutable);
    }
    if is_spec_key(section, key) {
        return Err(Report::not_implemented(&format!("the key '{full}'")));
    }
    let error = unknown_key(&full);
    Err(match close_key(section, key) {
        Some(close) => error.hint(format!("did you mean '{section}.{close}'?")),
        None => error,
    })
}

/// `unknown_key` for `key`.
fn unknown_key(key: &str) -> Report {
    Report::new(ErrorCode::UnknownKey, format!("unknown key '{key}'"))
}

/// The arguments of a config tool as an object without unknown fields.
fn arguments_object<'a>(
    arguments: &'a Value,
    fields: &[&str],
) -> Result<std::borrow::Cow<'a, serde_json::Map<String, Value>>, Report> {
    let object = match arguments {
        Value::Null => std::borrow::Cow::Owned(serde_json::Map::new()),
        Value::Object(object) => std::borrow::Cow::Borrowed(object),
        _ => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "the arguments must be an object",
            ));
        }
    };
    if let Some(unknown) = object.keys().find(|k| !fields.contains(&k.as_str())) {
        return Err(unknown_name("field", unknown, fields));
    }
    Ok(object)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            vars.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| (*v).to_string())
        }
    }

    fn found(
        cli: Option<&str>,
        vars: &[(&str, &str)],
        os: &str,
        existing: &[&str],
    ) -> Option<PathBuf> {
        let exists = |p: &Path| existing.iter().any(|e| Path::new(e) == p);
        locate(cli.map(Path::new), &env(vars), os, &exists)
    }

    #[test]
    fn the_lookup_order() {
        let all = [
            ("EASY_PEASY_CONFIG", "env.toml"),
            ("XDG_CONFIG_HOME", "/xdg"),
            ("HOME", "/home/me"),
        ];
        let platform = "/xdg/easy-peasy-mcp/config.toml";
        assert_eq!(
            found(Some("cli.toml"), &all, "linux", &[platform]),
            Some(PathBuf::from("cli.toml"))
        );
        assert_eq!(
            found(None, &all, "linux", &[platform]),
            Some(PathBuf::from("env.toml"))
        );
        let no_env = [("EASY_PEASY_CONFIG", ""), ("XDG_CONFIG_HOME", "/xdg")];
        assert_eq!(
            found(None, &no_env, "linux", &[platform]),
            Some(PathBuf::from(platform))
        );
        assert_eq!(found(None, &no_env, "linux", &[]), None);
    }

    #[test]
    fn platform_directories() {
        let home = [("HOME", "/home/me")];
        assert_eq!(
            found(
                None,
                &home,
                "linux",
                &["/home/me/.config/easy-peasy-mcp/config.toml"]
            ),
            Some(PathBuf::from("/home/me/.config/easy-peasy-mcp/config.toml"))
        );
        // a relative XDG_CONFIG_HOME is ignored
        let relative = [("HOME", "/home/me"), ("XDG_CONFIG_HOME", "xdg")];
        assert_eq!(
            found(
                None,
                &relative,
                "linux",
                &["/home/me/.config/easy-peasy-mcp/config.toml"]
            ),
            Some(PathBuf::from("/home/me/.config/easy-peasy-mcp/config.toml"))
        );
        let mac = "/Users/me/Library/Application Support/easy-peasy-mcp/config.toml";
        assert_eq!(
            found(None, &[("HOME", "/Users/me")], "macos", &[mac]),
            Some(PathBuf::from(mac))
        );
        let windows = Path::new("C:/Users/me/AppData/Roaming")
            .join("easy-peasy-mcp")
            .join("config.toml");
        assert_eq!(
            found(
                None,
                &[("APPDATA", "C:/Users/me/AppData/Roaming")],
                "windows",
                &[windows.to_str().unwrap()]
            ),
            Some(windows.clone())
        );
        assert_eq!(found(None, &[], "linux", &[]), None);
    }

    fn error(text: &str) -> (usize, String) {
        check(text).unwrap_err()
    }

    #[test]
    fn valid_files() {
        assert_eq!(check(""), Ok(FileValues::default()));
        assert_eq!(
            check("# only a comment\n\n[output]\n[server]\n"),
            Ok(FileValues::default())
        );
        assert_eq!(
            check("[server]\n# roots\nallowed_roots = ['a', \"b\"]\n"),
            Ok(FileValues {
                allowed_roots: Some((vec!["a".into(), "b".into()], 3)),
                ..FileValues::default()
            })
        );
        assert_eq!(
            error("server.allowed_roots = [1]\n"),
            (
                1,
                "invalid value for 'server.allowed_roots': a list of paths is expected".into()
            )
        );
    }

    #[test]
    fn errors_with_lines() {
        assert_eq!(
            error("# x\n[output\n"),
            (2, "invalid TOML: unclosed table, expected `]`".into())
        );
        assert_eq!(
            error("\n\n[colours]\n"),
            (
                3,
                "unknown section 'colours', expected one of: server, traversal, output, escape, query, kinds, git, languages".into()
            )
        );
        assert_eq!(
            error("[output]\nmax_file = 1\n"),
            (
                2,
                "unknown key 'output.max_file', did you mean 'output.max_files'?".into()
            )
        );
        assert_eq!(
            error("[output]\nzzz = 1\n"),
            (2, "unknown key 'output.zzz'".into())
        );
        assert_eq!(
            error("[git]\nbase = 'main'\n"),
            (2, "the key 'git.base' is not implemented yet".into())
        );
        assert_eq!(
            error("x = 1\n[output]\nmax_files = 1\n"),
            (
                1,
                "unknown section 'x', expected one of: server, traversal, output, escape, query, kinds, git, languages".into()
            )
        );
        assert_eq!(
            error("output = 1\n"),
            (1, "'output' must be a section".into())
        );
        assert_eq!(
            error("output.max_files = 1\n"),
            (
                1,
                "the key 'output.max_files' is not implemented yet".into()
            )
        );
        assert_eq!(
            error("output = { max_files = 1 }\n"),
            (
                1,
                "the key 'output.max_files' is not implemented yet".into()
            )
        );
        assert_eq!(
            error("[languages.terraform]\nextensions = ['tf']\n"),
            (
                1,
                "the key 'languages.terraform' is not implemented yet".into()
            )
        );
        // the first error in the file wins, whatever the section order of the spec
        assert_eq!(
            error("[server]\n[output]\nzzz = 1\n[server2]\n"),
            (3, "unknown key 'output.zzz'".into())
        );
    }

    #[test]
    fn get_config_arguments() {
        let settings = Settings {
            allowed_roots: vec![PathBuf::from("/a"), PathBuf::from("/it's")],
            allowed_roots_source: Source::Cli,
            ..Settings::defaults(Path::new("/"))
        };
        let first = format!(
            "# easy-peasy-mcp {}, spec 1, supported specs: 1\n# read-only: server.allowed_roots\n",
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(
            get_config(&settings, &Value::Null).unwrap(),
            format!(
                "{first}\n[server]\nallowed_roots = ['/a', \"/it's\"]\ndefault_dirs = ['.']\ntimeout = \"30s\"\nthreads = 0\n"
            )
        );
        assert_eq!(
            get_config(
                &settings,
                &serde_json::json!({ "sources": true, "section": "server" })
            )
            .unwrap(),
            format!(
                "{first}\n[server]\nallowed_roots = ['/a', \"/it's\"]    # cli\ndefault_dirs = ['.']    # default\ntimeout = \"30s\"    # default\nthreads = 0    # default\n"
            )
        );
        assert_eq!(
            get_config(&settings, &serde_json::json!({ "section": "git" })).unwrap(),
            first
        );
        let error = |v: Value| get_config(&settings, &v).unwrap_err().to_text();
        assert_eq!(
            error(serde_json::json!({ "section": 1 })),
            "error[invalid_option]: 'section' must be a string\n"
        );
        assert_eq!(
            error(serde_json::json!({ "section": "zzz" })),
            "error[invalid_option]: unknown section 'zzz'\n  hint: expected one of: server, traversal, output, escape, query, kinds, git, languages\n"
        );
        assert_eq!(
            error(serde_json::json!([])),
            "error[invalid_option]: the arguments must be an object\n"
        );
    }

    fn update(arguments: &Value) -> String {
        let base = Settings::defaults(Path::new("/"));
        let mut session = base.clone();
        match update_config(&base, &mut session, arguments) {
            Ok(text) => text,
            Err(report) => report.to_text(),
        }
    }

    #[test]
    fn update_config_changes_nothing_yet() {
        use serde_json::json;
        assert_eq!(update(&json!({})), "# nothing changed\n");
        assert_eq!(
            update(&json!({ "reset": ["all"], "set": "" })),
            "# nothing changed\n"
        );
        assert_eq!(
            update(&json!({ "set": "server.default_dirs = ['.']" })),
            "# nothing changed\n"
        );
        assert_eq!(
            update(&json!({ "set": "output.context = 2" })),
            "error[not_implemented]: the key 'output.context' is not implemented yet\n"
        );
        assert_eq!(
            update(&json!({ "reset": ["languages.rust"] })),
            "error[not_implemented]: the key 'languages.rust' is not implemented yet\n"
        );
    }

    #[test]
    fn update_config_changes_the_session() {
        use serde_json::json;
        let base = Settings {
            default_dirs: vec!["src".into()],
            default_dirs_source: Source::File("c.toml".into()),
            ..Settings::defaults(Path::new("/"))
        };
        let mut session = base.clone();
        let mut call = |arguments: Value| match update_config(&base, &mut session, &arguments) {
            Ok(text) => text,
            Err(report) => report.to_text(),
        };
        assert_eq!(
            call(json!({ "set": "[server]\ndefault_dirs = ['a', \"it's\"]" })),
            "[server]\ndefault_dirs = ['a', \"it's\"]    # was ['src']\n"
        );
        // atomic: an invalid value changes nothing
        assert_eq!(
            call(json!({ "set": "server.default_dirs = ['b']\nserver.zzz = 1" })),
            "error[unknown_key]: unknown key 'server.zzz'\n"
        );
        assert_eq!(
            call(json!({ "set": "server.default_dirs = [1]" })),
            "error[invalid_value]: invalid value for 'server.default_dirs'\n  hint: a non-empty list of paths is expected\n"
        );
        // set wins over reset
        assert_eq!(
            call(json!({ "reset": ["all"], "set": "server.default_dirs = ['b']" })),
            "[server]\ndefault_dirs = ['b']    # was ['a', \"it's\"]\n"
        );
        assert_eq!(
            call(json!({ "reset": ["server.default_dirs"] })),
            "[server]\ndefault_dirs = ['src']    # was ['b'], reset\n"
        );
        assert_eq!(session, base);
    }

    #[test]
    fn update_config_errors() {
        use serde_json::json;
        assert_eq!(
            update(&json!({ "set": "# x\n  a = \"é\" b" })),
            "error[invalid_toml]: invalid TOML in 'set'\n  hint: line 2, col 11: unexpected key or value, expected newline, `#`\n"
        );
        assert_eq!(
            update(&json!({ "set": "[server]\nallow_update = false" })),
            "error[read_only]: 'server.allow_update' is read-only\n  hint: allow_update is set only at start\n"
        );
        assert_eq!(
            update(&json!({ "set": "output = 1" })),
            "error[unknown_key]: 'output' must be a section\n"
        );
        assert_eq!(
            update(&json!({ "reset": ["output"] })),
            "error[unknown_key]: unknown key 'output'\n  hint: expected a key like 'output.max_files', or 'all'\n"
        );
        assert_eq!(
            update(&json!({ "reset": ["zzz.a"] })),
            "error[unknown_key]: unknown key 'zzz.a'\n  hint: expected one of: server, traversal, output, escape, query, kinds, git, languages\n"
        );
        // the first key of the TOML
        assert_eq!(
            update(&json!({ "set": "output.zzz = 1\nserver.allowed_roots = []" })),
            "error[unknown_key]: unknown key 'output.zzz'\n"
        );
        assert_eq!(
            update(&json!({ "reset": [1] })),
            "error[invalid_option]: 'reset' must be a list of strings\n"
        );
    }

    #[test]
    fn roots_are_resolved() {
        let tmp = std::env::temp_dir().join(format!("ep-roots-{}", std::process::id()));
        std::fs::create_dir_all(tmp.join("home/proj")).unwrap();
        let start = std::fs::canonicalize(&tmp).unwrap();
        let home = start.join("home");
        let roots = |cli: &[&str]| {
            let cli: Vec<String> = cli.iter().map(ToString::to_string).collect();
            let cli = CommandLine {
                roots: cli,
                timeout: None,
            };
            Settings::resolve(None, &cli, &start, Some(&home))
        };
        assert_eq!(roots(&[]).unwrap(), Settings::defaults(&start));
        assert_eq!(
            roots(&[]).unwrap().allowed_roots,
            std::slice::from_ref(&start)
        );
        let settings = roots(&["~/proj", "home", "~"]).unwrap();
        assert_eq!(
            settings.allowed_roots,
            [home.join("proj"), home.clone(), home.clone()]
        );
        assert_eq!(settings.allowed_roots_source, Source::Cli);
        assert!(settings.is_allowed(&home.join("proj/x")));
        assert!(!settings.is_allowed(&start));
        assert_eq!(
            roots(&["missing"]).unwrap_err().to_string(),
            "the allowed root 'missing' doesn't exist"
        );
        let file = ConfigFile {
            path: PathBuf::from("c.toml"),
            values: FileValues {
                allowed_roots: Some((vec!["missing".into()], 4)),
                ..FileValues::default()
            },
        };
        assert_eq!(
            Settings::resolve(Some(&file), &CommandLine::default(), &start, None)
                .unwrap_err()
                .to_string(),
            "c.toml:4: the allowed root 'missing' doesn't exist"
        );
        let empty = ConfigFile {
            path: PathBuf::from("c.toml"),
            values: FileValues {
                allowed_roots: Some((Vec::new(), 1)),
                default_dirs: Some(vec!["src".into()]),
                timeout: Some("1m".into()),
                threads: Some(2),
            },
        };
        let cli = CommandLine {
            roots: Vec::new(),
            timeout: Some("5s".into()),
        };
        let settings = Settings::resolve(Some(&empty), &cli, &start, None).unwrap();
        assert_eq!(settings.timeout, "5s");
        assert_eq!(settings.timeout_source, Source::Cli);
        assert_eq!(settings.threads, 2);
        assert_eq!(settings.timeout(), Some(std::time::Duration::from_secs(5)));
        assert_eq!(settings.allowed_roots, std::slice::from_ref(&start));
        assert_eq!(settings.allowed_roots_source, Source::File("c.toml".into()));
        assert_eq!(settings.default_dirs, ["src"]);
        assert_eq!(settings.default_dirs_source, Source::File("c.toml".into()));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn durations() {
        use std::time::Duration;
        assert_eq!(parse_duration("500ms"), Some(Duration::from_millis(500)));
        assert_eq!(parse_duration("30s"), Some(Duration::from_secs(30)));
        assert_eq!(parse_duration("2m"), Some(Duration::from_secs(120)));
        assert_eq!(parse_duration("0s"), Some(Duration::ZERO));
        for bad in ["", "10", "s", "1h", "1.5s", "-1s", " 1s", "1 s"] {
            assert_eq!(parse_duration(bad), None, "{bad}");
        }
        let settings = Settings {
            timeout: "0m".into(),
            ..Settings::defaults(Path::new("/"))
        };
        assert_eq!(settings.timeout(), None);
    }

    #[test]
    fn files_which_cant_be_read() {
        let e = load(Path::new("/nonexistent/easy-peasy.toml")).unwrap_err();
        assert_eq!(
            e.to_string(),
            "/nonexistent/easy-peasy.toml: the config file doesn't exist"
        );
    }
}
