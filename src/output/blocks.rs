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
    /// The context after every matched line goes on to the end of its block, instead of `after` lines
    /// (OUTPUT.md#block-context).
    pub after_block: bool,
    /// Keep the empty and whitespace-only context lines.
    pub blank_context: bool,
    /// Remove the common leading whitespace of every block.
    pub dedent: bool,
    /// Write the matches instead of their lines (OUTPUT.md#only-the-matches); the blocks are then made without
    /// context and dedent.
    pub only_match: bool,
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
    /// With the block context: the last line of the block, blank lines too.
    pub end: Option<usize>,
    /// The lines of the file at the end of the block left out by a limit, from the first one to `end`.
    pub more: usize,
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
            end: None,
            more: 0,
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

/// The blocks without the lines from the line `until` on; a block left without lines is dropped.
pub fn before(blocks: Vec<Block>, until: Option<usize>) -> Vec<Block> {
    let Some(until) = until else {
        return blocks;
    };
    blocks
        .into_iter()
        .filter_map(|mut block| {
            block.lines.retain(|line| line.index < until);
            (!block.lines.is_empty()).then_some(block)
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
            groups: Vec::new(),
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
                groups: Vec::new(),
            }
        };
        let ac = [late(0, 4), late(3, 6)];
        let context = BlockOptions {
            before: 3,
            after: 0,
            blank_context: true,
            dedent: true,
            ..BlockOptions::default()
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

    const RUST: &[u8] = b"use std::fs;\n\npub fn load(path: &str) -> String {\n    let raw = read(path);\n    if raw.is_empty() {\n        return String::new();\n    }\n    raw\n}\n\nfn read(path: &str) -> String {\n    fs::read_to_string(path).unwrap_or_default()\n}\n\npub fn parse(\n    text: &str,\n) -> Vec<String> {\n    text.lines()\n        .map(str::to_string)\n        .collect()\n}\n";

    const BLOCK: BlockOptions = BlockOptions {
        before: 0,
        after: 0,
        after_block: true,
        blank_context: false,
        dedent: false,
        only_match: false,
    };

    fn block_lines(content: &Content, lines: &[usize], options: &BlockOptions) -> Vec<Vec<usize>> {
        let fragments: Vec<Fragment> = lines.iter().map(|&l| at(content, l - 1)).collect();
        blocks(content, &fragments, options)
            .iter()
            .map(|b| b.lines.iter().map(|l| l.index + 1).collect())
            .collect()
    }

    #[test]
    fn the_block_of_a_function() {
        let c = Content::from_bytes(RUST);
        assert_eq!(block_lines(&c, &[3], &BLOCK), [vec![3, 4, 5, 6, 7, 8, 9]]);
        assert_eq!(block_lines(&c, &[5], &BLOCK), [vec![5, 6, 7]], "nested");
        assert_eq!(block_lines(&c, &[4], &BLOCK), [vec![4]], "no block");
        assert_eq!(
            block_lines(&c, &[3, 5], &BLOCK),
            [vec![3, 4, 5, 6, 7, 8, 9]]
        );
        assert_eq!(
            block_lines(&c, &[9, 13], &BLOCK),
            [vec![9], vec![13]],
            "closing lines open no block"
        );
        assert_eq!(
            block_lines(&c, &[15], &BLOCK),
            [vec![15, 16, 17, 18, 19, 20, 21]],
            "a closing line which opens again"
        );
        let before = BlockOptions { before: 2, ..BLOCK };
        assert_eq!(block_lines(&c, &[5], &before), [vec![3, 4, 5, 6, 7]]);
        let blocks = blocks(&c, &[at(&c, 2)], &BLOCK);
        assert_eq!(blocks[0].end, Some(8));
    }

    #[test]
    fn a_block_without_a_closing_line() {
        let python = Content::from_bytes(
            b"import os\n\n\ndef load(path):\n    with open(path) as f:\n        data = f.read()\n\n    return data.strip()\n\n\ndef main():\n    print(load(\"x\"))\n",
        );
        assert_eq!(block_lines(&python, &[4], &BLOCK), [vec![4, 5, 6, 8]]);
        let kept = BlockOptions {
            blank_context: true,
            ..BLOCK
        };
        assert_eq!(
            block_lines(&python, &[4], &kept),
            [vec![4, 5, 6, 7, 8]],
            "the blank lines at the end are not in the block"
        );
        assert_eq!(
            block_lines(&python, &[11], &BLOCK),
            [vec![11, 12]],
            "the last line"
        );
        let markdown = Content::from_bytes(b"# Demo\n\n## Usage\n\nRun the loader.\n");
        assert_eq!(block_lines(&markdown, &[3], &BLOCK), [vec![3]]);
    }

    #[test]
    fn closing_lines() {
        for (text, closing) in [
            ("}", true),
            ("});", true),
            (") -> X {", true),
            ("]", true),
            ("</div>", true),
            ("end", true),
            ("end)", true),
            ("fi", true),
            ("done", true),
            ("esac", true),
            ("ending()", false),
            ("x = 1", false),
            ("else:", false),
        ] {
            let c = Content::from_bytes(format!("a\n  b\n{text}\nc\n").as_bytes());
            let want = if closing { vec![1, 2, 3] } else { vec![1, 2] };
            assert_eq!(block_lines(&c, &[1], &BLOCK), [want], "{text}");
        }
        let tabs = Content::from_bytes(b"\tif x {\n\t\ty\n\t}\n}\n");
        assert_eq!(block_lines(&tabs, &[1], &BLOCK), [vec![1, 2, 3]]);
    }

    #[test]
    fn the_rest_of_a_cut_block() {
        let c = Content::from_bytes(RUST);
        let cut = before(blocks(&c, &[at(&c, 14)], &BLOCK), Some(19));
        assert_eq!(cut[0].lines.len(), 5);
        assert_eq!(cut[0].more, 2, "lines 20 and 21");
        let whole = before(blocks(&c, &[at(&c, 14)], &BLOCK), Some(21));
        assert_eq!(whole[0].more, 0);
        let numbers = BlockOptions {
            after: 6,
            after_block: false,
            ..BLOCK
        };
        let cut = before(blocks(&c, &[at(&c, 14)], &numbers), Some(19));
        assert_eq!(cut[0].more, 0, "only the block context says so");
    }
}
