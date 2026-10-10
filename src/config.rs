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

/// The config file to read: `--config`, a non-empty `$EASY_PEASY_CONFIG`, or the file of the platform config
/// directory when it exists; `None` for the built-in defaults. `env` looks up an environment variable, `os` is
/// `std::env::consts::OS`.
pub fn locate(
    _cli: Option<&Path>,
    _env: &dyn Fn(&str) -> Option<String>,
    _os: &str,
    _exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    None
}

/// Reads and checks the config file at `path`.
pub fn load(_path: &Path) -> Result<(), StartError> {
    Ok(())
}

/// Checks the text of a config file: the 1-based line and the message of the first error.
fn check(_text: &str) -> Result<(), (usize, String)> {
    Ok(())
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
