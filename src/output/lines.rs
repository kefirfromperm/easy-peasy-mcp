//! Matched lines with fragments in the text format: blocks, markers (OUTPUT.md#fragments, OUTPUT.md#highlight).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::query::ast::CondId;

    fn frag(range: std::ops::Range<usize>, ids: &[u32]) -> Fragment {
        Fragment {
            range,
            ids: ids.iter().map(|n| CondId::Number(*n)).collect(),
        }
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
