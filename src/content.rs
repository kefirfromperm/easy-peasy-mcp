//! The content of a file: decoded, line breaks normalized, positions of lines (QUERY.md#targets).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crlf_is_normalized() {
        let c = Content::from_bytes(b"first\r\nsecond\r\nthird");
        assert_eq!(c.text(), "first\nsecond\nthird");
        assert_eq!(c.line_count(), 3);
        assert_eq!(c.line(1), "second");
        assert_eq!(c.line(2), "third");
    }

    #[test]
    fn a_trailing_line_break_ends_the_last_line() {
        let c = Content::from_bytes(b"a\nb\n");
        assert_eq!(c.line_count(), 2);
        assert_eq!(c.line(1), "b");
        let empty = Content::from_bytes(b"");
        assert_eq!(empty.line_count(), 0);
    }

    #[test]
    fn positions_in_characters() {
        let c = Content::from_bytes("é = 1\nab".as_bytes());
        let offset = c.text().find('=').unwrap();
        assert_eq!(c.position(offset), (0, 2));
        assert_eq!(c.position(c.text().find('b').unwrap()), (1, 1));
        assert_eq!(c.line_of(c.text().len()), 1);
    }

    #[test]
    fn invalid_utf8_is_replaced() {
        let c = Content::from_bytes(b"a\xffb\n");
        assert_eq!(c.line(0), "a\u{fffd}b");
    }
}
