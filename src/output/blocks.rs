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
            lines: (lo..=hi)
                .map(|index| BlockLine {
                    index,
                    matched: matched.binary_search(&index).is_ok(),
                })
                .collect(),
        })
        .collect()
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
        };
        assert_eq!(blocks(&c, &fragments, &two).len(), 1);
        let first = [at(&c, 0)];
        assert_eq!(
            shape(&blocks(
                &c,
                &first,
                &BlockOptions {
                    before: 3,
                    after: 1
                }
            )),
            [vec![(1, true), (2, false)]]
        );
        let end = [at(&c, 8)];
        assert_eq!(
            shape(&blocks(
                &c,
                &end,
                &BlockOptions {
                    before: 0,
                    after: 5
                }
            )),
            [vec![(9, true)]]
        );
    }
}
