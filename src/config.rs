//! The config file: where it is, and its checks at start (CONFIG.md#config-file).

use std::fmt;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::SPEC_VERSION;
use crate::error::{ErrorCode, Report};
use crate::query::did_you_mean;

/// An error which stops the server at start: `<path>:<line>: <message>`, or `<path>: <message>` without a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartError {
    pub path: PathBuf,
    /// 1-based.
    pub line: Option<usize>,
    pub message: String,
}

impl fmt::Display for StartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{line}: {}", self.path.display(), self.message),
            None => write!(f, "{}: {}", self.path.display(), self.message),
        }
    }
}

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
pub fn load(path: &Path) -> Result<(), StartError> {
    let error = |line, message: String| StartError {
        path: path.to_path_buf(),
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
    check(&text).map_err(|(line, message)| error(Some(line), message))
}

/// Checks the text of a config file: the 1-based line and the message of the first error.
fn check(text: &str) -> Result<(), (usize, String)> {
    let line = |offset: usize| text[..offset.min(text.len())].matches('\n').count() + 1;
    let doc = toml_edit::Document::parse(text).map_err(|e| {
        let offset = e.span().map_or(0, |s| s.start);
        (line(offset), format!("invalid TOML: {}", e.message()))
    })?;
    let root = doc.as_table();
    let mut errors: Vec<(usize, String)> = Vec::new();
    let offset = |key: Option<&toml_edit::Key>, item: &toml_edit::Item| {
        key.and_then(toml_edit::Key::span)
            .or_else(|| item.span())
            .map_or(0, |s| s.start)
    };
    for (name, item) in root {
        let at = offset(root.key(name), item);
        let Some((_, keys)) = SECTIONS.iter().find(|(section, _)| *section == name) else {
            let names: Vec<&str> = SECTIONS.iter().map(|(section, _)| *section).collect();
            errors.push((
                at,
                format!(
                    "unknown section '{name}', expected one of: {}",
                    names.join(", ")
                ),
            ));
            continue;
        };
        let Some(table) = item.as_table_like() else {
            errors.push((at, format!("'{name}' must be a section")));
            continue;
        };
        for (key, value) in table.iter() {
            let at = offset(table.get_key_value(key).map(|(k, _)| k), value);
            let full = format!("{name}.{key}");
            let message = if name == "languages" || keys.contains(&key) {
                format!("the key '{full}' is not implemented yet")
            } else {
                match did_you_mean(key, keys) {
                    Some(close) => format!("unknown key '{full}', did you mean '{name}.{close}'?"),
                    None => format!("unknown key '{full}'"),
                }
            };
            errors.push((at, message));
        }
    }
    match errors.into_iter().min_by_key(|(at, _)| *at) {
        Some((at, message)) => Err((line(at), message)),
        None => Ok(()),
    }
}

/// The fields of `get_config`, in the order of its input schema.
const GET_CONFIG_FIELDS: [&str; 2] = ["section", "sources"];

/// The `get_config` tool (`CONFIG.md#get_config`): the effective configuration as TOML. The first line names the server
/// and the spec versions; the read-only keys and the sections with their keys follow, as far as they are
/// implemented — no key is implemented yet, so `section` and `sources` don't change the result.
pub(crate) fn get_config(arguments: &Value) -> Result<String, Report> {
    let empty = serde_json::Map::new();
    let object = match arguments {
        Value::Null => &empty,
        Value::Object(object) => object,
        _ => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "the arguments must be an object",
            ));
        }
    };
    if let Some(unknown) = object
        .keys()
        .find(|k| !GET_CONFIG_FIELDS.contains(&k.as_str()))
    {
        return Err(unknown_name("field", unknown, &GET_CONFIG_FIELDS));
    }
    match object.get("section") {
        None => {}
        Some(Value::String(section)) => {
            let names: Vec<&str> = SECTIONS.iter().map(|(name, _)| *name).collect();
            if !names.contains(&section.as_str()) {
                return Err(unknown_name("section", section, &names));
            }
        }
        Some(_) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'section' must be a string",
            ));
        }
    }
    if object.get("sources").is_some_and(|v| !v.is_boolean()) {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'sources' must be a boolean",
        ));
    }
    Ok(format!(
        "# easy-peasy-mcp {}, spec {SPEC_VERSION}, supported specs: {SPEC_VERSION}\n",
        env!("CARGO_PKG_VERSION")
    ))
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

/// The `update_config` tool (`CONFIG.md#update_config`).
pub(crate) fn update_config(_arguments: &Value) -> Result<String, Report> {
    Ok(String::new())
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
        assert_eq!(check(""), Ok(()));
        assert_eq!(check("# only a comment\n\n[output]\n[server]\n"), Ok(()));
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
        let first = format!(
            "# easy-peasy-mcp {}, spec 1, supported specs: 1\n",
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(get_config(&Value::Null).unwrap(), first);
        let error = |v: Value| get_config(&v).unwrap_err().to_text();
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

    fn update(arguments: Value) -> String {
        match update_config(&arguments) {
            Ok(text) => text,
            Err(report) => report.to_text(),
        }
    }

    #[test]
    fn update_config_changes_nothing_yet() {
        use serde_json::json;
        assert_eq!(update(json!({})), "# nothing changed\n");
        assert_eq!(
            update(json!({ "reset": ["all"], "set": "" })),
            "# nothing changed\n"
        );
        assert_eq!(
            update(json!({ "set": "output.context = 2" })),
            "error[not_implemented]: the key 'output.context' is not implemented yet\n"
        );
        assert_eq!(
            update(json!({ "reset": ["languages.rust"] })),
            "error[not_implemented]: the key 'languages.rust' is not implemented yet\n"
        );
    }

    #[test]
    fn update_config_errors() {
        use serde_json::json;
        assert_eq!(
            update(json!({ "set": "# x\n  a = \"é\" b" })),
            "error[invalid_toml]: invalid TOML in 'set'\n  hint: line 2, col 11: unexpected content, expected nothing\n"
        );
        assert_eq!(
            update(json!({ "set": "[server]\nallow_update = false" })),
            "error[read_only]: 'server.allow_update' is read-only\n  hint: allow_update is set only at start\n"
        );
        assert_eq!(
            update(json!({ "set": "output = 1" })),
            "error[unknown_key]: 'output' must be a section\n"
        );
        assert_eq!(
            update(json!({ "reset": ["output"] })),
            "error[unknown_key]: unknown key 'output'\n  hint: expected a key like 'output.max_files', or 'all'\n"
        );
        assert_eq!(
            update(json!({ "reset": ["zzz.a"] })),
            "error[unknown_key]: unknown key 'zzz.a'\n  hint: expected one of: server, traversal, output, escape, query, kinds, git, languages\n"
        );
        // the first key of the TOML
        assert_eq!(
            update(json!({ "set": "output.zzz = 1\nserver.allowed_roots = []" })),
            "error[unknown_key]: unknown key 'output.zzz'\n"
        );
        assert_eq!(
            update(json!({ "reset": [1] })),
            "error[invalid_option]: 'reset' must be a list of strings\n"
        );
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
