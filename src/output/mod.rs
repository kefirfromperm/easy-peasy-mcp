//! The response of `find` in the text format (OUTPUT.md#text-format) and in JSON (`json`).

use std::fmt::Write as _;
use std::path::{Component, Path};

use crate::query::ast::CondId;
use crate::search::Found;
use crate::warning::Warnings;

mod blocks;
mod json;
mod lines;
mod repeated;

pub use blocks::BlockOptions;

pub use lines::{Fragment, matched_lines, merge_identical};

/// How much a response returns (OUTPUT.md#detail-levels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Detail {
    /// Only whether at least one file matches.
    Exists,
    /// The totals.
    Count,
    /// The totals, the legend and the files.
    Files,
    /// The totals, the legend, the files and their matched lines.
    Lines,
}

/// The response of a `find` call.
pub struct Response<'a> {
    /// How much it returns.
    pub detail: Detail,
    /// How the matched lines of a file are put into blocks.
    pub blocks: BlockOptions,
    /// Text: files under directory headers, with paths relative to them.
    pub group_dirs: bool,
    /// A matched line in this many files is printed once (`lines` detail); `0`: off.
    pub group_repeated: usize,
    /// `None` when the result cache is off.
    pub result_id: Option<&'a str>,
    pub legend: &'a [(CondId, String)],
    pub files: &'a [Found],
    /// Printed after the files.
    pub warnings: &'a Warnings,
    /// The search was stopped by the timeout: `truncated: yes (timeout)`.
    pub timed_out: bool,
}

/// The separator of output paths on this platform.
const SEP: char = std::path::MAIN_SEPARATOR;

/// The locations printed for a repeated line.
const LOCATIONS: usize = 3;

