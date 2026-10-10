//! The query language (QUERY.md): text → tokens → syntax tree → condition ids and legend → compiled filter.

pub mod ast;
mod eval;
mod lexer;
mod parser;
mod subset;
mod suggest;
pub mod wildcard;

use std::ops::Range;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use ast::{
    CondId, Condition, Expr, Modifier, Node, NodeKind, Op, Spanned, Target, TargetClass, Value,
};
use lexer::Token;

pub use eval::{EntryType, FileValues, Filter, TextMatcher, TextTarget};
pub(crate) use suggest::did_you_mean;

use crate::error::{ErrorCode, QueryError};

/// Settings which change how the query is compiled.
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    /// `path_case`: whether `path` and `name` are compared case-insensitively.
    pub case_insensitive_paths: bool,
}

/// A parsed and compiled query.
#[derive(Debug, Clone)]
pub struct Query {
    /// Condition id → the condition as written, in the order of appearance (OUTPUT.md#condition-ids).
    pub legend: Vec<(CondId, String)>,
    /// `None` for the empty query, which matches every file.
    pub filter: Option<Filter>,
    /// Whether the query has a `type` condition; without one only regular files are returned.
    pub has_type: bool,
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
        validate(text, expr, false, options, &mut errors);
        duplicate_labels(expr, &mut errors);
    }
    errors.extend(subset::check(text, &lexed, expr.as_ref()));
    // on the same position a real error wins over not_implemented
    if let Some(first) = errors
        .into_iter()
        .min_by_key(|e| (e.span.start, e.code == ErrorCode::NotImplemented))
    {
        return Err(first);
    }
    let mut has_type = false;
    if let Some(expr) = &expr {
        expr.visit_nodes(&mut |node| {
            has_type |= matches!(&node.kind, NodeKind::Condition(c) if c.target == Target::Type);
        });
    }
    let filter = expr.map(|expr| compile(&expr, options));
    Ok(Query {
        legend,
        filter,
        has_type,
    })
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

/// Checks the values and modifiers of all conditions, also of those outside of the implemented subset.
fn validate(
    text: &str,
    expr: &Expr,
    in_line: bool,
    options: ParseOptions,
    errors: &mut Vec<QueryError>,
) {
    match expr {
        Expr::Or(items) | Expr::Xor(items) | Expr::And(items) => {
            for item in items {
                validate(text, item, in_line, options, errors);
            }
        }
        Expr::Not(inner) | Expr::Group(inner) => validate(text, inner, in_line, options, errors),
        Expr::Node(node) => match &node.kind {
            NodeKind::Condition(c) => validate_condition(text, c, in_line, options, errors),
            NodeKind::Line(inner) => validate(text, inner, true, options, errors),
            NodeKind::Near(near) => {
                validate(text, &near.a, true, options, errors);
                validate(text, &near.b, true, options, errors);
            }
        },
    }
}

fn validate_condition(
    text: &str,
    c: &Condition,
    in_line: bool,
    options: ParseOptions,
    errors: &mut Vec<QueryError>,
) {
    if let (Some((Op::Wildcard, _)), Value::Str(s), Target::Name | Target::Path) =
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
    if c.target == Target::Type {
        let words = match &c.value {
            Value::Word(w) => std::slice::from_ref(w),
            Value::Words(words) => words.as_slice(),
            _ => &[],
        };
        let names: Vec<&str> = EntryType::ALL.iter().map(|t| t.keyword()).collect();
        for word in words
            .iter()
            .filter(|w| EntryType::from_keyword(&w.text).is_none())
        {
            errors.push(unknown_value(word, &names));
        }
    }
    for modifier in &c.modifiers {
        if let Some(hint) = modifier_not_allowed(c, modifier, in_line) {
            errors.push(
                QueryError::new(
                    ErrorCode::ModifierNotAllowed,
                    format!(
                        "modifier '{}' is not allowed here",
                        &text[modifier.span.start..modifier.span.start + 1 + modifier.name.len()]
                    ),
                    modifier.span.start..modifier.span.start + 1 + modifier.name.len(),
                )
                .hint(hint),
            );
        }
    }
}

/// `unknown_value` for a word which is not one of `names`.
fn unknown_value(word: &Spanned, names: &[&str]) -> QueryError {
    let expected = format!("expected one of: {}", names.join(", "));
    let hint = match did_you_mean(&word.text, names) {
        Some(s) => format!("did you mean '{s}'? {expected}"),
        None => expected,
    };
    QueryError::new(
        ErrorCode::UnknownValue,
        format!("unknown value '{}'", word.text),
        word.span.clone(),
    )
    .hint(hint)
}

