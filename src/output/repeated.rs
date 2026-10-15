//! Matched lines repeated in many files, printed once (OUTPUT.md#repeated-lines).

use std::collections::HashMap;
use std::ops::Range;

use super::Fragment;
use super::blocks::{self, BlockLine, BlockOptions};
use super::lines::lines_of;
use crate::content::Content;
use crate::search::Found;

/// A matched line repeated in many files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatedLine {
    /// The context lines before it, without the common leading whitespace of its block and their trailing whitespace.
    pub before: Vec<String>,
    /// The line without its leading (or, with context, the common leading of its block) and trailing whitespace, as a
    /// content of one line.
    pub content: Content,
    /// Its fragments in `content`.
    pub fragments: Vec<Fragment>,
    /// Its listed files in their order: the index of the file among all the files and the 0-based index of its first
    /// such line.
    pub files: Vec<(usize, usize)>,
    /// The number of all the files of the result which have it, listed or not.
    pub count: usize,
    /// The context lines after it, like `before`.
    pub after: Vec<String>,
}

/// The repeated lines of the files and, per file, the lines which are left out of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Repeated {
    pub lines: Vec<RepeatedLine>,
    /// Per listed file, the indexes of its lines which are left out.
    removed: Vec<Vec<usize>>,
}

impl Repeated {
    /// The fragments of the listed file `file` (its index among the listed files) which are not on its repeated
    /// lines.
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

/// A line of `repeated` while the files are read: the last file counted and its lines in the listed files.
struct Group {
    line: RepeatedLine,
    last: Option<usize>,
    removed: Vec<(usize, usize)>,
}

/// The matched lines which are identical without their leading and trailing whitespace, with the same fragments, in
/// at least `min` of all the `files` (`0`: none) and in at least one of the `listed` ones (OUTPUT.md#repeated-lines).
/// A line touched by a multiline fragment or with a fragment only in its leading or trailing whitespace is never
/// repeated. With a `context` which has context lines the whole block of a line is compared
/// (OUTPUT.md#repeated-lines-with-context). The lines are ordered by the number of files, more first, then by their
/// first listed file.
pub fn repeated(
    files: &[Found],
    listed: Range<usize>,
    min: usize,
    context: Option<&BlockOptions>,
) -> Repeated {
    let mut result = Repeated {
        lines: Vec::new(),
        removed: vec![Vec::new(); listed.len()],
    };
    if min == 0 {
        return result;
    }
    let context = context.filter(|o| o.has_context());
    let mut keys: HashMap<(Vec<String>, String, String, Vec<String>), usize> = HashMap::new();
    let mut groups: Vec<Group> = Vec::new();
    for (index, file) in files.iter().enumerate() {
        let Some(m) = &file.matches else {
            continue;
        };
        let is_listed = listed.contains(&index);
        let matched = lines_of(&m.content, &m.fragments);
        for &line in &matched {
            let Some(block) = block(&m.content, &m.fragments, &matched, line, context) else {
                continue;
            };
            let key = (
                block.before.clone(),
                block.text.to_string(),
                format!("{:?}", block.fragments),
                block.after.clone(),
            );
            let group = *keys.entry(key).or_insert_with(|| {
                groups.push(Group {
                    line: RepeatedLine {
                        before: block.before,
                        content: Content::from_text(block.text.to_string()),
                        fragments: block.fragments,
                        files: Vec::new(),
                        count: 0,
                        after: block.after,
                    },
                    last: None,
                    removed: Vec::new(),
                });
                groups.len() - 1
            });
            let group = &mut groups[group];
            if group.last != Some(index) {
                group.last = Some(index);
                group.line.count += 1;
                if is_listed {
                    group.line.files.push((index, line));
                }
            }
            if is_listed {
                group.removed.push((index - listed.start, line));
            }
        }
    }
    for Group { line, removed, .. } in groups {
        if line.count >= min && !line.files.is_empty() {
            for (file, index) in removed {
                result.removed[file].push(index);
            }
            result.lines.push(line);
        }
    }
    result
        .lines
        .sort_by_key(|l| (std::cmp::Reverse(l.count), l.files[0]));
    result
}

/// A matched line as it is compared with the lines of the other files: its text and fragments with the context lines
/// of its block.
struct Block<'a> {
    before: Vec<String>,
    text: &'a str,
    fragments: Vec<Fragment>,
    after: Vec<String>,
}

