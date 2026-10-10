//! Matched lines with fragments in the text format: blocks, markers (OUTPUT.md#fragments, OUTPUT.md#highlight).

use std::fmt::Write as _;
use std::ops::Range;

use crate::content::Content;
use crate::query::ast::CondId;

/// The opening marker of `highlight = markers`.
const OPEN: &str = "⟦";
/// The closing marker of `highlight = markers`.
const CLOSE: &str = "⟧";

/// A matched part of the content: a byte range of the normalized text, the conditions which produced it with
/// their order of appearance in the query and, for a condition with `/e`, the escape level it was found on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragment {
    pub range: Range<usize>,
    pub ids: Vec<(usize, CondId)>,
    pub escape: Option<u8>,
}

/// A marked part of a line: overlapping fragments with the union of their ranges, ids and escape levels.
#[derive(Debug, Clone)]
struct Part {
    range: Range<usize>,
    ids: Vec<(usize, CondId)>,
    escapes: Vec<u8>,
}

/// Sorts the fragments by position and merges identical ones (the same range and escape level) into one fragment
/// with all ids in the order of appearance.
pub fn merge_identical(mut fragments: Vec<Fragment>) -> Vec<Fragment> {
    fragments.sort_by_key(|f| (f.range.start, f.range.end, f.escape));
    let mut out: Vec<Fragment> = Vec::new();
    for f in fragments {
        match out.last_mut() {
            Some(last) if last.range == f.range && last.escape == f.escape => {
                for id in f.ids {
                    if !last.ids.contains(&id) {
                        last.ids.push(id);
                    }
                }
            }
            _ => out.push(f),
        }
    }
    for f in &mut out {
        f.ids.sort_by_key(|(order, _)| *order);
    }
    out
}

/// The number of lines touched by the fragments.
pub fn matched_lines(content: &Content, fragments: &[Fragment]) -> usize {
    lines_of(content, fragments).len()
}

/// The indexes of the lines touched by the fragments, sorted.
fn lines_of(content: &Content, fragments: &[Fragment]) -> Vec<usize> {
    let mut lines: Vec<usize> = fragments
        .iter()
        .flat_map(|f| {
            let first = content.line_of(f.range.start);
            let last = content.line_of(f.range.end.saturating_sub(1).max(f.range.start));
            first..=last
        })
        .collect();
    lines.sort_unstable();
    lines.dedup();
    lines
}

/// The matched lines as `N:text` with the merged parts marked, blocks of adjacent lines, `--` between blocks.
pub fn render(content: &Content, fragments: &[Fragment]) -> String {
    let parts = merge_overlapping(fragments);
    let mut out = String::new();
    let mut previous: Option<usize> = None;
    for line in lines_of(content, fragments) {
        if previous.is_some_and(|p| p + 1 != line) {
            out.push_str("--\n");
        }
        previous = Some(line);
        let (start, end) = content.line_range(line);
        let _ = write!(out, "{}:", line + 1);
        let mut pos = start;
        for part in parts.iter().filter(|p| {
            (p.range.start >= start && p.range.start <= end)
                || (p.range.start < start && p.range.end > start)
        }) {
            if part.range.start >= start {
                out.push_str(&content.text()[pos..part.range.start]);
                let ids: Vec<String> = part.ids.iter().map(|(_, id)| id.to_string()).collect();
                let _ = write!(out, "{OPEN}{}", ids.join(","));
                for level in &part.escapes {
                    let _ = write!(out, "/e{level}");
                }
                out.push(':');
                pos = part.range.start;
            }
            // a part which ends with the line break closes at the end of the line
            if part.range.end <= end + 1 {
                let close = part.range.end.min(end);
                out.push_str(&content.text()[pos..close]);
                out.push_str(CLOSE);
                pos = close;
            }
        }
        out.push_str(&content.text()[pos..end]);
        out.push('\n');
    }
    out
}

