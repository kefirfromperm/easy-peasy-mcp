//! Matched lines repeated in many files, printed once (OUTPUT.md#repeated-lines).

use std::collections::HashMap;

use super::Fragment;
use super::lines::lines_of;
use crate::content::Content;
use crate::search::Found;

/// A matched line repeated in many files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatedLine {
    /// The line without its leading and trailing whitespace, as a content of one line.
    pub content: Content,
    /// Its fragments in `content`.
    pub fragments: Vec<Fragment>,
    /// Its files in their order: the index of the file and the 0-based index of its first such line.
    pub files: Vec<(usize, usize)>,
}

/// The repeated lines of the files and, per file, the lines which are left out of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Repeated {
    pub lines: Vec<RepeatedLine>,
    removed: Vec<Vec<usize>>,
}

impl Repeated {
    /// The fragments of the file `file` which are not on its repeated lines.
    pub fn rest(&self, file: usize, content: &Content, fragments: &[Fragment]) -> Vec<Fragment> {
        match self.removed.get(file) {
            Some(lines) if !lines.is_empty() => fragments
                .iter()
                .filter(|f| !lines.contains(&content.line_of(f.range.start)))
                .cloned()
                .collect(),
            _ => fragments.to_vec(),
        }
    }
}

/// The matched lines which are identical without their leading and trailing whitespace, with the same fragments, in
/// at least `min` files (`0`: none). A line touched by a multiline fragment or with a fragment only in its leading or
/// trailing whitespace is never repeated. The lines are ordered by the number of files, more first, then by their
/// first file.
pub fn repeated(files: &[Found], min: usize) -> Repeated {
    let mut result = Repeated {
        lines: Vec::new(),
        removed: vec![Vec::new(); files.len()],
    };
    if min == 0 {
        return result;
    }
    let mut keys: HashMap<(String, String), usize> = HashMap::new();
    let mut groups: Vec<(RepeatedLine, Vec<(usize, usize)>)> = Vec::new();
    for (index, file) in files.iter().enumerate() {
        let Some(m) = &file.matches else {
            continue;
        };
        for line in lines_of(&m.content, &m.fragments) {
            let Some((text, fragments)) = trimmed(&m.content, &m.fragments, line) else {
                continue;
            };
            let key = (text.to_string(), format!("{fragments:?}"));
            let group = *keys.entry(key).or_insert_with(|| {
                groups.push((
                    RepeatedLine {
                        content: Content::from_text(text.to_string()),
                        fragments,
                        files: Vec::new(),
                    },
                    Vec::new(),
                ));
                groups.len() - 1
            });
            let (repeated, all) = &mut groups[group];
            if repeated.files.last().is_none_or(|(f, _)| *f != index) {
                repeated.files.push((index, line));
            }
            all.push((index, line));
        }
    }
    for (line, all) in groups {
        if line.files.len() >= min {
            for (file, index) in all {
                result.removed[file].push(index);
            }
            result.lines.push(line);
        }
    }
    result
        .lines
        .sort_by_key(|l| (std::cmp::Reverse(l.files.len()), l.files[0]));
    result
}