/// The matched line `line` (one of the `matched` lines) with its block of `context` lines, without the common leading
/// whitespace of the block and the trailing whitespace of every line; without context lines the line without its
/// leading and trailing whitespace. `None` if it is not grouped: another matched line in its block, a fragment in
/// the common leading whitespace, or one of [`trimmed`].
fn block<'a>(
    content: &'a Content,
    fragments: &[Fragment],
    matched: &[usize],
    line: usize,
    context: Option<&BlockOptions>,
) -> Option<Block<'a>> {
    let alone = || {
        let (text, fragments) = trimmed(content, fragments, line, None)?;
        Some(Block {
            before: Vec::new(),
            text,
            fragments,
            after: Vec::new(),
        })
    };
    let Some(options) = context else {
        return alone();
    };
    let (lo, hi) = blocks::context(content, line, options);
    if matched.iter().any(|&m| m != line && (lo..=hi).contains(&m)) {
        return None;
    }
    let lines: Vec<BlockLine> = (lo..=hi)
        .map(|index| BlockLine {
            index,
            matched: index == line,
        })
        .filter(|l| l.matched || options.blank_context || !content.line(l.index).trim().is_empty())
        .collect();
    if lines.len() == 1 {
        return alone();
    }
    let indent = blocks::indent(content, &lines, &[]);
    let (text, fragments) = trimmed(content, fragments, line, Some(indent))?;
    let texts = |range: &mut dyn Iterator<Item = &BlockLine>| -> Vec<String> {
        range
            .map(|l| {
                let text = content.line(l.index);
                if text.trim().is_empty() {
                    String::new()
                } else {
                    text[indent..].trim_end().to_string()
                }
            })
            .collect()
    };
    Some(Block {
        before: texts(&mut lines.iter().filter(|l| l.index < line)),
        text,
        fragments,
        after: texts(&mut lines.iter().filter(|l| l.index > line)),
    })
}

