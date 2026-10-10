//! The comments of a file by the comment syntax of its language (CONFIG.md#comment-syntax).

use std::ops::Range;

use crate::languages::Language;

/// What starts at a position of the text outside of strings and comments.
enum Marker<'l> {
    String(&'l str),
    Line,
    Block(&'l str, &'l str),
}

/// The byte ranges of the comments of `text`, markers included, sorted.
pub fn comment_ranges(text: &str, language: &Language) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        let Some((marker, len)) = marker_at(&text[pos..], language) else {
            pos += text[pos..].chars().next().map_or(1, char::len_utf8);
            continue;
        };
        let start = pos;
        pos += len;
        match marker {
            Marker::String(delimiter) => pos = string_end(text, pos, delimiter),
            Marker::Line => {
                pos = text[pos..].find('\n').map_or(text.len(), |i| pos + i);
                ranges.push(start..pos);
            }
            Marker::Block(open, close) => {
                pos = block_end(text, pos, open, close, language.nested_block_comments);
                ranges.push(start..pos);
            }
        }
    }
    ranges
}

/// The longest marker at the start of `rest` and its length.
fn marker_at<'l>(rest: &str, language: &'l Language) -> Option<(Marker<'l>, usize)> {
    let strings = language
        .strings
        .iter()
        .map(|d| (Marker::String(d.as_str()), d.len(), d.as_str()));
    let lines = language
        .line_comments
        .iter()
        .map(|m| (Marker::Line, m.len(), m.as_str()));
    let blocks = language.block_comments.iter().map(|(open, close)| {
        (
            Marker::Block(open.as_str(), close.as_str()),
            open.len(),
            open.as_str(),
        )
    });
    strings
        .chain(lines)
        .chain(blocks)
        .filter(|(_, len, text)| *len > 0 && rest.starts_with(text))
        .max_by_key(|(_, len, _)| *len)
        .map(|(marker, len, _)| (marker, len))
}

/// The position after the string which starts before `pos`: after its unescaped delimiter, or at the line end for
/// `"` and `'`, or at the end of the text.
fn string_end(text: &str, mut pos: usize, delimiter: &str) -> usize {
    let single_line = delimiter == "\"" || delimiter == "'";
    while pos < text.len() {
        let rest = &text[pos..];
        if let Some(escaped) = rest.strip_prefix('\\') {
            pos += 1 + escaped.chars().next().map_or(0, char::len_utf8);
        } else if rest.starts_with(delimiter) {
            return pos + delimiter.len();
        } else if single_line && rest.starts_with('\n') {
            return pos;
        } else {
            pos += rest.chars().next().map_or(1, char::len_utf8);
        }
    }
    text.len()
}

/// The position after the block comment which starts before `pos`.
fn block_end(text: &str, mut pos: usize, open: &str, close: &str, nested: bool) -> usize {
    let mut depth = 1;
    while pos < text.len() {
        let rest = &text[pos..];
        if rest.starts_with(close) {
            pos += close.len();
            depth -= 1;
            if depth == 0 {
                return pos;
            }
        } else if nested && rest.starts_with(open) {
            pos += open.len();
            depth += 1;
        } else {
            pos += rest.chars().next().map_or(1, char::len_utf8);
        }
    }
    text.len()
}

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
