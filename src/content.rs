//! The content of a file: decoded, line breaks normalized, positions of lines (QUERY.md#targets).

use std::ops::Range;

use crate::comments::comment_ranges;
use crate::languages::Language;

/// The number of first bytes in which a zero byte makes a file binary (CONFIG.md#traversal).
const BINARY_CHECK_BYTES: usize = 8 * 1024;

/// Whether a file is binary: a zero byte in its first 8K.
pub fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(BINARY_CHECK_BYTES)].contains(&0)
}

/// The encoding a file was decoded with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
    Utf32Be,
    /// Another encoding of the WHATWG list, e.g. `windows-1252`.
    Other(&'static encoding_rs::Encoding),
}

impl Encoding {
    /// The encoding of a WHATWG encoding.
    pub fn of(encoding: &'static encoding_rs::Encoding) -> Self {
        if encoding == encoding_rs::UTF_8 {
            Self::Utf8
        } else if encoding == encoding_rs::UTF_16LE {
            Self::Utf16Le
        } else if encoding == encoding_rs::UTF_16BE {
            Self::Utf16Be
        } else {
            Self::Other(encoding)
        }
    }

    /// The name in the output: the WHATWG name in lower case.
    pub fn name(self) -> std::borrow::Cow<'static, str> {
        match self {
            Self::Utf8 => "utf-8".into(),
            Self::Utf16Le => "utf-16le".into(),
            Self::Utf16Be => "utf-16be".into(),
            Self::Utf32Le => "utf-32le".into(),
            Self::Utf32Be => "utf-32be".into(),
            Self::Other(encoding) => encoding.name().to_lowercase().into(),
        }
    }
}

/// How files are decoded (CONFIG.md#encoding, CONFIG.md#traversal).
#[derive(Debug, Clone, Copy, Default)]
pub struct DecodeOptions {
    /// `binary = search`: a file with a zero byte in its first 8K is decoded like any other.
    pub binary_as_text: bool,
    /// `encoding`: every file without a BOM is decoded with it; `None` — `auto`.
    pub encoding: Option<&'static encoding_rs::Encoding>,
}

/// A decoded file.
#[derive(Debug, Clone)]
pub enum Decoded {
    /// A binary file: its content is not searched.
    Binary,
    /// Text; `errors` when invalid bytes were replaced with `�`.
    Text {
        content: Content,
        encoding: Encoding,
        errors: bool,
    },
}

/// The BOM of a file: its encoding and the content after it.
fn by_bom(bytes: &[u8]) -> Option<(Encoding, &[u8])> {
    [
        (&b"\xef\xbb\xbf"[..], Encoding::Utf8),
        (&b"\xff\xfe\0\0"[..], Encoding::Utf32Le),
        (&b"\0\0\xfe\xff"[..], Encoding::Utf32Be),
        (&b"\xff\xfe"[..], Encoding::Utf16Le),
        (&b"\xfe\xff"[..], Encoding::Utf16Be),
    ]
    .into_iter()
    .find_map(|(bom, encoding)| bytes.strip_prefix(bom).map(|rest| (encoding, rest)))
}

/// The encoding of a file without a BOM, or `None` for a binary one: the configured encoding, binary by a zero byte
/// unless it is UTF-16; with `auto` UTF-16 by the pattern of the zero bytes, binary by a zero byte, otherwise UTF-8.
fn without_bom(bytes: &[u8], options: &DecodeOptions) -> Option<Encoding> {
    let encoding = match options.encoding {
        Some(forced) => Encoding::of(forced),
        None => {
            if let Some(utf16) = utf16_by_zero_bytes(bytes) {
                return Some(utf16);
            }
            Encoding::Utf8
        }
    };
    let utf16 = matches!(encoding, Encoding::Utf16Le | Encoding::Utf16Be);
    (utf16 || options.binary_as_text || !is_binary(bytes)).then_some(encoding)
}

/// Whether `decode` takes a file for binary. `bytes` may be only the first 8K of the file.
pub fn is_binary_content(bytes: &[u8], options: &DecodeOptions) -> bool {
    by_bom(bytes).is_none() && without_bom(bytes, options).is_none()
}

/// Decodes a file: by its BOM, otherwise by the options (CONFIG.md#encoding).
pub fn decode(bytes: &[u8], options: &DecodeOptions) -> Decoded {
    let (encoding, body) = match by_bom(bytes) {
        Some(found) => found,
        None => match without_bom(bytes, options) {
            Some(encoding) => (encoding, bytes),
            None => return Decoded::Binary,
        },
    };
    let (text, errors) = match encoding {
        Encoding::Utf8 => {
            let text = String::from_utf8_lossy(body);
            let errors = matches!(text, std::borrow::Cow::Owned(_));
            (text.into_owned(), errors)
        }
        Encoding::Utf16Le => whatwg(encoding_rs::UTF_16LE, body),
        Encoding::Utf16Be => whatwg(encoding_rs::UTF_16BE, body),
        Encoding::Other(other) => whatwg(other, body),
        Encoding::Utf32Le | Encoding::Utf32Be => decode_utf32(body, encoding == Encoding::Utf32Le),
    };
    Decoded::Text {
        content: Content::from_text(text),
        encoding,
        errors,
    }
}

