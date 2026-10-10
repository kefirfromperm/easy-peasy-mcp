//! Problems which don't make a request fail (OUTPUT.md#warnings).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::output::display_path;

/// The warning codes, in the order of the table of OUTPUT.md#warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningCode {
    DateUnsupported,
    NotAGitRepo,
}

impl WarningCode {
    /// The code as written in the response.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DateUnsupported => "date_unsupported",
            Self::NotAGitRepo => "not_a_git_repo",
        }
    }
}

/// At most this many example paths per warning.
const MAX_PATHS: usize = 3;

/// The warnings of one response: entries grouped by code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Warnings {
    entries: BTreeMap<WarningCode, Vec<(PathBuf, bool)>>,
}

impl Warnings {
    /// Adds an entry (a path relative to the working directory, whether it is a directory) with a warning.
    pub fn add(&mut self, code: WarningCode, rel: PathBuf, is_dir: bool) {
        self.entries.entry(code).or_default().push((rel, is_dir));
    }

    /// Whether there is no warning.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// One line per code: `warning: <code> (<count>): <paths>, … N more`.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for (code, entries) in &self.entries {
            let mut entries = entries.clone();
            entries.sort();
            entries.dedup();
            let shown: Vec<String> = entries
                .iter()
                .take(MAX_PATHS)
                .map(|(rel, is_dir)| display_path(rel, *is_dir))
                .collect();
            let _ = write!(
                out,
                "warning: {} ({}): {}",
                code.as_str(),
                entries.len(),
                shown.join(", ")
            );
            if entries.len() > MAX_PATHS {
                let _ = write!(out, ", … {} more", entries.len() - MAX_PATHS);
            }
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sep() -> char {
        std::path::MAIN_SEPARATOR
    }

    #[test]
    fn lines_show_three_paths_and_the_rest() {
        let mut w = Warnings::default();
        for name in ["e", "b", "a", "d", "c"] {
            w.add(WarningCode::DateUnsupported, PathBuf::from(name), false);
        }
        w.add(WarningCode::DateUnsupported, PathBuf::from("dir"), true);
        assert_eq!(
            w.to_text(),
            "warning: date_unsupported (6): a, b, c, … 3 more\n"
        );
        let mut order = Warnings::default();
        order.add(WarningCode::NotAGitRepo, PathBuf::from("b"), false);
        order.add(WarningCode::BinarySkipped, PathBuf::from("a"), false);
        assert!(
            order
                .to_text()
                .starts_with("warning: binary_skipped (1): a\n")
        );
        let mut one = Warnings::default();
        one.add(WarningCode::DateUnsupported, PathBuf::from("dir"), true);
        assert_eq!(
            one.to_text(),
            format!("warning: date_unsupported (1): dir{}\n", sep())
        );
        assert!(Warnings::default().is_empty());
    }
}
