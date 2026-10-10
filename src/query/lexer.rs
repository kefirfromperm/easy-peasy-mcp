//! Splits the query text into tokens with their byte ranges (QUERY.md#grammar).
//!
//! The lexer never fails: characters which start no token become [`Tok::Invalid`] and an unclosed string becomes
//! [`Tok::Unterminated`], so the parser reports the first error from the start of the query.

use std::ops::Range;

/// The quote of a string literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
    Single,
    Double,
    Raw,
}

impl Quote {
    /// The quote character.
    pub fn char(self) -> char {
        match self {
            Self::Single => '\'',
            Self::Double => '"',
            Self::Raw => '`',
        }
    }
}

/// The kind and content of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tok {
    /// Letters, digits, `_` and `-`, starting with a letter or `_`: targets, operators like `has`, keywords, values.
    Word(String),
    /// A string literal with its escapes resolved.
    Str {
        value: String,
        quote: Quote,
    },
    /// A string literal without its closing quote; it runs to the end of the query.
    Unterminated(Quote),
    /// A run of the operator characters `=`, `!`, `<`, `>`, `~`.
    Op(String),
    /// A number, size or date: starts with a digit, or with `-` and a digit.
    Value(String),
    /// `..`
    Range,
    LParen,
    RParen,
    Comma,
    /// `/` and the letters after it, without the `/`.
    Modifier(String),
    /// `@` and the label name, without the `@`.
    Label(String),
    /// A character which starts no token.
    Invalid(char),
}

/// A token and its byte range in the query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tok: Tok,
    pub span: Range<usize>,
}

/// The tokens of a query and the byte ranges of its `#` comments.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Lexed {
    pub tokens: Vec<Token>,
    pub comments: Vec<Range<usize>>,
}

fn is_word_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_op_char(c: char) -> bool {
    matches!(c, '=' | '!' | '<' | '>' | '~')
}

/// Splits `query` into tokens.
pub fn lex(query: &str) -> Lexed {
    let mut out = Lexed::default();
    let chars: Vec<(usize, char)> = query.char_indices().collect();
    let offset = |i: usize| chars.get(i).map_or(query.len(), |&(o, _)| o);
    let mut i = 0;
    while let Some(&(start, c)) = chars.get(i) {
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '#' {
            i = scan(&chars, i, |c, _| c != '\n');
            out.comments.push(start..offset(i));
            continue;
        }
        let (tok, next) = lex_token(query, &chars, i);
        i = next;
        out.tokens.push(Token {
            tok,
            span: start..offset(i),
        });
    }
    out
}

/// The index after the run of characters from `chars[from]` for which `keep(char, next char)` holds.
fn scan(chars: &[(usize, char)], from: usize, keep: impl Fn(char, Option<char>) -> bool) -> usize {
    let mut j = from;
    while let Some(&(_, c)) = chars.get(j) {
        if !keep(c, chars.get(j + 1).map(|&(_, n)| n)) {
            break;
        }
        j += 1;
    }
    j
}

/// Reads the token starting at `chars[i]`, which is not whitespace or a comment; returns it and the index after it.
fn lex_token(query: &str, chars: &[(usize, char)], i: usize) -> (Tok, usize) {
    let offset = |i: usize| chars.get(i).map_or(query.len(), |&(o, _)| o);
    let at = |i: usize| chars.get(i).map(|&(_, c)| c);
    let c = chars[i].1;
    let text = |j: usize| query[offset(i)..offset(j)].to_string();
    if is_word_start(c) {
        let j = scan(chars, i + 1, |c, _| is_word_char(c));
        return (Tok::Word(text(j)), j);
    }
    if c.is_ascii_digit() || (c == '-' && at(i + 1).is_some_and(|d| d.is_ascii_digit())) {
        let j = scan(chars, i + 1, |d, next| {
            let range_follows = d == '.' && next == Some('.');
            !range_follows && (d.is_alphanumeric() || matches!(d, '.' | ':' | '+' | '-' | '_'))
        });
        return (Tok::Value(text(j)), j);
    }
    if is_op_char(c) {
        let j = scan(chars, i + 1, |c, _| is_op_char(c));
        return (Tok::Op(text(j)), j);
    }
    match c {
        '\'' => lex_string(chars, i, Quote::Single),
        '"' => lex_string(chars, i, Quote::Double),
        '`' => lex_string(chars, i, Quote::Raw),
        '(' => (Tok::LParen, i + 1),
        ')' => (Tok::RParen, i + 1),
        ',' => (Tok::Comma, i + 1),
        '.' if at(i + 1) == Some('.') => (Tok::Range, i + 2),
        '/' => {
            let j = scan(chars, i + 1, |c, _| c.is_alphabetic());
            (
                Tok::Modifier(query[offset(i + 1)..offset(j)].to_string()),
                j,
            )
        }
        '@' if at(i + 1).is_some_and(is_word_start) => {
            let j = scan(chars, i + 2, |c, _| is_word_char(c));
            (Tok::Label(query[offset(i + 1)..offset(j)].to_string()), j)
        }
        other => (Tok::Invalid(other), i + 1),
    }
}

