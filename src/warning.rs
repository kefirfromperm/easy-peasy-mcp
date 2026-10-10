//! Problems which don't make a request fail (OUTPUT.md#warnings).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::output::display_path;

/// The warning codes, in the order of the table of OUTPUT.md#warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningCode {
    PermissionDenied,
    TooBig,
    BinarySkipped,
    DecodeErrors,
    ArchiveError,
    DateUnsupported,
    Timeout,
    NotAGitRepo,
}

impl WarningCode {
    /// The code as written in the response.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PermissionDenied => "permission_denied",
            Self::TooBig => "too_big",
            Self::BinarySkipped => "binary_skipped",
            Self::DecodeErrors => "decode_errors",
            Self::ArchiveError => "archive_error",
            Self::DateUnsupported => "date_unsupported",
            Self::Timeout => "timeout",
            Self::NotAGitRepo => "not_a_git_repo",
        }
    }
}

/// At most this many example paths per warning.
const MAX_PATHS: usize = 3;

/// The warnings of one response: entries grouped by code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Warnings {
    /// Per code: the path relative to the working directory, the member path inside an archive, whether it is a
    /// directory.
    entries: BTreeMap<WarningCode, Vec<(PathBuf, Option<String>, bool)>>,
}

impl Warnings {
    /// Adds an entry (a path relative to the working directory, whether it is a directory) with a warning.
    pub fn add(&mut self, code: WarningCode, rel: PathBuf, is_dir: bool) {
        self.entries
            .entry(code)
            .or_default()
            .push((rel, None, is_dir));
    }

    /// Adds a member of an archive (the archive path relative to the working directory, the member path inside it)
    /// with a warning.
    pub fn add_member(&mut self, code: WarningCode, rel: PathBuf, member: String) {
        self.entries
            .entry(code)
            .or_default()
            .push((rel, Some(member), false));
    }

    /// Adds a warning which is not about entries.
    pub fn add_global(&mut self, code: WarningCode) {
        self.entries.entry(code).or_default();
    }

    /// Adds the warnings of another working directory, their paths prefixed with it.
    pub fn extend(&mut self, other: Warnings, prefix: &std::path::Path) {
        for (code, entries) in other.entries {
            self.entries.entry(code).or_default().extend(
                entries
                    .into_iter()
                    .map(|(rel, member, is_dir)| (prefix.join(rel), member, is_dir)),
            );
        }
    }

    /// Whether there is no warning.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// One line per code: `warning: <code> (<count>): <paths>, … N more`.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for (code, entries) in &self.entries {
            if entries.is_empty() {
                let _ = writeln!(out, "warning: {}", code.as_str());
                continue;
            }
            let mut entries = entries.clone();
            entries.sort();
            entries.dedup();
            let shown: Vec<String> = entries
                .iter()
                .take(MAX_PATHS)
                .map(|(rel, member, is_dir)| match member {
                    Some(member) => format!("{}!/{member}", display_path(rel, false)),
                    None => display_path(rel, *is_dir),
                })
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

    #[test]
    fn warnings_without_paths() {
        let mut w = Warnings::default();
        w.add(WarningCode::NotAGitRepo, PathBuf::from("a"), false);
        w.add_global(WarningCode::Timeout);
        assert_eq!(
            w.to_text(),
            "warning: timeout\nwarning: not_a_git_repo (1): a\n"
        );
    }

    #[test]
    fn too_big_comes_after_permission_denied() {
        let mut w = Warnings::default();
        w.add(WarningCode::BinarySkipped, PathBuf::from("b"), false);
        w.add(WarningCode::TooBig, PathBuf::from("t"), false);
        w.add(WarningCode::PermissionDenied, PathBuf::from("p"), false);
        assert_eq!(
            w.to_text(),
            "warning: permission_denied (1): p\nwarning: too_big (1): t\nwarning: binary_skipped (1): b\n"
        );
    }

    #[test]
    fn permission_denied_comes_first() {
        let mut w = Warnings::default();
        w.add(WarningCode::BinarySkipped, PathBuf::from("b"), false);
        w.add(WarningCode::PermissionDenied, PathBuf::from("secret"), true);
        assert_eq!(
            w.to_text(),
            format!(
                "warning: permission_denied (1): secret{}\nwarning: binary_skipped (1): b\n",
                sep()
            )
        );
    }
}