/// `duplicate_label` at every repeated use of a label.
fn duplicate_labels(expr: &Expr, errors: &mut Vec<QueryError>) {
    let mut seen = std::collections::HashSet::new();
    expr.visit_nodes(&mut |node| {
        if let Some(label) = &node.label
            && !seen.insert(label.text.clone())
        {
            errors.push(
                QueryError::new(
                    ErrorCode::DuplicateLabel,
                    format!("duplicate label '@{}'", label.text),
                    label.span.clone(),
                )
                .hint("labels must be unique"),
            );
        }
    });
}

/// The hint of `modifier_not_allowed` if the modifier is not allowed for the condition (QUERY.md#modifiers).
fn modifier_not_allowed(c: &Condition, modifier: &Modifier, in_line: bool) -> Option<&'static str> {
    let op = c.op.as_ref().map(|(op, _)| *op);
    let on_text = c.target == Target::Text;
    let text_class = c.target.class() == TargetClass::Text;
    let plain = matches!(op, Some(Op::Eq | Op::Has | Op::Starts | Op::Ends));
    let has = |name: &str| c.modifiers.iter().any(|m| m.name == name);
    match modifier.name.as_str() {
        "i" => (!text_class)
            .then_some("'/i' works with the text operators on 'path', 'name' and 'text'"),
        "w" => (!(on_text && (plain || op == Some(Op::Regex))))
            .then_some("'/w' works with '=', 'has', 'starts', 'ends', '=~' on 'text'"),
        "m" => (!(on_text && op == Some(Op::Regex) && !in_line))
            .then_some("'/m' works with '=~' on 'text' in file scope"),
        "c" => (!on_text).then_some("'/c' works with 'text'"),
        "e" if !(on_text && plain) => {
            Some("'/e' works with '=', 'has', 'starts', 'ends' on 'text'")
        }
        "e" => has("n").then_some("'/e' can't be combined with '/n'"),
        "n" => (!(text_class && plain)).then_some("'/n' works with '=', 'has', 'starts', 'ends'"),
        _ => None,
    }
}

