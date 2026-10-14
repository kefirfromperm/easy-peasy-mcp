//! Matched lines with fragments in the text format: blocks, markers (OUTPUT.md#fragments, OUTPUT.md#highlight).

use std::fmt::Write as _;
use std::ops::Range;

use super::Highlight;
use super::blocks::Block;
use super::omit::{Omit, Part as OmitPart};
use crate::content::Content;
use crate::query::ast::CondId;

/// A matched part of the content: a byte range of the normalized text, the conditions which produced it with
/// their order of appearance in the query, for a condition with `/e` the escape level it was found on and, for a
/// `text =~` condition with capture groups, its sub-groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragment {
    pub range: Range<usize>,
    pub ids: Vec<(usize, CondId)>,
    pub escape: Option<u8>,
    pub groups: Vec<SubMatch>,
}

/// A capture group of a regex match (OUTPUT.md#sub-groups): a byte range of the normalized text and its references,
/// `@def.1`, `@def.fname`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubMatch {
    pub range: Range<usize>,
    pub ids: Vec<String>,
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
                for group in f.groups {
                    if !last.groups.contains(&group) {
                        last.groups.push(group);
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
/// blocks, `⇥N` before a dedented block; without the line numbers, the gaps and the `⇥N` lines which `omit` leaves
/// out.
pub fn render(
    content: &Content,
    fragments: &[Fragment],
    blocks: &[Block],
    highlight: Highlight<'_>,
    max_line_length: usize,
    omit: Omit,
) -> String {
    let parts = merge_overlapping(fragments);
    let mut out = String::new();
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 && !omit.has(OmitPart::Gaps) {
            out.push_str("--\n");
        }
        if block.indent > 0 && !omit.has(OmitPart::Indent) {
            let _ = writeln!(out, "⇥{}", block.indent);
        }
        let plain = match highlight {
            Highlight::Auto(open, close) => {
                plain_lines(content, &parts, block, [open, close], max_line_length)
            }
            _ => Vec::new(),
        };
        for line in &block.lines {
            if !omit.has(OmitPart::LineNumbers) {
                let separator = if line.matched { ':' } else { '-' };
                let _ = write!(out, "{}{separator}", line.index + 1);
            }
            let style = if plain.contains(&line.index) {
                Highlight::None
            } else {
                highlight
            };
            write_cut(
                &mut out,
                content,
                &parts,
                line.index,
                block.indent,
                style,
                max_line_length,
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
/// `markers` and for `auto` unless they outgrow it, as it is otherwise.
pub fn shown_line(
    content: &Content,
    fragments: &[Fragment],
    i: usize,
    skip: usize,
    highlight: Highlight<'_>,
) -> String {
    let parts = merge_overlapping(fragments);
    let styled = |highlight| {
        let mut out = String::new();
        write_styled(&mut out, content, &parts, i, skip, highlight);
        out
    };
    let marked = styled(highlight);
    if let Highlight::Auto(..) = highlight {
        let plain = styled(Highlight::None);
        if outgrown(plain.chars().count(), marked.chars().count()) {
            return plain;
        }
    }
    marked
}

/// The lines of the blocks which `auto` writes without markers (OUTPUT.md#highlight).
pub(super) fn plain_lines_of(
    content: &Content,
    fragments: &[Fragment],
    blocks: &[Block],
    markers: [&str; 2],
    max: usize,
) -> Vec<usize> {
    let parts = merge_overlapping(fragments);
    blocks
        .iter()
        .flat_map(|block| plain_lines(content, &parts, block, markers, max))
        .collect()
}

/// Whether markers which make a text of `plain` characters `marked` characters long outgrow it
/// (OUTPUT.md#highlight).
fn outgrown(plain: usize, marked: usize) -> bool {
    marked - plain > plain
}

/// The lines of the block which `auto` writes without markers: those whose markers would add more characters than
/// they have as they are shown, after the dedent and the cut. The lines joined by a part which goes on in the next
/// line are counted together.
fn plain_lines(
    content: &Content,
    parts: &[Part],
    block: &Block,
    markers: [&str; 2],
    max: usize,
) -> Vec<usize> {
    let mut plain = Vec::new();
    // the lines of the current group, their characters without and with the markers, the last line its parts reach
    let mut group: Vec<usize> = Vec::new();
    let (mut chars, mut marked, mut reach) = (0, 0, 0);
    let mut flush = |group: &mut Vec<usize>, chars: &mut usize, marked: &mut usize| {
        if outgrown(*chars, *marked) {
            plain.append(group);
        }
        group.clear();
        (*chars, *marked) = (0, 0);
    };
    for line in &block.lines {
        let (start, end) = content.line_range(line.index);
        // the parts are merged, so sorted by their starts and by their ends
        let first = parts.partition_point(|p| p.range.end <= start && p.range.start < start);
        let Some(last) = parts[first..]
            .iter()
            .take_while(|p| p.range.start <= end)
            .last()
        else {
            continue;
        };
        if !group.is_empty() && line.index > reach {
            flush(&mut group, &mut chars, &mut marked);
        }
        let shown = |highlight| {
            let mut out = String::new();
            write_cut(
                &mut out,
                content,
                parts,
                line.index,
                block.indent,
                highlight,
                max,
            );
            out.chars().count()
        };
        chars += shown(Highlight::None);
        marked += shown(Highlight::Markers(markers[0], markers[1]));
        group.push(line.index);
        let last_line = content.line_of(last.range.end.saturating_sub(1).max(last.range.start));
        reach = reach.max(line.index).max(last_line);
    }
    flush(&mut group, &mut chars, &mut marked);
    plain
}

/// Writes the line `i` from its byte `skip`, with the marked parts for `markers`.
fn write_styled(
    out: &mut String,
    content: &Content,
    parts: &[Part],
    i: usize,
    skip: usize,
    highlight: Highlight<'_>,
) {
    if let Some(markers) = highlight.markers() {
        write_marked(out, content, parts, i, skip, markers);
    } else {
        let (start, end) = content.line_range(i);
        out.push_str(&content.text()[(start + skip).min(end)..end]);
    }
}

/// Writes the line `  @ s..e ids  @ …` of the fragments which start in the line `i`, if any (OUTPUT.md#highlight).
pub fn push_columns(out: &mut String, content: &Content, fragments: &[Fragment], i: usize) {
    if let Some(columns) = columns_text(content, fragments, i) {
        let _ = writeln!(out, "  {columns}");
    }
}

/// `@ s..e ids  @ …`: the columns of the fragments which start in the line `i`; `None` without any.
pub fn columns_text(content: &Content, fragments: &[Fragment], i: usize) -> Option<String> {
    let mut out = String::new();
    for f in fragments.iter().filter(|f| starts_in(content, f, i)) {
        let at = columns(content, f, i);
        if !out.is_empty() {
            out.push_str("  ");
        }
        let _ = write!(out, "@ {}..", at.start);
        match at.to {
            Some((line, col)) => {
                let _ = write!(out, "{line}:{col}");
            }
            None => {
                let _ = write!(out, "{}", at.end);
            }
        }
        // a fragment without ids: `match_ids` is left out
        if !f.ids.is_empty() {
            let ids: Vec<String> = f.ids.iter().map(|(_, id)| id.to_string()).collect();
            let _ = write!(out, " {}", ids.join(","));
        }
        if let Some(level) = f.escape {
            let _ = write!(out, "/e{level}");
        }
    }
    (!out.is_empty()).then_some(out)
}

/// The lines touched by the fragments with the number of fragments which start in each, so a line a multiline
/// fragment only continues into has 0 (OUTPUT.md#counts-per-file-and-per-line).
pub(super) fn line_counts(content: &Content, fragments: &[Fragment]) -> Vec<(usize, usize)> {
    let mut starts = std::collections::HashMap::new();
    for f in fragments {
        *starts.entry(content.line_of(f.range.start)).or_insert(0) += 1;
    }
    lines_of(content, fragments)
        .into_iter()
        .map(|i| (i, starts.get(&i).copied().unwrap_or(0)))
        .collect()
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

/// Writes the line `i` with its parts marked with the opening and the closing marker, from its byte `skip`.
fn write_marked(
    out: &mut String,
    content: &Content,
    parts: &[Part],
    i: usize,
    skip: usize,
    markers: [&str; 2],
) {
    let (start, end) = content.line_range(i);
    write_window(
        out,
        content,
        parts,
        (start + skip).min(end)..end,
        end,
        markers,
    );
}

/// Writes the bytes `window` of a line which ends at `line_end` with the parts in it marked: a part opens where it
/// starts and closes where it ends, one which ends with the line break at the end of the line; a part which goes on
/// in the next line stays open.
fn write_window(
    out: &mut String,
    content: &Content,
    parts: &[Part],
    window: Range<usize>,
    line_end: usize,
    [open, close]: [&str; 2],
) {
    let text = content.text();
    let mut pos = window.start;
    for part in parts.iter().filter(|p| {
        (p.range.start >= window.start && p.range.start <= window.end)
            || (p.range.start < window.start && p.range.end > window.start)
    }) {
        if part.range.start >= window.start {
            out.push_str(&text[pos..part.range.start]);
            out.push_str(open);
            // a part without ids and levels: `match_ids` is left out
            if !part.ids.is_empty() || !part.escapes.is_empty() {
                let ids: Vec<String> = part.ids.iter().map(|(_, id)| id.to_string()).collect();
                out.push_str(&ids.join(","));
                for level in &part.escapes {
                    let _ = write!(out, "/e{level}");
                }
                out.push(':');
            }
            pos = part.range.start;
        }
        if part.range.end <= line_end + 1 {
            let to = part.range.end.min(line_end).max(pos);
            out.push_str(&text[pos..to]);
            out.push_str(close);
            pos = to;
        }
    }
    out.push_str(&text[pos..window.end]);
}

/// The text of the line `i` from its byte `skip` as it is shown in the style, cut to windows when it is longer than
/// `max` characters (OUTPUT.md#long-lines); whether it was cut.
pub fn cut_line(
    content: &Content,
    fragments: &[Fragment],
    i: usize,
    skip: usize,
    highlight: Highlight<'_>,
    max: usize,
) -> (String, bool) {
    let mut out = String::new();
    let cut = write_cut(
        &mut out,
        content,
        &merge_overlapping(fragments),
        i,
        skip,
        highlight,
        max,
    );
    (out, cut)
}

/// Writes the line `i` from its byte `skip` in the style, cut to windows when it is longer than `max` characters,
/// `…` where characters are left out; whether it was cut.
fn write_cut(
    out: &mut String,
    content: &Content,
    parts: &[Part],
    i: usize,
    skip: usize,
    highlight: Highlight<'_>,
    max: usize,
) -> bool {
    let (start, end) = content.line_range(i);
    let from = (start + skip).min(end);
    let Some(windows) = windows(content.text(), parts, from..end, max) else {
        write_styled(out, content, parts, i, skip, highlight);
        return false;
    };
    for (n, window) in windows.iter().enumerate() {
        if n > 0 || window.start > from {
            out.push('…');
        }
        match highlight.markers() {
            Some(markers) => write_window(out, content, parts, window.clone(), end, markers),
            None => out.push_str(&content.text()[window.clone()]),
        }
    }
    if windows.last().is_some_and(|w| w.end < end) {
        out.push('…');
    }
    true
}

/// The byte ranges of the line `line` which are shown when it is longer than `max` characters: windows around the
/// parts in it, each with `(max − their characters) / (2 × parts)` characters on both sides, merged when they touch;
/// without parts its first `max` characters. `None` when the line is shown whole.
fn windows(
    text: &str,
    parts: &[Part],
    line: Range<usize>,
    max: usize,
) -> Option<Vec<Range<usize>>> {
    // the byte offset of every character of the line and of its end
    let offsets: Vec<usize> = text[line.clone()]
        .char_indices()
        .map(|(b, _)| line.start + b)
        .chain([line.end])
        .collect();
    let n = offsets.len() - 1;
    if n <= max {
        return None;
    }
    let char_of = |byte: usize| offsets.partition_point(|&o| o < byte);
    let in_line: Vec<(usize, usize)> = parts
        .iter()
        .filter(|p| p.range.start < line.end && p.range.end > line.start)
        .map(|p| {
            (
                char_of(p.range.start.max(line.start)),
                char_of(p.range.end.min(line.end)),
            )
        })
        .collect();
    if in_line.is_empty() {
        let start = line.start..offsets[max];
        return Some(std::iter::once(start).collect());
    }
    let marked: usize = in_line.iter().map(|(s, e)| e - s).sum();
    let around = max.saturating_sub(marked) / (2 * in_line.len());
    let mut windows: Vec<(usize, usize)> = Vec::new();
    for (s, e) in in_line {
        let window = (s.saturating_sub(around), (e + around).min(n));
        match windows.last_mut() {
            Some(last) if window.0 <= last.1 => last.1 = last.1.max(window.1),
            _ => windows.push(window),
        }
    }
    Some(
        windows
            .into_iter()
            .map(|(s, e)| offsets[s]..offsets[e])
            .collect(),
    )
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
        render(
            c,
            fragments,
            &blocks,
            Highlight::Markers("⟦", "⟧"),
            usize::MAX,
            super::super::omit::Omit::default(),
        )
    }

    fn frag(range: std::ops::Range<usize>, ids: &[u32]) -> Fragment {
        Fragment {
            range,
            ids: ids
                .iter()
                .map(|n| (*n as usize, CondId::Number(*n)))
                .collect(),
            escape: None,
            groups: Vec::new(),
        }
    }

    fn escaped(range: std::ops::Range<usize>, id: u32, level: u8) -> Fragment {
        Fragment {
            escape: Some(level),
            ..frag(range, &[id])
        }
    }

    #[test]
    fn only_the_matches() {
        let c = Content::from_bytes(b"let a = bb;\nx\nyz");
        let fragments = vec![frag(4..5, &[1]), frag(8..10, &[2]), frag(12..15, &[1])];
        let omit = super::super::omit::Omit::default();
        let markers = Highlight::Markers("⟦", "⟧");
        assert_eq!(
            render_matches(&c, &fragments, markers, omit),
            "1:⟦#1:a⟧\n1:⟦#2:bb⟧\n2:⟦#1:x\n3:y⟧\n"
        );
        assert_eq!(
            render_matches(&c, &fragments, Highlight::None, omit),
            "1:a\n1:bb\n2:x\n3:y\n"
        );
        // auto: the markers outgrow the short matches
        assert_eq!(
            render_matches(&c, &fragments, Highlight::Auto("⟦", "⟧"), omit),
            "1:a\n1:bb\n2:x\n3:y\n"
        );
        assert_eq!(
            render_matches(&c, &fragments[..1], Highlight::Columns, omit),
            "1:a\n  @ 5..6 #1\n"
        );
        let numbers = super::super::omit::Omit::from_parts(&[OmitPart::LineNumbers]);
        assert_eq!(render_matches(&c, &fragments[..2], Highlight::None, numbers), "a\nbb\n");
    }

    #[test]
    fn only_the_matches_with_auto() {
        let c = Content::from_bytes(b"let a = bb;");
        let auto = Highlight::Auto("⟦", "⟧");
        let omit = super::super::omit::Omit::default();
        assert_eq!(render_matches(&c, &[frag(0..10, &[1])], auto, omit), "1:⟦#1:let a = bb⟧\n");
        // a match without ids has nothing to mark with auto, `markers` marks it
        assert_eq!(render_matches(&c, &[frag(0..10, &[])], auto, omit), "1:let a = bb\n");
        assert_eq!(
            render_matches(&c, &[frag(0..10, &[])], Highlight::Markers("⟦", "⟧"), omit),
            "1:⟦let a = bb⟧\n"
        );
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
            render(
                &c,
                &fragments,
                &blocks,
                Highlight::Columns,
                usize::MAX,
                Omit::default()
            ),
            "1:é = x x\n  @ 5..6 #1,#3  @ 7..8 #2/e0\n2:ab\n  @ 1..3:3 #1\n3:cd\n"
        );
        assert_eq!(
            render(
                &c,
                &fragments,
                &blocks,
                Highlight::None,
                usize::MAX,
                Omit::default()
            ),
            "1:é = x x\n2:ab\n3:cd\n"
        );
    }

    #[test]
    fn long_lines_cut_to_windows() {
        let c = Content::from_bytes(
            "The quick brown fox jumps over the lazy dog and then TODO fix the fence near the old barn\n\
             short TODO\n\
             This context line is much longer than twenty characters\n\
             TODO at the start and TODO again at the end of this rather long line TODO\n\
             \x20   é TODO é is here and there and everywhere\n"
                .as_bytes(),
        );
        let fragments: Vec<Fragment> = c
            .text()
            .match_indices("TODO")
            .map(|(i, _)| frag(i..i + 4, &[1]))
            .collect();
        let markers = Highlight::Markers("⟦", "⟧");
        let cut =
            |i: usize, skip: usize, max: usize| cut_line(&c, &fragments, i, skip, markers, max);
        assert_eq!(
            cut(0, 0, 20),
            ("…nd then ⟦#1:TODO⟧ fix the…".to_string(), true)
        );
        assert_eq!(cut(1, 0, 20), ("short ⟦#1:TODO⟧".to_string(), false));
        assert_eq!(cut(2, 0, 20), ("This context line is…".to_string(), true));
        assert_eq!(
            cut(3, 0, 20),
            ("⟦#1:TODO⟧ … ⟦#1:TODO⟧ … ⟦#1:TODO⟧".to_string(), true)
        );
        assert_eq!(cut(0, 0, 89), (cut(0, 0, 300).0, false), "89 characters");
        // after the dedent of four spaces the line has 41 characters, `é` is one
        assert!(!cut(4, 4, 41).1);
        assert_eq!(cut(4, 4, 20), ("é ⟦#1:TODO⟧ é is he…".to_string(), true));
        assert_eq!(
            cut_line(&c, &fragments, 0, 0, Highlight::Columns, 20),
            ("…nd then TODO fix the…".to_string(), true)
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
    fn without_line_numbers_gaps_and_indentation() {
        use super::super::omit::Omit;
        let c = Content::from_bytes(b"  one two\nthree\nfour\n  five two\n");
        let text = c.text();
        let two = text.find("two").unwrap();
        let five = text.find("five").unwrap();
        let fragments = vec![frag(two..two + 3, &[]), frag(five + 5..five + 8, &[1])];
        let blocks = super::super::blocks::blocks(
            &c,
            &fragments,
            &super::super::BlockOptions {
                dedent: true,
                ..super::super::BlockOptions::default()
            },
        );
        let render_omit = |omit: Omit| {
            render(
                &c,
                &fragments,
                &blocks,
                Highlight::Markers("⟦", "⟧"),
                usize::MAX,
                omit,
            )
        };
        assert_eq!(
            render_omit(Omit::default()),
            "⇥2\n1:one ⟦two⟧\n--\n⇥2\n4:five ⟦#1:two⟧\n",
            "a fragment without ids has a marker without them"
        );
        let all = Omit::from_parts(&[OmitPart::LineNumbers, OmitPart::Gaps, OmitPart::Indent]);
        assert_eq!(render_omit(all), "one ⟦two⟧\nfive ⟦#1:two⟧\n");
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

    /// The matched lines without context with `auto`, cut at `max` characters.
    fn render_auto(c: &Content, fragments: &[Fragment], max: usize) -> String {
        let blocks =
            super::super::blocks::blocks(c, fragments, &super::super::BlockOptions::default());
        render(
            c,
            fragments,
            &blocks,
            Highlight::Auto("⟦", "⟧"),
            max,
            super::super::omit::Omit::default(),
        )
    }

    #[test]
    fn auto_writes_plain_the_lines_which_markers_outgrow() {
        let c = Content::from_bytes(b"a x b\nax b\n");
        let fragments: Vec<Fragment> = c
            .text()
            .match_indices('x')
            .map(|(i, _)| frag(i..i + 1, &[1]))
            .collect();
        assert_eq!(
            render_auto(&c, &fragments, usize::MAX),
            "1:a ⟦#1:x⟧ b\n2:ax b\n"
        );
        assert_eq!(render_plain(&c, &fragments), "1:a ⟦#1:x⟧ b\n2:a⟦#1:x⟧ b\n");
        assert_eq!(
            shown_line(&c, &fragments, 1, 0, Highlight::Auto("⟦", "⟧")),
            "ax b"
        );
    }

    #[test]
    fn auto_counts_the_lines_of_a_multiline_fragment_together() {
        let c = Content::from_bytes(b"a\nb\nlong\nline\n");
        let fragments = vec![frag(0..3, &[1]), frag(4..13, &[1])];
        assert_eq!(
            render_auto(&c, &fragments, usize::MAX),
            "1:a\n2:b\n3:⟦#1:long\n4:line⟧\n"
        );
    }

    #[test]
    fn auto_counts_the_line_as_it_is_cut() {
        let c = Content::from_bytes(format!("x x x {}\n", "b".repeat(40)).as_bytes());
        let fragments: Vec<Fragment> = c
            .text()
            .match_indices('x')
            .map(|(i, _)| frag(i..i + 1, &[1]))
            .collect();
        assert!(render_auto(&c, &fragments, usize::MAX).starts_with("1:⟦#1:x⟧ ⟦#1:x⟧ ⟦#1:x⟧ b"));
        assert_eq!(render_auto(&c, &fragments, 20), "1:x x x b…\n");
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