/// The line `line` without its leading whitespace (`indent` bytes of it when given) and its trailing whitespace, and
/// the fragments which touch it, relative to that text; `None` if a fragment touches another line too, is only in the
/// whitespace or, with `indent`, starts in the removed whitespace.
fn trimmed<'a>(
    content: &'a Content,
    fragments: &[Fragment],
    line: usize,
    indent: Option<usize>,
) -> Option<(&'a str, Vec<Fragment>)> {
    let (start, _) = content.line_range(line);
    let full = content.line(line);
    let skip = indent.unwrap_or(full.len() - full.trim_start().len());
    // a whitespace-only matched line can be shorter than the common indentation of its block
    let text = full.get(skip..)?.trim_end();
    let from = start + skip;
    let to = from + text.len();
    let mut out = Vec::new();
    for f in fragments {
        let first = content.line_of(f.range.start);
        let last = content.line_of(f.range.end.saturating_sub(1).max(f.range.start));
        if first > line || last < line {
            continue;
        }
        let range = f.range.start.max(from)..f.range.end.min(to);
        if first != last
            || range.start > range.end
            || (range.is_empty() && !f.range.is_empty())
            || (indent.is_some() && f.range.start < from)
        {
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
    use crate::output::blocks::{After, BlockOptions};
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
        let r = repeated(&files, 0..files.len(), 3, None);
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
        assert_eq!(repeated(&files, 0..files.len(), 4, None).lines.len(), 0);
        assert!(
            repeated(&files, 0..files.len(), 0, None).lines.is_empty(),
            "0 is off"
        );
    }

    #[test]
    fn other_ids_are_other_lines() {
        let files = [
            file("use x;\n", "use", 1),
            file("use x;\n", "use", 2),
            file("use x;\n", "use", 1),
        ];
        assert_eq!(repeated(&files, 0..files.len(), 3, None).lines.len(), 0);
        assert_eq!(
            repeated(&files, 0..files.len(), 2, None).lines[0].files,
            [(0, 0), (2, 0)]
        );
    }

    #[test]
    fn multiline_fragments_are_not_grouped() {
        let files = [
            file("a\nb\n", "a\nb", 1),
            file("a\nb\n", "a\nb", 1),
            file("a\nb\n", "a\nb", 1),
        ];
        assert_eq!(repeated(&files, 0..files.len(), 2, None).lines.len(), 0);
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
        let r = repeated(&files, 0..files.len(), 2, None);
        let texts: Vec<&str> = r.lines.iter().map(|l| l.content.text()).collect();
        assert_eq!(texts, ["a", "b"]);
    }

    #[test]
    fn files_which_are_not_listed_count() {
        type Located<'a> = (&'a str, usize, &'a [(usize, usize)]);
        let files = [
            file("use x;\n", "use", 1),
            file("use x;\nuse y;\n", "use", 1),
            file("use x;\nuse y;\n", "use", 1),
            file("use y;\n", "use", 1),
        ];
        // the second page: the files 2 and 3
        let r = repeated(&files, 2..4, 3, None);
        let lines: Vec<Located> = r
            .lines
            .iter()
            .map(|l| (l.content.text(), l.count, l.files.as_slice()))
            .collect();
        assert_eq!(
            lines,
            [
                ("use x;", 3, &[(2, 0)][..]),
                ("use y;", 3, &[(2, 1), (3, 0)][..])
            ],
            "all the files counted, the listed ones located"
        );
        let m = files[3].matches.as_ref().unwrap();
        assert!(
            r.rest(1, &m.content, &m.fragments).is_empty(),
            "the left out lines are per listed file"
        );
        assert!(
            repeated(&files, 3..4, 3, None)
                .lines
                .iter()
                .all(|l| l.content.text() == "use y;"),
            "a line without a listed file is not repeated"
        );
    }

    fn around(before: usize, after: usize) -> BlockOptions {
        BlockOptions {
            before,
            after: After::Lines(after),
            blank_context: true,
            ..BlockOptions::default()
        }
    }

    #[test]
    fn the_same_block_is_grouped_with_its_context() {
        let files = [
            file("a\nkey x\nb\n", "key", 1),
            file("  a\n  key x \n  b\nz\n", "key", 1),
            file("q\na\nkey x\nb\n", "key", 1),
            file("a\nkey x\nc\n", "key", 1),
        ];
        let r = repeated(&files, 0..files.len(), 3, Some(&around(1, 1)));
        assert_eq!(r.lines.len(), 1);
        let line = &r.lines[0];
        assert_eq!(
            (line.before.as_slice(), line.content.text(), line.after.as_slice()),
            (&["a".to_string()][..], "key x", &["b".to_string()][..]),
            "without the common indentation and the trailing whitespace"
        );
        assert_eq!(line.fragments[0].range, 0..3);
        assert_eq!(line.files, [(0, 1), (1, 1), (2, 2)]);
        assert_eq!(line.count, 3, "the file with another context line is not counted");
        let m = files[3].matches.as_ref().unwrap();
        assert_eq!(r.rest(3, &m.content, &m.fragments).len(), 1, "it keeps its line");
    }

    #[test]
    fn a_context_line_of_another_file_differs() {
        let files = [
            file("a\nkey\n", "key", 1),
            file("b\nkey\n", "key", 1),
            file("c\nkey\n", "key", 1),
        ];
        assert!(repeated(&files, 0..files.len(), 2, Some(&around(1, 0))).lines.is_empty());
        assert_eq!(
            repeated(&files, 0..files.len(), 3, None).lines.len(),
            1,
            "spec 1 groups by the line"
        );
    }

    #[test]
    fn a_block_with_another_matched_line_is_not_grouped() {
        let files = [
            file("key\nkey\n", "key", 1),
            file("key\nkey\n", "key", 1),
            file("key\nkey\n", "key", 1),
        ];
        assert!(repeated(&files, 0..files.len(), 2, Some(&around(1, 1))).lines.is_empty());
    }

    #[test]
    fn a_fragment_in_the_common_indentation_is_not_grouped() {
        let files = [
            file("  a\n  key\n", "  key", 1),
            file("  a\n  key\n", "  key", 1),
        ];
        assert!(repeated(&files, 0..files.len(), 2, Some(&around(1, 0))).lines.is_empty());
    }

    #[test]
    fn the_blank_context_lines_follow_blank_context() {
        let files = [
            file("a\n\nkey\n", "key", 1),
            file("a\n  \nkey\n", "key", 1),
        ];
        let r = repeated(&files, 0..files.len(), 2, Some(&around(2, 0)));
        assert_eq!(r.lines[0].before, ["a", ""]);
        let without = BlockOptions {
            blank_context: false,
            ..around(2, 0)
        };
        let r = repeated(&files, 0..files.len(), 2, Some(&without));
        assert_eq!(r.lines[0].before, ["a"]);
    }

    #[test]
    fn a_block_of_only_the_line_is_the_line() {
        let files = [file("key\n", "key", 1), file("  key\n", "key", 1)];
        let r = repeated(&files, 0..files.len(), 2, Some(&around(1, 1)));
        assert_eq!(r.lines.len(), 1);
        assert!(r.lines[0].before.is_empty() && r.lines[0].after.is_empty());
    }
}