/// The NFC form of a string.
pub fn nfc(s: &str) -> String {
    if unicode_normalization::is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

/// Compiles a checked query: every condition is in the implemented subset and valid.
fn compile(expr: &Expr, options: ParseOptions) -> Filter {
    let all = |items: &[Expr]| items.iter().map(|e| compile(e, options)).collect();
    match expr {
        Expr::Or(items) => Filter::Or(all(items)),
        Expr::Xor(items) => Filter::Xor(all(items)),
        Expr::And(items) => Filter::And(all(items)),
        Expr::Not(inner) => Filter::Not(Box::new(compile(inner, options))),
        Expr::Group(inner) => compile(inner, options),
        Expr::Node(node) => compile_node(node, options),
    }
}

fn compile_node(node: &Node, options: ParseOptions) -> Filter {
    let NodeKind::Condition(c) = &node.kind else {
        unreachable!("the subset check allows only conditions")
    };
    if c.target == Target::Type {
        let words = match &c.value {
            Value::Word(w) => vec![w],
            Value::Words(words) => words.iter().collect(),
            _ => Vec::new(),
        };
        return Filter::Type {
            id: node.id.clone(),
            order: node.tokens.start,
            values: words
                .iter()
                .filter_map(|w| EntryType::from_keyword(&w.text))
                .collect(),
            negated: matches!(c.op, Some((Op::Ne, _))),
        };
    }
    let (Some((op, _)), Value::Str(s)) = (&c.op, &c.value) else {
        unreachable!("name and path conditions have an operator and a string")
    };
    let value = nfc(&s.value);
    let insensitive = options.case_insensitive_paths || c.modifiers.iter().any(|m| m.name == "i");
    let regex = if *op == Op::Wildcard {
        wildcard::compile(&value, insensitive).expect("wildcards are validated before compiling")
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
    let target = match c.target {
        Target::Path => TextTarget::Path,
        _ => TextTarget::Name,
    };
    Filter::Text {
        id: node.id.clone(),
        order: node.tokens.start,
        target,
        matcher: TextMatcher(regex),
    }
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

    /// The hint of the first validation error, before the subset check.
    fn validation_hint(q: &str) -> String {
        let lexed = lexer::lex(q);
        let expr = parser::parse(q, &lexed).unwrap().unwrap();
        let mut errors = Vec::new();
        validate(q, &expr, false, SENSITIVE, &mut errors);
        assert_eq!(errors[0].code, ErrorCode::ModifierNotAllowed, "{q}");
        errors[0].hint.clone().unwrap()
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
        assert_eq!(not_allowed("name = 'a' /c").2, "'/c' works with 'text'");
        assert_eq!(
            not_allowed("name ~ 'a' /n").2,
            "'/n' works with '=', 'has', 'starts', 'ends'"
        );
        assert_eq!(
            not_allowed("name = 'a' /e(xml, 1)").2,
            "'/e' works with '=', 'has', 'starts', 'ends' on 'text'"
        );
        assert_eq!(not_allowed("name = 'a' /e(xml, 1)").1, 11..13);
        assert_eq!(
            validation_hint("size > 1M /i"),
            "'/i' works with the text operators on 'path', 'name' and 'text'"
        );
        assert_eq!(
            validation_hint("name has 'a' /w"),
            "'/w' works with '=', 'has', 'starts', 'ends', '=~' on 'text'"
        );
        assert_eq!(
            validation_hint("line(text =~ 'a' /m)"),
            "'/m' works with '=~' on 'text' in file scope"
        );
        assert_eq!(
            validation_hint("text ends 'a' /n /e(xml, 1)"),
            "'/e' can't be combined with '/n'"
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
    fn comments_only_is_the_empty_query() {
        let q = parse("# a\n  # b", SENSITIVE).unwrap();
        assert!(q.filter.is_none());
        let q = parse("name = 'a#b' # c\nor name = 'd'", SENSITIVE).unwrap();
        let texts: Vec<&str> = q.legend.iter().map(|(_, t)| t.as_str()).collect();
        assert_eq!(texts, ["name = 'a#b'", "name = 'd'"]);
    }

    fn matches(q: &str, name: &str, entry_type: EntryType) -> bool {
        let query = parse(q, SENSITIVE).unwrap();
        let file = FileValues {
            name,
            path: name,
            entry_type,
        };
        query.filter.unwrap().eval(&file)
    }

    #[test]
    fn type_conditions() {
        assert!(matches("type = dir", "a", EntryType::Dir));
        assert!(!matches("type = dir", "a", EntryType::File));
        assert!(matches("TYPE = LINK", "a", EntryType::Link));
        assert!(matches("type != file", "a", EntryType::Link));
        assert!(!matches("type != file", "a", EntryType::File));
        assert!(matches("type in (file, link)", "a", EntryType::Link));
        assert!(!matches("type in (file, link)", "a", EntryType::Dir));
    }

    #[test]
    fn has_type_anywhere() {
        assert!(!parse("name = 'a'", SENSITIVE).unwrap().has_type);
        assert!(
            parse("name = 'a' or not type = dir", SENSITIVE)
                .unwrap()
                .has_type
        );
        assert!(!parse("", SENSITIVE).unwrap().has_type);
    }

    #[test]
    fn unknown_type_values() {
        let e = parse("type = folder", SENSITIVE).unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.as_deref()),
            (
                ErrorCode::UnknownValue,
                "unknown value 'folder'",
                7..13,
                Some("did you mean 'dir'? expected one of: file, dir, link")
            )
        );
        let e = parse("type in (file, sym)", SENSITIVE).unwrap_err();
        assert_eq!(e.span, 15..18);
        assert_eq!(e.hint.as_deref(), Some("expected one of: file, dir, link"));
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
    fn duplicate_labels() {
        let e = parse(
            "name = 'a' @x or (name = 'b' @y and name = 'c' @x)",
            SENSITIVE,
        )
        .unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.as_deref()),
            (
                ErrorCode::DuplicateLabel,
                "duplicate label '@x'",
                47..49,
                Some("labels must be unique")
            )
        );
    }

    #[test]
    fn labelled_ids_in_the_filter() {
        let q = parse("name = 'a' @x or name = 'b'", SENSITIVE).unwrap();
        let ids: Vec<String> = q.legend.iter().map(|(id, _)| id.to_string()).collect();
        assert_eq!(ids, ["@x", "#1"]);
    }

    #[test]
    fn the_earliest_error_wins() {
        let e = parse("name ~ '[a-' and path ~ 'x'", SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::InvalidWildcard);
        let e = parse("size > 1 and name ~ '[a-'", SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotImplemented);
    }
}