/// Decodes with an encoding of the WHATWG list; whether bytes were replaced.
fn whatwg(encoding: &'static encoding_rs::Encoding, body: &[u8]) -> (String, bool) {
    let (text, errors) = encoding.decode_without_bom_handling(body);
    (text.into_owned(), errors)
}

/// UTF-16 without a BOM: in the first 8K more than half of the bytes at odd positions are zero and none at even
/// positions (little endian), or the other way round (big endian).
fn utf16_by_zero_bytes(bytes: &[u8]) -> Option<Encoding> {
    let head = &bytes[..bytes.len().min(BINARY_CHECK_BYTES)];
    let pairs = head.len() / 2;
    if pairs == 0 {
        return None;
    }
    let zeros = |offset: usize| {
        head.as_chunks::<2>()
            .0
            .iter()
            .filter(|pair| pair[offset] == 0)
            .count()
    };
    let (even, odd) = (zeros(0), zeros(1));
    if even == 0 && odd * 2 > pairs {
        Some(Encoding::Utf16Le)
    } else if odd == 0 && even * 2 > pairs {
        Some(Encoding::Utf16Be)
    } else {
        None
    }
}

/// UTF-32: invalid code points and a trailing partial unit are replaced with `�`.
fn decode_utf32(bytes: &[u8], little_endian: bool) -> (String, bool) {
    let mut errors = !bytes.len().is_multiple_of(4);
    let mut text = String::with_capacity(bytes.len() / 4);
    for unit in bytes.as_chunks::<4>().0 {
        let unit = *unit;
        let code = if little_endian {
            u32::from_le_bytes(unit)
        } else {
            u32::from_be_bytes(unit)
        };
        text.push(char::from_u32(code).unwrap_or_else(|| {
            errors = true;
            char::REPLACEMENT_CHARACTER
        }));
    }
    if !bytes.len().is_multiple_of(4) {
        text.push(char::REPLACEMENT_CHARACTER);
    }
    (text, errors)
}

/// The decoded text of a file with `\n` line breaks (a `\r` before `\n` is removed) and the byte offsets where
/// its lines start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    text: String,
    line_starts: Vec<usize>,
    /// The sorted byte ranges of the comments; empty unless computed with `with_comments`.
    comments: Vec<Range<usize>>,
}

/// A line of a content: its text and where it starts, for the line scope conditions.
#[derive(Debug, Clone, Copy)]
pub struct LineRef<'a> {
    /// The line without its line break.
    pub text: &'a str,
    /// The byte offset of the line in the content.
    pub start: usize,
    content: &'a Content,
}

impl LineRef<'_> {
    /// Whether the byte `offset` of the line is inside a comment.
    pub fn in_comment(&self, offset: usize) -> bool {
        self.content.in_comment(self.start + offset)
    }
}