/// Overlapping fragments as one part with the union of their ranges, ids and escape levels.
fn merge_overlapping(fragments: &[Fragment]) -> Vec<Part> {
    let mut sorted: Vec<&Fragment> = fragments.iter().collect();
    sorted.sort_by_key(|f| (f.range.start, f.range.end));
    let mut parts: Vec<Part> = Vec::new();
    for f in sorted {
        match parts.last_mut() {
            Some(last) if f.range.start < last.range.end => {
                last.range.end = last.range.end.max(f.range.end);
                for id in &f.ids {
                    if !last.ids.contains(id) {
                        last.ids.push(id.clone());
                    }
                }
                last.escapes.extend(f.escape);
            }
            _ => parts.push(Part {
                range: f.range.clone(),
                ids: f.ids.clone(),
                escapes: f.escape.into_iter().collect(),
            }),
        }
    }
    for p in &mut parts {
        p.ids.sort_by_key(|(order, _)| *order);
        p.escapes.sort_unstable();
        p.escapes.dedup();
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::query::ast::CondId;

    fn frag(range: std::ops::Range<usize>, ids: &[u32]) -> Fragment {
        Fragment {
            range,
            ids: ids
                .iter()
                .map(|n| (*n as usize, CondId::Number(*n)))
                .collect(),
            escape: None,
        }
    }

    fn escaped(range: std::ops::Range<usize>, id: u32, level: u8) -> Fragment {
        Fragment {
            escape: Some(level),
            ..frag(range, &[id])
        }
    }

    #[test]
    fn escape_levels_after_the_ids() {
        let c = Content::from_bytes(b"abcdef\n");
        assert_eq!(
            render(&c, &[escaped(1..3, 1, 0)]),
            "1:a\u{27e6}#1/e0:bc\u{27e7}def\n"
        );
        let fragments = vec![escaped(1..3, 2, 1), frag(2..4, &[1]), escaped(3..5, 2, 0)];
        assert_eq!(
            render(&c, &fragments),
            "1:a\u{27e6}#1,#2/e0/e1:bcde\u{27e7}f\n"
        );
        let merged = merge_identical(vec![
            escaped(1..3, 1, 1),
            escaped(1..3, 2, 0),
            escaped(1..3, 2, 1),
        ]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[1].ids.len(), 2);
    }

    #[test]
    fn markers_and_blocks() {
        let c = Content::from_bytes(b"one two\nthree\nfour\nfive two\n");
        let text = c.text();
        let two = text.find("two").unwrap();
        let five = text.find("five").unwrap();
        let fragments = vec![frag(two..two + 3, &[1]), frag(five + 5..five + 8, &[1])];
        assert_eq!(
            render(&c, &fragments),
            "1:one ⟦#1:two⟧\n--\n4:five ⟦#1:two⟧\n"
        );
    }

    #[test]
    fn overlapping_parts_merge_with_all_ids() {
        let c = Content::from_bytes(b"abcdef\n");
        let fragments = vec![frag(1..4, &[2]), frag(2..5, &[1])];
        assert_eq!(render(&c, &fragments), "1:a⟦#1,#2:bcde⟧f\n");
    }

    #[test]
    fn multiline_fragments_open_and_close_on_their_lines() {
        let c = Content::from_bytes(b"ab\ncd\nef\n");
        let fragments = vec![frag(1..7, &[1])];
        assert_eq!(render(&c, &fragments), "1:a⟦#1:b\n2:cd\n3:e⟧f\n");
    }

    #[test]
    fn empty_fragments_of_empty_lines() {
        let c = Content::from_bytes(b"a\n\nb\n");
        assert_eq!(render(&c, &[frag(2..2, &[1])]), "2:⟦#1:⟧\n");
    }

    #[test]
    fn counts() {
        let c = Content::from_bytes(b"ab\ncd\nef\n");
        let fragments = vec![frag(0..1, &[1]), frag(1..4, &[1]), frag(7..8, &[2])];
        assert_eq!(matched_lines(&c, &fragments), 3);
    }

    #[test]
    fn identical_fragments_merge() {
        let merged = merge_identical(vec![frag(1..3, &[2]), frag(1..3, &[1]), frag(0..1, &[1])]);
        assert_eq!(merged, vec![frag(0..1, &[1]), frag(1..3, &[1, 2])]);
    }
}