/// The line `line` without its leading and trailing whitespace and the fragments which touch it, relative to that
/// text; `None` if a fragment touches another line too or is only in the whitespace.
fn trimmed<'a>(
    content: &'a Content,
    fragments: &[Fragment],
    line: usize,
) -> Option<(&'a str, Vec<Fragment>)> {
    let (start, _) = content.line_range(line);
    let full = content.line(line);
    let text = full.trim();
    let from = start + (full.len() - full.trim_start().len());
    let to = from + text.len();
    let mut out = Vec::new();
    for f in fragments {
        let first = content.line_of(f.range.start);
        let last = content.line_of(f.range.end.saturating_sub(1).max(f.range.start));
        if first > line || last < line {
            continue;
        }
        let range = f.range.start.max(from)..f.range.end.min(to);
        if first != last || range.start > range.end || (range.is_empty() && !f.range.is_empty()) {
            return None;
        }
        let shifted = |r: &std::ops::Range<usize>| {
            let start = r.start.clamp(from, to) - from;
            start..(r.end.clamp(from, to) - from).max(start)
        };
        out.push(Fragment {
            range: range.start - from..range.end - from,
            ids: f.ids.clone(),
            escape: f.escape,
            groups: f
                .groups
                .iter()
                .map(|g| super::SubMatch {
                    range: shifted(&g.range),
                    ids: g.ids.clone(),
                })
                .collect(),
        });
    }
    Some((text, out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::ast::CondId;
    use crate::search::FileMatches;

    fn file(text: &str, needle: &str, id: u32) -> Found {
        let content = Content::from_bytes(text.as_bytes());
        let fragments: Vec<Fragment> = content
            .text()
            .match_indices(needle)
            .map(|(i, _)| Fragment {
                range: i..i + needle.len(),
                ids: vec![(id as usize, CondId::Number(id))],
                escape: None,
                groups: Vec::new(),
            })
            .collect();
        let lines = super::super::matched_lines(&content, &fragments);
        Found {
            rel: "f".into(),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: Some(FileMatches {
                content,
                fragments,
                lines,
            }),
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
            git: None,
            root: 0,
        }
    }

    #[test]
    fn lines_in_enough_files() {
        let files = [
            file("use x;\nuse y;\n", "use", 1),
            file("  use x;  \n", "use", 1),
            file("use x;\nuse x;\n", "use", 1),
        ];
        let r = repeated(&files, 0..files.len(), 3);
        assert_eq!(r.lines.len(), 1);
        assert_eq!(r.lines[0].content.text(), "use x;");
        assert_eq!(r.lines[0].fragments[0].range, 0..3);
        assert_eq!(r.lines[0].files, [(0, 0), (1, 0), (2, 0)]);
        let m = files[2].matches.as_ref().unwrap();
        assert!(
            r.rest(2, &m.content, &m.fragments).is_empty(),
            "every line of the key is left out"
        );
        let m = files[0].matches.as_ref().unwrap();
        assert_eq!(r.rest(0, &m.content, &m.fragments).len(), 1);
        assert!(repeated(&files, 0..files.len(), 4).lines.is_empty());
        assert!(repeated(&files, 0..files.len(), 0).lines.is_empty(), "0 is off");
    }

    #[test]
    fn other_ids_are_other_lines() {
        let files = [
            file("use x;\n", "use", 1),
            file("use x;\n", "use", 2),
            file("use x;\n", "use", 1),
        ];
        assert!(repeated(&files, 0..files.len(), 3).lines.is_empty());
        assert_eq!(repeated(&files, 0..files.len(), 2).lines[0].files, [(0, 0), (2, 0)]);
    }

    #[test]
    fn multiline_fragments_are_not_grouped() {
        let files = [
            file("a\nb\n", "a\nb", 1),
            file("a\nb\n", "a\nb", 1),
            file("a\nb\n", "a\nb", 1),
        ];
        assert!(repeated(&files, 0..files.len(), 2).lines.is_empty());
    }

    #[test]
    fn more_files_first() {
        let files = [
            file("b\n", "b", 1),
            file("b\n", "b", 1),
            file("a\nb\n", "a", 1),
            file("a\n", "a", 1),
            file("a\n", "a", 1),
        ];
        let r = repeated(&files, 0..files.len(), 2);
        let texts: Vec<&str> = r.lines.iter().map(|l| l.content.text()).collect();
        assert_eq!(texts, ["a", "b"]);
    }

    #[test]
    fn files_which_are_not_listed_count() {
        let files = [
            file("use x;\n", "use", 1),
            file("use x;\nuse y;\n", "use", 1),
            file("use x;\nuse y;\n", "use", 1),
            file("use y;\n", "use", 1),
        ];
        // the second page: the files 2 and 3
        let r = repeated(&files, 2..4, 3);
        let lines: Vec<(&str, usize, &[(usize, usize)])> = r
            .lines
            .iter()
            .map(|l| (l.content.text(), l.count, l.files.as_slice()))
            .collect();
        assert_eq!(
            lines,
            [("use x;", 3, &[(2, 0)][..]), ("use y;", 3, &[(2, 1), (3, 0)][..])],
            "all the files counted, the listed ones located"
        );
        let m = files[3].matches.as_ref().unwrap();
        assert!(
            r.rest(1, &m.content, &m.fragments).is_empty(),
            "the left out lines are per listed file"
        );
        assert!(
            repeated(&files, 3..4, 3)
                .lines
                .iter()
                .all(|l| l.content.text() == "use y;"),
            "a line without a listed file is not repeated"
        );
    }
}