impl Content {
    /// Decodes UTF-8, replacing invalid bytes, and normalizes the line breaks.
    #[cfg(test)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_text(String::from_utf8_lossy(bytes).into_owned())
    }

    /// Normalizes the line breaks of a decoded text.
    pub fn from_text(decoded: String) -> Self {
        let text = if decoded.contains("\r\n") {
            decoded.replace("\r\n", "\n")
        } else {
            decoded
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
        Self {
            text,
            line_starts,
            comments: Vec::new(),
        }
    }

    /// The content with the comments of `language` known (CONFIG.md#comment-syntax).
    #[must_use]
    pub fn with_comments(mut self, language: &Language) -> Self {
        self.comments = comment_ranges(&self.text, language);
        self
    }

    /// Whether the byte `offset` is inside a comment.
    pub fn in_comment(&self, offset: usize) -> bool {
        let i = self.comments.partition_point(|r| r.end <= offset);
        self.comments.get(i).is_some_and(|r| r.start <= offset)
    }

    /// The line with the 0-based index `i` and its position.
    pub fn line_ref(&self, i: usize) -> LineRef<'_> {
        LineRef {
            text: self.line(i),
            start: self.line_starts[i],
            content: self,
        }
    }

    /// The whole normalized text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The number of lines; a trailing line break doesn't start a new line.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

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
    fn offsets_in_comments() {
        let table = crate::languages::LanguageTable::builtin();
        let rust = table.detect("a.rs", || None).unwrap();
        let c = Content::from_bytes(b"a // b\nc /* d */ e").with_comments(rust);
        let at = |s: &str| c.text().find(s).unwrap();
        assert!(!c.in_comment(at("a")));
        assert!(c.in_comment(at("//")));
        assert!(c.in_comment(at("b")));
        assert!(!c.in_comment(at("c")));
        assert!(c.in_comment(at("*/") + 1));
        assert!(!c.in_comment(at(" e")));
        assert!(c.line_ref(0).in_comment(2));
        assert!(!c.line_ref(1).in_comment(0));
        assert!(c.line_ref(1).in_comment(3));
    }

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

    fn text(bytes: &[u8]) -> (String, Encoding, bool) {
        match decode(bytes, &DecodeOptions::default()) {
            Decoded::Text {
                content,
                encoding,
                errors,
            } => (content.text().to_string(), encoding, errors),
            Decoded::Binary => panic!("binary"),
        }
    }

    #[test]
    fn byte_order_marks() {
        assert_eq!(
            text(b"\xef\xbb\xbfhi\n"),
            ("hi\n".into(), Encoding::Utf8, false)
        );
        assert_eq!(
            text(b"\xff\xfeh\0i\0"),
            ("hi".into(), Encoding::Utf16Le, false)
        );
        assert_eq!(
            text(b"\xfe\xff\0h\0i"),
            ("hi".into(), Encoding::Utf16Be, false)
        );
        assert_eq!(
            text(b"\xff\xfe\0\0h\0\0\0"),
            ("h".into(), Encoding::Utf32Le, false)
        );
        assert_eq!(
            text(b"\0\0\xfe\xff\0\0\0h"),
            ("h".into(), Encoding::Utf32Be, false)
        );
    }

    #[test]
    fn utf16_without_a_bom() {
        assert_eq!(
            text(b"h\0i\0\n\0"),
            ("hi\n".into(), Encoding::Utf16Le, false)
        );
        assert_eq!(text(b"\0h\0i"), ("hi".into(), Encoding::Utf16Be, false));
        assert!(matches!(
            decode(b"a\0\0b", &DecodeOptions::default()),
            Decoded::Binary
        ));
    }

    #[test]
    fn invalid_bytes_are_reported() {
        assert_eq!(text(b"a\xffb"), ("a\u{fffd}b".into(), Encoding::Utf8, true));
        let (t, _, errors) = text(b"\xff\xfe\0\0\0\0\x11\0");
        assert_eq!((t.as_str(), errors), ("\u{fffd}", true));
        assert_eq!(Encoding::Utf16Le.name(), "utf-16le");
    }

    #[test]
    fn binary_files_searched_as_text() {
        let as_text = DecodeOptions {
            binary_as_text: true,
            encoding: None,
        };
        match decode(b"\0\0\0\0\nlogo\n", &as_text) {
            Decoded::Text {
                content, encoding, ..
            } => {
                assert_eq!(content.line(1), "logo");
                assert_eq!(encoding, Encoding::Utf8);
            }
            Decoded::Binary => panic!("searched as text"),
        }
        assert!(matches!(
            decode(b"\0\0\0\0\nlogo\n", false),
            Decoded::Binary
        ));
    }

    #[test]
    fn binary_content_after_the_bom_and_utf16_checks() {
        let auto = DecodeOptions::default();
        assert!(is_binary_content(b"\0\0\0\0\nlogo\n", &auto));
        assert!(!is_binary_content(b"\xff\xfeh\0i\0", &auto));
        assert!(!is_binary_content(b"h\0i\0\n\0", &auto));
        assert!(!is_binary_content(b"plain text\n", &auto));
    }

    fn forced(label: &str) -> DecodeOptions {
        DecodeOptions {
            binary_as_text: false,
            encoding: encoding_rs::Encoding::for_label(label.as_bytes()),
        }
    }

    #[test]
    fn a_forced_encoding_for_files_without_a_bom() {
        let latin = forced("windows-1252");
        let Decoded::Text {
            content, encoding, ..
        } = decode(b"caf\xe9\n", &latin)
        else {
            panic!("text")
        };
        assert_eq!(content.line(0), "caf\u{e9}");
        assert_eq!(encoding.name(), "windows-1252");
        let Decoded::Text { encoding, .. } = decode(b"\xef\xbb\xbfcaf\xc3\xa9\n", &latin) else {
            panic!("text")
        };
        assert_eq!(encoding, Encoding::Utf8);
        // UTF-16 has zero bytes, other encodings are binary by them
        assert!(matches!(
            decode(b"h\0i\0", &forced("utf-16le")),
            Decoded::Text { .. }
        ));
        assert!(matches!(decode(b"h\0i\0", &latin), Decoded::Binary));
        assert!(!is_binary_content(b"h\0i\0", &forced("utf-16le")));
        assert!(is_binary_content(b"h\0i\0", &latin));
        assert_eq!(Encoding::Utf16Le.name(), "utf-16le");
    }

    #[test]
    fn invalid_utf8_is_replaced() {
        let c = Content::from_bytes(b"a\xffb\n");
        assert_eq!(c.line(0), "a\u{fffd}b");
    }
}
