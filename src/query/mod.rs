//! The query language (QUERY.md): text → tokens → syntax tree → condition ids and legend → compiled filter.

pub mod ast;
mod lexer;
mod parser;
mod subset;
mod suggest;
pub mod wildcard;

use std::ops::Range;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use ast::{CondId, Expr, NodeKind, Op, Target, Value};
use lexer::Token;

pub(crate) use suggest::did_you_mean;

use crate::error::{ErrorCode, QueryError};

/// Settings which change how the query is compiled.
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    /// `path_case`: whether `path` and `name` are compared case-insensitively.
    pub case_insensitive_paths: bool,
}

/// How a text value is matched: a regex over the NFC form of the value.
#[derive(Debug, Clone)]
pub struct TextMatcher(Regex);

impl TextMatcher {
    /// Whether `value` matches.
    pub fn is_match(&self, value: &str) -> bool {
        self.0.is_match(value)
    }
}

/// The file values a text condition can look at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTarget {
    /// The file name.
    Name,
    /// The path relative to the working directory, `/` as the separator.
    Path,
}

/// The compiled query, evaluated for every file.
#[derive(Debug, Clone)]
pub enum Filter {
    /// A `name` or `path` condition, on the NFC form of the value.
    Text {
        id: CondId,
        target: TextTarget,
        matcher: TextMatcher,
    },
}

/// A parsed and compiled query.
#[derive(Debug, Clone)]
pub struct Query {
    /// Condition id → the condition as written, in the order of appearance (OUTPUT.md#condition-ids).
    pub legend: Vec<(CondId, String)>,
    /// `None` for the empty query, which matches every file.
    pub filter: Option<Filter>,
}

/// Parses and checks `text`; the first error from the start of the query is returned.
pub fn parse(text: &str, options: ParseOptions) -> Result<Query, QueryError> {
    let lexed = lexer::lex(text);
    let mut expr = parser::parse(text, &lexed)?;
    let mut legend = Vec::new();
    if let Some(expr) = expr.as_mut() {
        assign_ids(expr);
        expr.visit_nodes(&mut |node| {
            legend.push((
                node.id.clone(),
                legend_text(text, &lexed.tokens, node.tokens.clone()),
            ));
        });
    }
    let mut errors: Vec<QueryError> = Vec::new();
    if let Some(expr) = &expr {
        validate(text, expr, options, &mut errors);
    }
    errors.extend(subset::check(text, &lexed));
    if let Some(first) = errors.into_iter().min_by_key(|e| e.span.start) {
        return Err(first);
    }
    let filter = expr.map(|expr| {
        compile(&expr, options).expect("the subset check allows only one name or path condition")
    });
    Ok(Query { legend, filter })
}

/// Gives every node its id: its label, or the next number for nodes without a label.
fn assign_ids(expr: &mut Expr) {
    let mut n = 0;
    expr.visit_nodes_mut(&mut |node| {
        node.id = if let Some(label) = &node.label {
            CondId::Label(label.text.clone())
        } else {
            n += 1;
            CondId::Number(n)
        };
    });
}

/// The condition as written: its tokens without labels, a gap between two tokens written as one space.
fn legend_text(query: &str, tokens: &[Token], range: Range<usize>) -> String {
    let mut out = String::new();
    for i in range {
        let token = &tokens[i];
        if matches!(token.tok, lexer::Tok::Label(_)) {
            continue;
        }
        if !out.is_empty() && i > 0 && tokens[i - 1].span.end != token.span.start {
            out.push(' ');
        }
        out.push_str(&query[token.span.clone()]);
    }
    out
}

/// Checks the values of all conditions, also of those outside of the implemented subset.
fn validate(text: &str, expr: &Expr, options: ParseOptions, errors: &mut Vec<QueryError>) {
    expr.visit_nodes(&mut |node| {
        if let NodeKind::Condition(c) = &node.kind
            && let (Some((Op::Wildcard, _)), Value::Str(s), Target::Name | Target::Path) =
                (&c.op, &c.value, c.target)
            && let Err(e) = wildcard::compile(&nfc(&s.value), options.case_insensitive_paths)
        {
            let written = &text[s.span.clone()];
            errors.push(
                QueryError::new(
                    ErrorCode::InvalidWildcard,
                    format!("invalid wildcard {written}"),
                    s.span.clone(),
                )
                .hint(e.0),
            );
        }
    });
}

