//! The comments of a file by the comment syntax of its language (CONFIG.md#comment-syntax).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::LanguageTable;

    fn comments(file: &str, text: &str) -> Vec<String> {
        let table = LanguageTable::builtin();
        let lang = table.detect(file, || None).unwrap();
        comment_ranges(text, lang)
            .into_iter()
            .map(|r| text[r].to_string())
            .collect()
    }

    #[test]
    fn line_and_block_comments() {
        assert_eq!(
            comments("a.rs", "a // b\nc /* d\ne */ f"),
            ["// b", "/* d\ne */"]
        );
        assert_eq!(comments("a.py", "x = 1  # y\n#z"), ["# y", "#z"]);
    }

    #[test]
    fn nested_block_comments() {
        assert_eq!(
            comments("a.rs", "/* a /* b */ c */ d"),
            ["/* a /* b */ c */"]
        );
        assert_eq!(comments("a.c", "/* a /* b */ c */ d"), ["/* a /* b */"]);
    }

    #[test]
    fn strings_hide_markers() {
        assert_eq!(comments("a.rs", "let s = \"// x\"; // y"), ["// y"]);
        assert_eq!(comments("a.rs", "let s = \"a\\\"// x\"; // y"), ["// y"]);
        assert_eq!(comments("a.py", "x = '# no' # yes"), ["# yes"]);
        assert_eq!(comments("a.js", "let s = `a\n// b`; // c"), ["// c"]);
        assert_eq!(comments("a.py", "x = 'open\n# comment"), ["# comment"]);
    }

    #[test]
    fn the_longest_marker_wins() {
        assert_eq!(
            comments("a.lua", "x --[[ a\nb ]] y -- z"),
            ["--[[ a\nb ]]", "-- z"]
        );
    }

    #[test]
    fn unterminated_block_comments_run_to_the_end() {
        assert_eq!(comments("a.c", "a /* b"), ["/* b"]);
    }
}
