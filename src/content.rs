//! The content of a file: decoded, line breaks normalized, positions of lines (QUERY.md#targets).

/// The number of first bytes in which a zero byte makes a file binary (CONFIG.md#traversal).
const BINARY_CHECK_BYTES: usize = 8 * 1024;

/// Whether a file is binary: a zero byte in its first 8K.
pub fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(BINARY_CHECK_BYTES)].contains(&0)
}

/// The decoded text of a file with `\n` line breaks (a `\r` before `\n` is removed) and the byte offsets where
/// its lines start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    text: String,
    line_starts: Vec<usize>,
}

impl Content {
    /// Decodes UTF-8, replacing invalid bytes, and normalizes the line breaks.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let decoded = String::from_utf8_lossy(bytes);
        let text = if decoded.contains("\r\n") {
            decoded.replace("\r\n", "\n")
        } else {
            decoded.into_owned()
        };
        let mut line_starts = Vec::new();
        if !text.is_empty() {
            line_starts.push(0);
            line_starts.extend(
                text.match_indices('\n')
                    .map(|(i, _)| i + 1)
                    .filter(|&start| start < text.len()),
            );
        }
        Self { text, line_starts }
    }

    /// The whole normalized text.
    pub fn text(&self) -> &str {
        &self.text
    }

    #[cfg(test)]
    /// The number of lines; a trailing line break doesn't start a new line.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    #[cfg(test)]
    /// The line with the 0-based index `i`, without its line break.
    pub fn line(&self, i: usize) -> &str {
        let (start, end) = self.line_range(i);
        &self.text[start..end]
    }

    /// The byte range of line `i` without its line break.
    pub fn line_range(&self, i: usize) -> (usize, usize) {
        let start = self.line_starts[i];
        let end = match self.line_starts.get(i + 1) {
            Some(next) => next - 1,
            None if self.text.ends_with('\n') => self.text.len() - 1,
            None => self.text.len(),
        };
        (start, end)
    }

    /// The 0-based index of the line which contains the byte `offset` (an offset at a line break belongs to the line
    /// it ends).
    pub fn line_of(&self, offset: usize) -> usize {
        self.line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1)
    }

    #[cfg(test)]
    /// The 0-based line and the 0-based column in characters of the byte `offset`.
    pub fn position(&self, offset: usize) -> (usize, usize) {
        let line = self.line_of(offset);
        let start = self.line_starts.get(line).copied().unwrap_or(0);
        (line, self.text[start..offset].chars().count())
    }
}

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
    fn binary_files_have_a_zero_byte_in_the_first_8k() {
        assert!(is_binary(b"\0abc"));
        let mut late = vec![b'a'; 8191];
        late.push(0);
        assert!(is_binary(&late));
        let mut too_late = vec![b'a'; 8192];
        too_late.push(0);
        assert!(!is_binary(&too_late));
        assert!(!is_binary(b"text\n"));
    }

    #[test]
    fn invalid_utf8_is_replaced() {
        let c = Content::from_bytes(b"a\xffb\n");
        assert_eq!(c.line(0), "a\u{fffd}b");
    }
}
