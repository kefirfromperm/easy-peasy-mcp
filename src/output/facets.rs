//! The values of the facets of the matched files (OUTPUT.md#facets).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::ser::SerializeMap;

use super::{SEP, path_parts};
use crate::query::ast::CondId;
use crate::search::Found;

/// The width of the column of the facet names in the text format.
const NAME_WIDTH: usize = 12;

/// A facet with its values and their numbers of files, in their order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet {
    pub name: String,
    pub values: Vec<(String, usize)>,
}

impl Facet {
    /// Whether no matched file has a value of the facet: it has no line in the text format.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// The text line: the name in its column, the first `limit` values, `… N more (M)`.
    #[must_use]
    pub fn line(&self, limit: usize) -> String {
        let mut out = format!("{:<NAME_WIDTH$}", format!("{}:", self.name));
        let shown: Vec<String> = self
            .values
            .iter()
            .take(limit)
            .map(|(value, n)| format!("{value} {n}"))
            .collect();
        out.push_str(&shown.join(" · "));
        if let Some((more, files)) = self.more(limit) {
            let _ = write!(out, " · … {more} more ({files})");
        }
        out
    }

    /// The number of values over the limit and their files.
    fn more(&self, limit: usize) -> Option<(usize, usize)> {
        let rest = self.values.get(limit..).filter(|rest| !rest.is_empty())?;
        Some((rest.len(), rest.iter().map(|(_, n)| n).sum()))
    }

    /// The JSON object: the first `limit` values and `"… N more": M`.
    #[must_use]
    pub fn json(&self, limit: usize) -> Values<'_> {
        Values {
            values: &self.values[..self.values.len().min(limit)],
            more: self.more(limit),
        }
    }
}

/// The values of a facet in JSON.
pub struct Values<'a> {
    values: &'a [(String, usize)],
    more: Option<(usize, usize)>,
}

impl serde::Serialize for Values<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for (value, n) in self.values {
            map.serialize_entry(value, n)?;
        }
        if let Some((more, files)) = self.more {
            map.serialize_entry(&format!("… {more} more"), &files)?;
        }
        map.end()
    }
}

/// The facets `names` of the files: `dir` with its directories `depth` levels deep; the conditions in the order of
/// the legend for equal numbers of files.
#[must_use]
pub fn facets(
    files: &[Found],
    names: &[String],
    legend: &[(CondId, String)],
    depth: usize,
) -> Vec<Facet> {
    names
        .iter()
        .map(|name| {
            let values = match name.as_str() {
                "dir" => count(files.iter().map(|f| dir_value(&f.rel, depth))),
                "ext" => count(files.iter().map(ext_value)),
                "lang" => count(files.iter().filter_map(|f| f.meta.as_ref()?.lang.clone())),
                "kind" => count(
                    files
                        .iter()
                        .filter_map(|f| Some(f.meta.as_ref()?.kind?.keyword().to_string())),
                ),
                "git" => count(files.iter().flat_map(git_values)),
                _ => conditions(files, legend),
            };
            Facet {
                name: name.clone(),
                values,
            }
        })
        .collect()
}

/// The `ext` value: the extension of the name without the dot, `(none)` without one.
fn ext_value(file: &Found) -> String {
    let name = match &file.member {
        Some(member) => member.rsplit('/').next().unwrap_or(member).to_string(),
        None => file
            .rel
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
    };
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext.to_string(),
        _ => "(none)".to_string(),
    }
}

/// The `git` values of a file: its statuses, `clean` without one; none outside of a repository.
fn git_values(file: &Found) -> Vec<String> {
    let Some(status) = file.git else {
        return Vec::new();
    };
    let mut values: Vec<String> = [
        (status.modified, "modified"),
        (status.staged, "staged"),
        (status.untracked, "untracked"),
        (status.conflicted, "conflicted"),
    ]
    .into_iter()
    .filter(|(has, _)| *has)
    .map(|(_, value)| value.to_string())
    .collect();
    if values.is_empty() {
        values.push("clean".to_string());
    }
    values
}

/// The `condition` values: the files every condition made match or produced a fragment in, more first, then in the
/// order of the legend.
fn conditions(files: &[Found], legend: &[(CondId, String)]) -> Vec<(String, usize)> {
    let mut counts: Vec<(String, usize)> = legend
        .iter()
        .map(|(id, _)| {
            let n = files
                .iter()
                .filter(|f| {
                    f.conditions.contains(id)
                        || f.matches.as_ref().is_some_and(|m| {
                            m.fragments
                                .iter()
                                .any(|fr| fr.ids.iter().any(|(_, i)| i == id))
                        })
                })
                .count();
            (id.to_string(), n)
        })
        .filter(|(_, n)| *n > 0)
        .collect();
    // a stable sort keeps the order of the legend for equal numbers
    counts.sort_by(|(_, a), (_, b)| b.cmp(a));
    counts
}

/// The `dir` value of a file: its directory `depth` levels below the working directory, its own directory when it is
/// higher up, `./` in the working directory; with the separator at the end.
#[must_use]
pub fn dir_value(rel: &Path, depth: usize) -> String {
    let parts = path_parts(rel);
    let dirs = &parts[..parts.len().saturating_sub(1)];
    let dirs = &dirs[..dirs.len().min(depth)];
    if dirs.is_empty() {
        format!(".{SEP}")
    } else {
        format!("{}{SEP}", dirs.join(&SEP.to_string()))
    }
}

/// The number of files per value, more files first, then in the order of the values.
pub fn count(values: impl IntoIterator<Item = String>) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for value in values {
        *counts.entry(value).or_default() += 1;
    }
    let mut counts: Vec<(String, usize)> = counts.into_iter().collect();
    counts.sort_by(|(a, n), (b, m)| m.cmp(n).then_with(|| a.cmp(b)));
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn directories_at_a_depth() {
        let s = std::path::MAIN_SEPARATOR;
        assert_eq!(
            dir_value(Path::new("tests/integration/foo/x.rs"), 2),
            format!("tests{s}integration{s}")
        );
        assert_eq!(dir_value(Path::new("src/main.rs"), 2), format!("src{s}"));
        assert_eq!(dir_value(Path::new("README.md"), 2), format!(".{s}"));
        assert_eq!(
            dir_value(Path::new("tests/integration/x.rs"), 1),
            format!("tests{s}")
        );
    }

    #[test]
    fn more_files_first() {
        let values = ["b", "a", "b", "c", "a", "b"].map(String::from);
        assert_eq!(
            count(values),
            [
                ("b".to_string(), 3),
                ("a".to_string(), 2),
                ("c".to_string(), 1)
            ]
        );
    }

    #[test]
    fn values_over_the_limit() {
        let facet = Facet {
            name: "dir".to_string(),
            values: (1..=10).rev().map(|n| (format!("d{n}/"), n)).collect(),
        };
        assert_eq!(
            facet.line(8),
            "dir:        d10/ 10 · d9/ 9 · d8/ 8 · d7/ 7 · d6/ 6 · d5/ 5 · d4/ 4 · d3/ 3 · … 2 more (3)"
        );
        let short = Facet {
            name: "condition".to_string(),
            values: vec![("#1".to_string(), 3)],
        };
        assert_eq!(short.line(8), "condition:  #1 3");
    }
}
