//! Matched lines with fragments in the text format: blocks, markers (OUTPUT.md#fragments, OUTPUT.md#highlight).

use std::fmt::Write as _;
use std::ops::Range;

use super::Highlight;
use super::blocks::Block;
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
pub(super) fn lines_of(content: &Content, fragments: &[Fragment]) -> Vec<usize> {
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

/// The blocks as `N:text` for a matched line and `N-text` for a context line in the highlight style, `--` between
/// blocks.
pub fn render(
    content: &Content,
    fragments: &[Fragment],
    blocks: &[Block],
    highlight: Highlight,
) -> String {
    let parts = merge_overlapping(fragments);
    let mut out = String::new();
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            out.push_str("--\n");
        }
        if block.indent > 0 {
            let _ = writeln!(out, "⇥{}", block.indent);
        }
        for line in &block.lines {
            let separator = if line.matched { ':' } else { '-' };
            let _ = write!(out, "{}{separator}", line.index + 1);
            write_styled(
                &mut out,
                content,
                &parts,
                line.index,
                block.indent,
                highlight,
            );
            out.push('\n');
            if highlight == Highlight::Columns {
                push_columns(&mut out, content, fragments, line.index);
            }
        }
    }
    out
}

/// The line with the 0-based index `i` without its first `skip` bytes, with the markers of the fragments for
/// `markers`, as it is otherwise.
pub fn shown_line(
    content: &Content,
    fragments: &[Fragment],
    i: usize,
    skip: usize,
    highlight: Highlight,
) -> String {
    let mut out = String::new();
    write_styled(
        &mut out,
        content,
        &merge_overlapping(fragments),
        i,
        skip,
        highlight,
    );
    out
}

/// Writes the line `i` from its byte `skip`, with the marked parts for `markers`.
fn write_styled(
    out: &mut String,
    content: &Content,
    parts: &[Part],
    i: usize,
    skip: usize,
    highlight: Highlight,
) {
    if highlight == Highlight::Markers {
        write_marked(out, content, parts, i, skip);
    } else {
        let (start, end) = content.line_range(i);
        out.push_str(&content.text()[(start + skip).min(end)..end]);
    }
}

/// Writes the line `  @ s..e ids  @ …` of the fragments which start in the line `i`, if any (OUTPUT.md#highlight).
pub fn push_columns(out: &mut String, content: &Content, fragments: &[Fragment], i: usize) {
    let mut any = false;
    for f in fragments.iter().filter(|f| starts_in(content, f, i)) {
        let at = columns(content, f, i);
        out.push_str("  @ ");
        any = true;
        let _ = write!(out, "{}..", at.start);
        match at.to {
            Some((line, col)) => {
                let _ = write!(out, "{line}:{col}");
            }
            None => {
                let _ = write!(out, "{}", at.end);
            }
        }
        let ids: Vec<String> = f.ids.iter().map(|(_, id)| id.to_string()).collect();
        let _ = write!(out, " {}", ids.join(","));
        if let Some(level) = f.escape {
            let _ = write!(out, "/e{level}");
        }
    }
    if any {
        out.push('\n');
    }
}

/// Whether the fragment starts in the line `i`.
pub(super) fn starts_in(content: &Content, f: &Fragment, i: usize) -> bool {
    let (_, end) = content.line_range(i);
    content.line_of(f.range.start) == i && f.range.start <= end
}

/// Where a fragment which starts in its line is, in characters (OUTPUT.md#fragments).
pub(super) struct Columns {
    /// The 1-based start column.
    pub start: usize,
    /// The exclusive end column; of a multiline fragment the end of its first line.
    pub end: usize,
    /// Of a multiline fragment the 1-based line and the exclusive column of its end.
    pub to: Option<(usize, usize)>,
}