/// Reads a string starting at the quote `chars[start]`; returns the token and the index after it.
fn lex_string(chars: &[(usize, char)], start: usize, quote: Quote) -> (Tok, usize) {
    let q = quote.char();
    let mut value = String::new();
    let mut i = start + 1;
    while let Some(&(_, c)) = chars.get(i) {
        if c == q {
            return (Tok::Str { value, quote }, i + 1);
        }
        if c == '\\' && quote != Quote::Raw {
            match chars.get(i + 1).map(|&(_, c)| c) {
                Some('\'') => value.push('\''),
                Some('"') => value.push('"'),
                Some('\\') => value.push('\\'),
                Some('n') => value.push('\n'),
                Some('t') => value.push('\t'),
                _ => {
                    value.push('\\');
                    i += 1;
                    continue;
                }
            }
            i += 2;
            continue;
        }
        value.push(c);
        i += 1;
    }
    (Tok::Unterminated(quote), i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(q: &str) -> Vec<Tok> {
        lex(q).tokens.into_iter().map(|t| t.tok).collect()
    }

    fn word(s: &str) -> Tok {
        Tok::Word(s.into())
    }

    fn single(s: &str) -> Tok {
        Tok::Str {
            value: s.into(),
            quote: Quote::Single,
        }
    }

    #[test]
    fn condition() {
        assert_eq!(
            toks("name ~ '*.rs'"),
            [word("name"), Tok::Op("~".into()), single("*.rs")]
        );
        assert_eq!(
            toks("NAME~'a'"),
            [word("NAME"), Tok::Op("~".into()), single("a")]
        );
    }

    #[test]
    fn operators_are_runs() {
        assert_eq!(
            toks("= != <= ~~ =~ =="),
            ["=", "!=", "<=", "~~", "=~", "=="].map(|o| Tok::Op(o.into()))
        );
    }

    #[test]
    fn string_escapes() {
        assert_eq!(toks(r"'it\'s'"), [single("it's")]);
        assert_eq!(toks(r#"'\"\\\n\t'"#), [single("\"\\\n\t")]);
        assert_eq!(toks(r"'\d+'"), [single(r"\d+")]);
        assert_eq!(
            toks(r#""it's""#),
            [Tok::Str {
                value: "it's".into(),
                quote: Quote::Double
            }]
        );
        assert_eq!(
            toks(r"`a\'b`"),
            [Tok::Str {
                value: r"a\'b".into(),
                quote: Quote::Raw
            }]
        );
    }

    #[test]
    fn unterminated_strings_run_to_the_end() {
        let lexed = lex("name = 'abc");
        assert_eq!(
            lexed.tokens[2],
            Token {
                tok: Tok::Unterminated(Quote::Single),
                span: 7..11
            }
        );
        assert_eq!(toks(r"'a\'"), [Tok::Unterminated(Quote::Single)]);
    }

    #[test]
    fn values_ranges_and_dates() {
        assert_eq!(
            toks("-7d..-1d 1.5M 2026-10-01T14:30:00+02:00 10"),
            [
                Tok::Value("-7d".into()),
                Tok::Range,
                Tok::Value("-1d".into()),
                Tok::Value("1.5M".into()),
                Tok::Value("2026-10-01T14:30:00+02:00".into()),
                Tok::Value("10".into()),
            ]
        );
    }

    #[test]
    fn brackets_modifiers_labels_comments() {
        let lexed = lex("line(text has 'a' /i/w @x-1, 3) # note\nor");
        assert_eq!(
            lexed
                .tokens
                .iter()
                .map(|t| t.tok.clone())
                .collect::<Vec<_>>(),
            [
                word("line"),
                Tok::LParen,
                word("text"),
                word("has"),
                single("a"),
                Tok::Modifier("i".into()),
                Tok::Modifier("w".into()),
                Tok::Label("x-1".into()),
                Tok::Comma,
                Tok::Value("3".into()),
                Tok::RParen,
                word("or"),
            ]
        );
        assert_eq!(lexed.comments, vec![32..38]);
    }

    #[test]
    fn invalid_characters() {
        assert_eq!(
            toks("a && b"),
            [word("a"), Tok::Invalid('&'), Tok::Invalid('&'), word("b")]
        );
        assert_eq!(toks("@1"), [Tok::Invalid('@'), Tok::Value("1".into())]);
    }

    #[test]
    fn spans_are_byte_ranges() {
        let lexed = lex("name = 'é' x");
        assert_eq!(lexed.tokens[2].span, 7..11);
        assert_eq!(lexed.tokens[3].span, 12..13);
    }
}
