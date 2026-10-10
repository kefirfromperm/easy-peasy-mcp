//! The text format of the response (OUTPUT.md#text-format).

use std::fmt::Write as _;
use std::path::{Component, Path};

use crate::query::ast::CondId;
use crate::search::Found;
use crate::warning::Warnings;

mod lines;

pub use lines::{Fragment, matched_lines, merge_identical};

/// The response of a `find` call with `detail = lines`, for a query of file conditions only.
pub struct Response<'a> {
    pub result_id: &'a str,
    pub legend: &'a [(CondId, String)],
    pub files: &'a [Found],
    /// Printed after the files.
    pub warnings: &'a Warnings,
    /// The search was stopped by the timeout: `truncated: yes (timeout)`.
    pub timed_out: bool,
}

/// The separator of output paths on this platform.
const SEP: char = std::path::MAIN_SEPARATOR;

impl Response<'_> {
    /// The text format.
    #[must_use]
    pub fn to_text(&self) -> String {
        let count = |f: fn(&crate::search::FileMatches) -> usize| -> usize {
            self.files
                .iter()
                .filter_map(|file| file.matches.as_ref())
                .map(f)
                .sum()
        };
        let mut out = format!(
            "files: {}  lines: {}  matches: {}  result: {}\n",
            self.files.len(),
            count(|m| m.lines),
            count(|m| m.fragments.len()),
            self.result_id
        );
        if self.timed_out {
            out.insert_str(out.len() - 1, "  truncated: yes (timeout)");
        }
        if self.files.is_empty() {
            self.push_warnings(&mut out);
            return out;
        }
        for (id, text) in self.legend {
            let _ = writeln!(out, "{id} {text}");
        }
        out.push('\n');
        let mut current: Option<String> = None;
        for file in self.files {
            let (dir, name) = split(&file.rel);
            if current.as_deref() != Some(dir.as_str()) {
                out.push_str(&dir);
                out.push('\n');
                current = Some(dir);
            }
            out.push_str(&name);
            if file.is_dir {
                out.push(SEP);
            }
            if !file.conditions.is_empty() {
                let ids: Vec<String> = file.conditions.iter().map(ToString::to_string).collect();
                let _ = write!(out, "  [{}]", ids.join(" "));
            }
            if let Some(m) = &file.matches {
                let _ = write!(out, "  lines: {}  matches: {}", m.lines, m.fragments.len());
            }
            if let Some(encoding) = file.encoding {
                let _ = write!(out, "  {}", encoding.name());
            }
            out.push('\n');
            if let Some(m) = &file.matches {
                out.push_str(&lines::render(&m.content, &m.fragments));
            }
        }
        self.push_warnings(&mut out);
        out
    }

    fn push_warnings(&self, out: &mut String) {
        if !self.warnings.is_empty() {
            out.push('\n');
            out.push_str(&self.warnings.to_text());
        }
    }
}

/// The parts of a path to join with the separator: `..` kept, `.` dropped, an empty first part for the root of an
/// absolute path.
fn path_parts(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            Component::ParentDir => Some("..".to_string()),
            Component::CurDir => None,
            Component::RootDir => Some(String::new()),
            Component::Prefix(p) => Some(p.as_os_str().to_string_lossy().into_owned()),
        })
        .collect()
}

/// A path in the native form, a directory with a trailing separator.
pub fn display_path(rel: &Path, is_dir: bool) -> String {
    let parts = path_parts(rel);
    let mut out = parts.join(&SEP.to_string());
    if is_dir {
        out.push(SEP);
    }
    out
}

/// The directory header (ending with the separator; `./` for the working directory) and the file name.
fn split(rel: &Path) -> (String, String) {
    let parts = path_parts(rel);
    let (name, dirs) = parts
        .split_last()
        .map_or((String::new(), &[][..]), |(n, d)| (n.clone(), d));
    let dir = if dirs.is_empty() {
        format!(".{SEP}")
    } else {
        let mut d = dirs.join(&SEP.to_string());
        d.push(SEP);
        d
    };
    (dir, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn found(path: &str) -> Found {
        Found {
            rel: PathBuf::from(path),
            is_dir: false,
            conditions: vec![CondId::Number(1)],
            matches: None,
            encoding: None,
            id: None,
        }
    }

    #[test]
    fn groups_files_under_directory_headers() {
        let files = [
            found("Cargo.toml"),
            found("src/a.rs"),
            found("src/b.rs"),
            found("src/x/c.rs"),
        ];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let text = Response {
            result_id: "r3",
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert_eq!(
            text,
            format!(
                "files: 4  lines: 0  matches: 0  result: r3\n#1 name ~ '*'\n\n.{s}\nCargo.toml  [#1]\nsrc{s}\na.rs  [#1]\nb.rs  [#1]\nsrc{s}x{s}\nc.rs  [#1]\n"
            )
        );
    }

    #[test]
    fn files_without_conditions_have_no_brackets() {
        let files = [Found {
            rel: PathBuf::from("a.md"),
            is_dir: false,
            conditions: vec![],
            matches: None,
            encoding: None,
            id: None,
        }];
        let text = Response {
            result_id: "r1",
            legend: &[],
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert_eq!(
            text,
            format!("files: 1  lines: 0  matches: 0  result: r1\n\n.{s}\na.md\n")
        );
    }

    #[test]
    fn directories_end_with_the_separator() {
        let mut dir = found("lib/sub");
        dir.is_dir = true;
        let files = [dir];
        let legend = [(CondId::Number(1), "type = dir".to_string())];
        let text = Response {
            result_id: "r1",
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert!(
            text.ends_with(&format!("\n\nlib{s}\nsub{s}  [#1]\n")),
            "{text}"
        );
    }

    #[test]
    fn an_empty_result_has_only_the_header() {
        let text = Response {
            result_id: "r1",
            legend: &[(CondId::Number(1), "x".into())],
            files: &[],
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        assert_eq!(text, "files: 0  lines: 0  matches: 0  result: r1\n");
    }
}
