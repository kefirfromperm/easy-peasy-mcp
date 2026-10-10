//! The config file: where it is, and its checks at start (CONFIG.md#config-file).

use std::fmt;
use std::path::{Path, PathBuf};

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
                match crate::query::did_you_mean(key, keys) {
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
    fn files_which_cant_be_read() {
        let e = load(Path::new("/nonexistent/easy-peasy.toml")).unwrap_err();
        assert_eq!(
            e.to_string(),
            "/nonexistent/easy-peasy.toml: the config file doesn't exist"
        );
    }
}