/// The columns of the fragment `f` which starts in the line `i`.
pub(super) fn columns(content: &Content, f: &Fragment, i: usize) -> Columns {
    let (start, _) = content.line_range(i);
    let column = |from: usize, offset: usize| content.text()[from..offset].chars().count() + 1;
    let last = content.line_of(f.range.end.saturating_sub(1).max(f.range.start));
    if last > i {
        let (last_start, _) = content.line_range(last);
        Columns {
            start: column(start, f.range.start),
            end: content.line(i).chars().count() + 1,
            to: Some((last + 1, column(last_start, f.range.end))),
        }
    } else {
        Columns {
            start: column(start, f.range.start),
            end: column(start, f.range.end),
            to: None,
        }
    }
}

/// Writes the line `i` with its marked parts, from its byte `skip`.
fn write_marked(out: &mut String, content: &Content, parts: &[Part], i: usize, skip: usize) {
    let (start, end) = content.line_range(i);
    let mut pos = (start + skip).min(end);
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
            let close = part.range.end.min(end).max(pos);
            out.push_str(&content.text()[pos..close]);
            out.push_str(CLOSE);
            pos = close;
        }
    }
    out.push_str(&content.text()[pos..end]);
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

    /// The matched lines without context.
    fn render_plain(c: &Content, fragments: &[Fragment]) -> String {
        let blocks =
            super::super::blocks::blocks(c, fragments, &super::super::BlockOptions::default());
        render(c, fragments, &blocks, Highlight::Markers)
    }

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
    fn columns_after_the_line() {
        let c = Content::from_bytes("é = x x\nab\ncd\n".as_bytes());
        let x = c.text().find('x').unwrap();
        let ab = c.text().find("ab").unwrap();
        let fragments = vec![
            frag(x..x + 1, &[1, 3]),
            escaped(x + 2..x + 3, 2, 0),
            frag(ab..ab + 5, &[1]),
        ];
        let blocks =
            super::super::blocks::blocks(&c, &fragments, &super::super::BlockOptions::default());
        assert_eq!(
            render(&c, &fragments, &blocks, Highlight::Columns),
            "1:é = x x\n  @ 5..6 #1,#3  @ 7..8 #2/e0\n2:ab\n  @ 1..3:3 #1\n3:cd\n"
        );
        assert_eq!(
            render(&c, &fragments, &blocks, Highlight::None),
            "1:é = x x\n2:ab\n3:cd\n"
        );
    }

    #[test]
    fn escape_levels_after_the_ids() {
        let c = Content::from_bytes(b"abcdef\n");
        assert_eq!(
            render_plain(&c, &[escaped(1..3, 1, 0)]),
            "1:a\u{27e6}#1/e0:bc\u{27e7}def\n"
        );
        let fragments = vec![escaped(1..3, 2, 1), frag(2..4, &[1]), escaped(3..5, 2, 0)];
        assert_eq!(
            render_plain(&c, &fragments),
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
            render_plain(&c, &fragments),
            "1:one ⟦#1:two⟧\n--\n4:five ⟦#1:two⟧\n"
        );
    }

    #[test]
    fn overlapping_parts_merge_with_all_ids() {
        let c = Content::from_bytes(b"abcdef\n");
        let fragments = vec![frag(1..4, &[2]), frag(2..5, &[1])];
        assert_eq!(render_plain(&c, &fragments), "1:a⟦#1,#2:bcde⟧f\n");
    }

    #[test]
    fn multiline_fragments_open_and_close_on_their_lines() {
        let c = Content::from_bytes(b"ab\ncd\nef\n");
        let fragments = vec![frag(1..7, &[1])];
        assert_eq!(render_plain(&c, &fragments), "1:a⟦#1:b\n2:cd\n3:e⟧f\n");
    }

    #[test]
    fn empty_fragments_of_empty_lines() {
        let c = Content::from_bytes(b"a\n\nb\n");
        assert_eq!(render_plain(&c, &[frag(2..2, &[1])]), "2:⟦#1:⟧\n");
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