/// The NFC form of a string.
pub fn nfc(s: &str) -> String {
    if unicode_normalization::is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

fn compile(expr: &Expr, options: ParseOptions) -> Option<Filter> {
    let Expr::Node(node) = expr else { return None };
    let NodeKind::Condition(c) = &node.kind else {
        return None;
    };
    let (Some((op, _)), Value::Str(s)) = (&c.op, &c.value) else {
        return None;
    };
    let value = nfc(&s.value);
    let insensitive = options.case_insensitive_paths;
    let regex = if *op == Op::Wildcard {
        wildcard::compile(&value, insensitive).ok()?
    } else {
        let escaped = regex::escape(&value);
        let pattern = match op {
            Op::Eq => format!("^{escaped}$"),
            Op::Starts => format!("^{escaped}"),
            Op::Ends => format!("{escaped}$"),
            _ => escaped,
        };
        let flags = if insensitive { "(?i)" } else { "" };
        Regex::new(&format!("{flags}{pattern}")).expect("an escaped string is a valid regex")
    };
    let matcher = TextMatcher(regex);
    let target = match c.target {
        Target::Path => TextTarget::Path,
        _ => TextTarget::Name,
    };
    Some(Filter::Text {
        id: node.id.clone(),
        target,
        matcher,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SENSITIVE: ParseOptions = ParseOptions {
        case_insensitive_paths: false,
    };

    fn legend(q: &str) -> Vec<String> {
        let lexed = lexer::lex(q);
        let mut expr = parser::parse(q, &lexed).unwrap().unwrap();
        let mut out = Vec::new();
        expr.visit_nodes_mut(&mut |n| out.push(legend_text(q, &lexed.tokens, n.tokens.clone())));
        out
    }

    #[test]
    fn legend_keeps_the_spelling_and_squeezes_gaps() {
        assert_eq!(legend("NAME   ~\n'*.rs'"), ["NAME ~ '*.rs'"]);
        assert_eq!(
            legend("line( text has 'a' @x )"),
            ["line( text has 'a' )", "text has 'a'"]
        );
        assert_eq!(
            legend("line(text has 'unwrap(' @unwrap or text has 'expect(' @expect)"),
            [
                "line(text has 'unwrap(' or text has 'expect(')",
                "text has 'unwrap('",
                "text has 'expect('"
            ]
        );
        assert_eq!(legend("name = 'a' # c\n/i"), ["name = 'a' /i"]);
    }

    #[test]
    fn ids_number_unlabelled_conditions() {
        let q = "text has 'a' @x or text has 'b'";
        let lexed = lexer::lex(q);
        let mut expr = parser::parse(q, &lexed).unwrap().unwrap();
        assign_ids(&mut expr);
        let mut ids = Vec::new();
        expr.visit_nodes(&mut |node| ids.push(node.id.to_string()));
        assert_eq!(ids, ["@x", "#1"]);
    }

    #[test]
    fn compiles_name_conditions() {
        let q = parse("name ~ '*.rs'", SENSITIVE).unwrap();
        assert_eq!(q.legend, [(CondId::Number(1), "name ~ '*.rs'".to_string())]);
        let Some(Filter::Text { matcher, .. }) = q.filter else {
            panic!()
        };
        assert!(matcher.is_match("a.rs"));
        let Some(Filter::Text { matcher, .. }) = parse(
            "name = 'A.rs'",
            ParseOptions {
                case_insensitive_paths: true,
            },
        )
        .unwrap()
        .filter
        else {
            panic!()
        };
        assert!(matcher.is_match("a.RS"));
        let Some(Filter::Text { matcher, .. }) =
            parse("name = 'caf\u{e9}'", SENSITIVE).unwrap().filter
        else {
            panic!()
        };
        assert!(matcher.is_match("caf\u{e9}"));
    }

    fn matcher(q: &str, insensitive: bool) -> TextMatcher {
        let options = ParseOptions {
            case_insensitive_paths: insensitive,
        };
        let Some(Filter::Text { matcher, .. }) = parse(q, options).unwrap().filter else {
            panic!("{q}")
        };
        matcher
    }

    #[test]
    fn path_conditions() {
        let Some(Filter::Text {
            target, matcher, ..
        }) = parse("path ~ 'src/**/*.rs'", SENSITIVE).unwrap().filter
        else {
            panic!()
        };
        assert_eq!(target, TextTarget::Path);
        assert!(matcher.is_match("src/a/b.rs"));
    }

    #[test]
    fn text_operators() {
        let has = matcher("name has 'a.b'", false);
        assert!(has.is_match("xa.by"));
        assert!(!has.is_match("xaxby"));
        let starts = matcher("name starts '(x'", false);
        assert!(starts.is_match("(x).rs"));
        assert!(!starts.is_match("a(x).rs"));
        let ends = matcher("name ends '.test.ts'", false);
        assert!(ends.is_match("a.test.ts"));
        assert!(!ends.is_match("a.test.tsx"));
        assert!(matcher("name ends '.TS'", true).is_match("a.ts"));
        assert!(!matcher("name ends '.TS'", false).is_match("a.ts"));
        assert!(matcher("name has '\u{e4}'", true).is_match("\u{c4}pfel"));
    }

    #[test]
    fn modifier_i() {
        assert!(matcher("name = 'readme.md' /i", false).is_match("README.md"));
        assert!(matcher("name has '\u{e4}pfel' /I", false).is_match("\u{c4}pfel.md"));
        assert!(matcher("name ~ 'READ*'/i", false).is_match("Readme.md"));
        assert!(matcher("path starts 'LIB/' /i", false).is_match("lib/a.rs"));
        assert!(!matcher("name = 'readme.md'", false).is_match("README.md"));
    }

    fn not_allowed(q: &str) -> (String, Range<usize>, String) {
        let e = parse(q, SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::ModifierNotAllowed, "{q}: {e:?}");
        (e.message, e.span, e.hint.unwrap())
    }

    #[test]
    fn modifier_not_allowed() {
        assert_eq!(
            not_allowed("name = 'a' /m"),
            (
                "modifier '/m' is not allowed here".into(),
                11..13,
                "'/m' works with '=~' on 'text' in file scope".into()
            )
        );
        assert_eq!(
            not_allowed("size > 1M /i").2,
            "'/i' works with the text operators on 'path', 'name' and 'text'"
        );
        assert_eq!(
            not_allowed("name has 'a' /w").2,
            "'/w' works with '=', 'has', 'starts', 'ends', '=~' on 'text'"
        );
        assert_eq!(
            not_allowed("line(text =~ 'a' /m)").2,
            "'/m' works with '=~' on 'text' in file scope"
        );
        assert_eq!(not_allowed("name = 'a' /c").2, "'/c' works with 'text'");
        assert_eq!(
            not_allowed("text ends 'a' /n /e(xml, 1)").2,
            "'/e' can't be combined with '/n'"
        );
        assert_eq!(
            not_allowed("name ~ 'a' /n").2,
            "'/n' works with '=', 'has', 'starts', 'ends'"
        );
        assert_eq!(
            not_allowed("name = 'a' /e(xml, 1)").2,
            "'/e' works with '=', 'has', 'starts', 'ends' on 'text'"
        );
    }

    #[test]
    fn allowed_modifiers_pass_validation() {
        for q in [
            "text =~ 'a' /m /w /i /c",
            "text has 'a' /e(xml, 1) /w",
            "name has 'a b' /n",
            "line(text has 'a' /n /i)",
        ] {
            let e = parse(q, SENSITIVE).unwrap_err();
            assert_eq!(e.code, ErrorCode::NotImplemented, "{q}: {e:?}");
        }
    }

    #[test]
    fn legend_keeps_modifiers_as_written() {
        assert_eq!(legend("name = 'a' /i/w"), ["name = 'a' /i/w"]);
        assert_eq!(legend("name = 'a'/i /w"), ["name = 'a'/i /w"]);
    }

    #[test]
    fn the_empty_query_has_no_filter_and_no_legend() {
        let q = parse(" \n ", SENSITIVE).unwrap();
        assert!(q.filter.is_none());
        assert!(q.legend.is_empty());
    }

    #[test]
    fn invalid_wildcard() {
        let e = parse("name ~ '[a-'", SENSITIVE).unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.unwrap().as_str()),
            (
                ErrorCode::InvalidWildcard,
                "invalid wildcard '[a-'",
                7..12,
                "unclosed '['"
            )
        );
    }

    #[test]
    fn the_earliest_error_wins() {
        let e = parse("name ~ '[a-' and path ~ 'x'", SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::InvalidWildcard);
        let e = parse("path ~ 'x' and name ~ '[a-'", SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotImplemented);
    }
}