impl Response<'_> {
    /// The matched lines and the fragments of all files.
    fn totals(&self) -> (usize, usize) {
        self.files
            .iter()
            .filter_map(|file| file.matches.as_ref())
            .fold((0, 0), |(lines, matches), m| {
                (lines + m.lines, matches + m.fragments.len())
            })
    }

    /// The text format.
    #[must_use]
    pub fn to_text(&self) -> String {
        let timeout = if self.timed_out {
            "  truncated: yes (timeout)"
        } else {
            ""
        };
        if self.detail == Detail::Exists {
            let exists = if self.files.is_empty() { "no" } else { "yes" };
            let mut out = format!("exists: {exists}{timeout}\n");
            self.push_warnings(&mut out);
            return out;
        }
        let (lines, matches) = self.totals();
        let mut out = format!(
            "files: {}  lines: {lines}  matches: {matches}",
            self.files.len(),
        );
        if let Some(id) = self.result_id {
            let _ = write!(out, "  result: {id}");
        }
        out.push_str(timeout);
        out.push('\n');
        if self.files.is_empty() || self.detail == Detail::Count {
            self.push_warnings(&mut out);
            return out;
        }
        for (id, text) in self.legend {
            let _ = writeln!(out, "{id} {text}");
        }
        out.push('\n');
        let repeated = self.repeated();
        self.push_repeated(&mut out, &repeated);
        let mut current: Option<String> = None;
        for (index, file) in self.files.iter().enumerate() {
            let rest = self.rest(&repeated, index);
            if rest.as_ref().is_some_and(Vec::is_empty) {
                continue;
            }
            if self.group_dirs {
                let (dir, name) = split_found(file);
                if current.as_deref() != Some(dir.as_str()) {
                    out.push_str(&dir);
                    out.push('\n');
                    current = Some(dir);
                }
                out.push_str(&name);
                if file.is_dir {
                    out.push(SEP);
                }
            } else {
                out.push_str(&display_found(file));
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
            if let (Some(m), Some(fragments)) = (&file.matches, rest) {
                let blocks = blocks::blocks(&m.content, &fragments, &self.blocks);
                out.push_str(&lines::render(&m.content, &fragments, &blocks));
            }
        }
        self.push_warnings(&mut out);
        out
    }

    /// The repeated lines of the files, only in `lines` detail.
    fn repeated(&self) -> repeated::Repeated {
        if self.detail == Detail::Lines {
            repeated::repeated(self.files, self.group_repeated)
        } else {
            repeated::Repeated::default()
        }
    }

    /// In `lines` detail the fragments of the file `index` to print, those which are not on its repeated lines: empty
    /// if it has fragments only there and is left out; `None` for a file without fragments and in other detail levels.
    fn rest(&self, repeated: &repeated::Repeated, index: usize) -> Option<Vec<Fragment>> {
        let m = self.files[index]
            .matches
            .as_ref()
            .filter(|m| !m.fragments.is_empty())?;
        (self.detail == Detail::Lines).then(|| repeated.rest(index, &m.content, &m.fragments))
    }

    /// The `repeated:` section: `  ×N <line>    <locations>` (OUTPUT.md#repeated-lines).
    fn push_repeated(&self, out: &mut String, repeated: &repeated::Repeated) {
        if repeated.lines.is_empty() {
            return;
        }
        out.push_str("repeated:\n");
        for line in &repeated.lines {
            let _ = writeln!(
                out,
                "  ×{} {}    {}",
                line.files.len(),
                lines::marked_line(&line.content, &line.fragments, 0, 0),
                self.locations(line).join(", ")
            );
        }
    }

    /// The first three locations of a repeated line, `path:line`, and `… M more`.
    fn locations(&self, line: &repeated::RepeatedLine) -> Vec<String> {
        let mut out: Vec<String> = line
            .files
            .iter()
            .take(LOCATIONS)
            .map(|&(file, i)| format!("{}:{}", display_found(&self.files[file]), i + 1))
            .collect();
        if line.files.len() > LOCATIONS {
            out.push(format!("… {} more", line.files.len() - LOCATIONS));
        }
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

/// The full path of a matched entry: a directory with the separator, a member of an archive after `!/`.
pub fn display_found(file: &Found) -> String {
    match &file.member {
        Some(member) => format!("{}!/{member}", display_path(&file.rel, false)),
        None => display_path(&file.rel, file.is_dir),
    }
}

/// The directory header and the name of a matched entry; a member of an archive is under `<archive>!/<member dir>/`,
/// the part after `!/` with `/`.
fn split_found(file: &Found) -> (String, String) {
    let Some(member) = &file.member else {
        return split(&file.rel);
    };
    let archive = display_path(&file.rel, false);
    match member.rsplit_once('/') {
        Some((dir, name)) => (format!("{archive}!/{dir}/"), name.to_string()),
        None => (format!("{archive}!/"), member.clone()),
    }
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
            member: None,
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
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            result_id: Some("r3"),
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
    fn full_paths_without_groups() {
        let files = [found("Cargo.toml"), found("src/a.rs")];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: false,
            group_repeated: 3,
            result_id: None,
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
                "files: 2  lines: 0  matches: 0\n#1 name ~ '*'\n\nCargo.toml  [#1]\nsrc{s}a.rs  [#1]\n"
            )
        );
    }

    #[test]
    fn files_without_conditions_have_no_brackets() {
        let files = [Found {
            rel: PathBuf::from("a.md"),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: None,
            encoding: None,
            id: None,
        }];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            result_id: Some("r1"),
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
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            result_id: Some("r1"),
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
    fn repeated_lines_show_three_locations() {
        let files: Vec<Found> = ["a", "b", "c", "d"]
            .iter()
            .map(|name| {
                let content = crate::content::Content::from_bytes(b"use x;\n");
                let fragments = vec![Fragment {
                    range: 4..5,
                    ids: vec![(1, CondId::Number(1))],
                    escape: None,
                }];
                Found {
                    matches: Some(crate::search::FileMatches {
                        content,
                        fragments,
                        lines: 1,
                    }),
                    ..found(name)
                }
            })
            .collect();
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        assert_eq!(
            text,
            "files: 4  lines: 4  matches: 4\n#1 text has 'x'\n\nrepeated:\n  ×4 use ⟦#1:x⟧;    a:1, b:1, c:1, … 1 more\n"
        );
    }

    #[test]
    fn an_empty_result_has_only_the_header() {
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            result_id: Some("r1"),
            legend: &[(CondId::Number(1), "x".into())],
            files: &[],
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        assert_eq!(text, "files: 0  lines: 0  matches: 0  result: r1\n");
    }
}
