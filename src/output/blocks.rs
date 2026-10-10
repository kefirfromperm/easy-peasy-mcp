//! The matched lines of a file with their context, in blocks (OUTPUT.md#blocks).

use super::lines::lines_of;
use crate::content::Content;

use super::Fragment;

/// How the blocks of a file are made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BlockOptions {
    /// Context lines before every matched line.
    pub before: usize,
    /// Context lines after every matched line.
    pub after: usize,
    /// Keep the empty and whitespace-only context lines.
    pub blank_context: bool,
    /// Remove the common leading whitespace of every block.
    pub dedent: bool,
}

/// A line of a block: its 0-based index in the content and whether it is matched (or context).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockLine {
    pub index: usize,
    pub matched: bool,
}

/// A run of consecutive lines: matched lines and their context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The bytes of leading whitespace removed from every line (`dedent`).
    pub indent: usize,
    pub lines: Vec<BlockLine>,
}

/// The blocks of the matched lines of the fragments with their context; the context of two matched lines which
/// touches or overlaps makes one block.
pub fn blocks(content: &Content, fragments: &[Fragment], options: &BlockOptions) -> Vec<Block> {
    let matched = lines_of(content, fragments);
    let last = content.line_count().saturating_sub(1);
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for &m in &matched {
        let lo = m.saturating_sub(options.before);
        let hi = (m + options.after).min(last).max(m);
        match ranges.last_mut() {
            Some((_, end)) if lo <= *end + 1 => *end = (*end).max(hi),
            _ => ranges.push((lo, hi)),
        }
    }
    ranges
        .into_iter()
        .map(|(lo, hi)| Block {
            indent: 0,
            lines: (lo..=hi)
                .map(|index| BlockLine {
                    index,
                    matched: matched.binary_search(&index).is_ok(),
                })
                .filter(|line| {
                    line.matched
                        || options.blank_context
                        || !content.line(line.index).trim().is_empty()
                })
                .collect(),
        })
        .map(|mut block| {
            if options.dedent {
                block.indent = indent(content, &block.lines, fragments);
            }
            block
        })
        .collect()
}

/// The common leading spaces and tabs of the lines of a block which are not blank, in bytes, cut back to the start of
/// a fragment which starts in them.
fn indent(content: &Content, lines: &[BlockLine], fragments: &[Fragment]) -> usize {
    let mut common: Option<&str> = None;
    for line in lines {
        let text = content.line(line.index);
        if text.trim().is_empty() {
            continue;
        }
        let lead = &text[..text.len() - text.trim_start_matches([' ', '\t']).len()];
        common = Some(match common {
            None => lead,
            Some(c) => {
                let same = c
                    .bytes()
                    .zip(lead.bytes())
                    .take_while(|(a, b)| a == b)
                    .count();
                &c[..same]
            }
        });
    }
    let mut indent = common.map_or(0, str::len);
    for f in fragments {
        let i = content.line_of(f.range.start);
        if lines.iter().any(|l| l.index == i) {
            let (start, _) = content.line_range(i);
            indent = indent.min(f.range.start - start);
        }
    }
    indent
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::ast::CondId;

    fn at(content: &Content, line: usize) -> Fragment {
        let (start, _) = content.line_range(line);
        Fragment {
            range: start..start + 1,
            ids: vec![(1, CondId::Number(1))],
            escape: None,
        }
    }

    fn shape(blocks: &[Block]) -> Vec<Vec<(usize, bool)>> {
        blocks
            .iter()
            .map(|b| b.lines.iter().map(|l| (l.index + 1, l.matched)).collect())
            .collect()
    }

    #[test]
    fn context_before_and_after() {
        let c = Content::from_bytes(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n");
        let fragments = [at(&c, 2), at(&c, 7)];
        let none = BlockOptions::default();
        assert_eq!(
            shape(&blocks(&c, &fragments, &none)),
            [vec![(3, true)], vec![(8, true)]]
        );
        let one = BlockOptions {
            before: 1,
            after: 1,
            ..BlockOptions::default()
        };
        assert_eq!(
            shape(&blocks(&c, &fragments, &one)),
            [
                vec![(2, false), (3, true), (4, false)],
                vec![(7, false), (8, true), (9, false)]
            ]
        );
        // touching context: 3+2 = 5 and 8-2 = 6
        let two = BlockOptions {
            before: 2,
            after: 2,
            ..BlockOptions::default()
        };
        assert_eq!(blocks(&c, &fragments, &two).len(), 1);
        let first = [at(&c, 0)];
        assert_eq!(
            shape(&blocks(
                &c,
                &first,
                &BlockOptions {
                    before: 3,
                    after: 1,
                    ..BlockOptions::default()
                }
            )),
            [vec![(1, true), (2, false)]]
        );
        let end = [at(&c, 8)];
        let blank = Content::from_bytes(b"a\n\nb\n \t\nc\n");
        let around = [at(&blank, 2)];
        let wide = BlockOptions {
            before: 2,
            after: 2,
            ..BlockOptions::default()
        };
        assert_eq!(
            shape(&blocks(&blank, &around, &wide)),
            [vec![(1, false), (3, true), (5, false)]]
        );
        let kept = BlockOptions {
            blank_context: true,
            ..wide
        };
        assert_eq!(blocks(&blank, &around, &kept)[0].lines.len(), 5);
        assert_eq!(
            shape(&blocks(
                &c,
                &end,
                &BlockOptions {
                    before: 0,
                    after: 5,
                    ..BlockOptions::default()
                }
            )),
            [vec![(9, true)]]
        );
    }

    #[test]
    fn the_indent_of_a_block() {
        let indented = Content::from_bytes(b"    a\n  \t\n\tb\n      c\n\n");
        let all: Vec<Fragment> = (0..4).map(|i| at(&indented, i)).collect();
        let dedent = BlockOptions {
            dedent: true,
            ..BlockOptions::default()
        };
        // a, the blank line and c: two spaces in common; the tab of b has none
        assert_eq!(
            blocks(&indented, &all[..1], &dedent)[0].indent,
            0,
            "a fragment at the line start"
        );
        let late = |line: usize, offset: usize| {
            let (start, _) = indented.line_range(line);
            Fragment {
                range: start + offset..start + offset + 1,
                ids: vec![(1, CondId::Number(1))],
                escape: None,
            }
        };
        let ac = [late(0, 4), late(3, 6)];
        let context = BlockOptions {
            before: 3,
            after: 0,
            blank_context: true,
            dedent: true,
        };
        assert_eq!(
            blocks(&indented, &ac[1..], &context)[0].indent,
            0,
            "b starts with a tab"
        );
        assert_eq!(blocks(&indented, &ac[..1], &dedent)[0].indent, 4);
        let near = [late(3, 2)];
        assert_eq!(
            blocks(&indented, &near, &dedent)[0].indent,
            2,
            "the fragment starts at 2"
        );
    }
}
