//! The query language (QUERY.md): text → tokens → syntax tree → condition ids and legend → compiled filter.

pub mod ast;
mod escape;
mod eval;
mod fuzzy;
mod lexer;
pub mod naming;
mod parser;
mod suggest;
mod value;
pub mod wildcard;

use std::ops::{Range, RangeInclusive};

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use ast::{
    CondId, Condition, Expr, Modifier, Node, NodeKind, Op, Spanned, Target, TargetClass, Value,
};
use escape::Style;
use fuzzy::Fuzzy;
use lexer::Token;

pub use eval::{
    ContentState, EntryType, FileValues, Filter, Kind, NumberTarget, Report, TextMatcher,
    TextPattern, TextTarget, Truth,
};
pub(crate) use suggest::{did_you_mean, edit_distance};
pub use value::parse_size;

use crate::error::{ErrorCode, QueryError};
use crate::git::GitValue;

/// Settings which change how the query is compiled.
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// `path_case`: whether `path` and `name` are compared case-insensitively.
    pub case_insensitive_paths: bool,
    /// The moment the search starts: relative dates are relative to it.
    pub now: jiff::Timestamp,
    /// The local time zone of dates without an offset.
    pub time_zone: jiff::tz::TimeZone,
    /// The language table: the values of `lang`.
    pub languages: std::sync::Arc<crate::languages::LanguageTable>,
    /// Whether `\` is a path separator besides `/` in the values of `path` conditions: on Windows.
    pub backslash_separator: bool,
    /// `escape.max_level`: the upper level of `/e(style, any)` and the highest level a query may ask for.
    pub max_escape_level: usize,
    /// `query.fuzzy_min_score`: `~~` matches with a lower score are dropped.
    pub fuzzy_min_score: f64,
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
    /// What the search must read for every entry.
    pub needs: Needs,
    /// What the relevance score looks at.
    pub ranking: crate::relevance::Ranking,
    /// The steps of the funnel; `None` for the empty query.
    pub funnel: Option<Funnel>,
    /// What could narrow the query (OUTPUT.md#hints).
    pub narrowing: Narrowing,
}

/// What the hints of a huge result suggest to narrow a query.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Narrowing {
    /// The query has a `kind` condition.
    pub kind: bool,
    /// The query has a `text ... has` condition without `/w` and `/n`, outside of `not`.
    pub whole_word: bool,
}

impl Narrowing {
    /// What could narrow an expression; `negated` under `not`.
    fn of(expr: &Expr, negated: bool) -> Self {
        let both = |items: &[Expr]| {
            items
                .iter()
                .map(|e| Self::of(e, negated))
                .fold(Self::default(), |a, b| Self {
                    kind: a.kind || b.kind,
                    whole_word: a.whole_word || b.whole_word,
                })
        };
        match expr {
            Expr::Or(items) | Expr::Xor(items) | Expr::And(items) => both(items),
            Expr::Not(inner) => Self::of(inner, true),
            Expr::Group(inner) => Self::of(inner, negated),
            Expr::Node(node) => match &node.kind {
                NodeKind::Condition(c) => {
                    let has = |name: &str| c.modifiers.iter().any(|m| m.name == name);
                    Self {
                        kind: c.target == Target::Kind,
                        whole_word: !negated
                            && c.target == Target::Text
                            && c.op.as_ref().is_some_and(|(op, _)| *op == Op::Has)
                            && !has("w")
                            && !has("n"),
                    }
                }
                NodeKind::Line(inner) => Self::of(inner, negated),
                NodeKind::Near(near) => both(&[near.a.as_ref().clone(), near.b.as_ref().clone()]),
            },
        }
    }
}

/// How the steps of a funnel are joined (OUTPUT.md#steps).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunnelKind {
    /// The items of a top-level `and`: a file is left after a step when it passes it and the steps before.
    And,
    /// The branches of a top-level `or`: a file is counted in its first true branch.
    Or,
    /// Any other query: one step.
    One,
}

/// A step of the funnel: its text as in the legend (with `and ` / `or ` before all but the first) and the index of
/// its item in the top-level `and` / `or` of the filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunnelStep {
    pub text: String,
    pub item: usize,
    /// The text without `and ` / `or `.
    pub condition: String,
    /// When the step is one `text`, `name` or `path` condition with a string: its target.
    pub target: Option<Target>,
    /// The variations which the hints of an empty result try (OUTPUT.md#hints).
    pub variations: Vec<String>,
    /// What the hints look up for similar names or directories.
    pub lookup: Option<Lookup>,
}

/// The names or directories the hints of an empty result look at for a step (OUTPUT.md#hints).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// `name = '...'`: the name.
    Name(String),
    /// `path = '...'`, `path starts '...'`: the directory, `/` as the separator, without a trailing `/`.
    Dir(String),
}

/// The steps of the funnel of a query, in their order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Funnel {
    pub kind: FunnelKind,
    pub steps: Vec<FunnelStep>,
}

/// What the search must find out about every entry for the query.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // independent flags, one per thing to read
pub struct Needs {
    /// An `empty` condition: whether entries are empty.
    pub empty: bool,
    /// `size` or a date condition: the metadata of entries.
    pub metadata: bool,
    /// A `created` condition: entries without a creation date get a warning.
    pub created: bool,
    /// An `accessed` condition: entries without an access date get a warning.
    pub accessed: bool,
    /// A `lang` or `kind` condition: the language of entries.
    pub lang: bool,
    /// A `kind` condition: the kind of entries; nothing is skipped as generated.
    pub kind: bool,
    /// A `git` condition: the git status of files; files outside of a repository get a warning.
    pub git: bool,
    /// A `text` condition: the content of files.
    pub content: bool,
    /// A condition with `/c`: the comments of files, by their language.
    pub comments: bool,
}

/// Parses and checks `text`; the first error from the start of the query is returned.
pub fn parse(text: &str, options: &ParseOptions) -> Result<Query, QueryError> {
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
        validate(text, expr, Scope::File, options, &mut errors);
        duplicate_labels(expr, &mut errors);
    }
    if let Some(first) = errors.into_iter().min_by_key(|e| e.span.start) {
        return Err(first);
    }
    let mut has_type = false;
    let mut needs = Needs::default();
    if let Some(expr) = &expr {
        expr.visit_nodes(&mut |node| {
            if let NodeKind::Condition(c) = &node.kind {
                has_type |= c.target == Target::Type;
                needs.empty |= c.target == Target::Empty;
                needs.metadata |= matches!(
                    c.target,
                    Target::Size | Target::Modified | Target::Created | Target::Accessed
                );
                needs.created |= c.target == Target::Created;
                needs.accessed |= c.target == Target::Accessed;
                needs.lang |= matches!(c.target, Target::Lang | Target::Kind);
                needs.kind |= c.target == Target::Kind;
                needs.git |= c.target == Target::Git;
                needs.content |= matches!(c.target, Target::Text | Target::Len);
                needs.comments |= c.modifiers.iter().any(|m| m.name == "c");
            }
        });
    }
    let mut ranking = crate::relevance::Ranking::default();
    if let Some(expr) = &expr {
        expr.visit_nodes(&mut |node| {
            if let NodeKind::Condition(c) = &node.kind {
                let op = c.op.as_ref().map(|(op, _)| *op);
                ranking.fuzzy |= op == Some(Op::Fuzzy);
                if let (
                    Target::Text,
                    Some(Op::Eq | Op::Has | Op::Starts | Op::Ends),
                    Value::Str(s),
                ) = (c.target, op, &c.value)
                {
                    ranking.add_words(&s.value);
                }
            }
        });
    }
    ranking.content = needs.content;
    let funnel = expr
        .as_ref()
        .map(|expr| funnel_steps(text, &lexed.tokens, expr));
    let narrowing = expr
        .as_ref()
        .map_or_else(Narrowing::default, |expr| Narrowing::of(expr, false));
    let filter = expr.map(|expr| compile(&expr, options));
    ranking.conditions = filter.as_ref().map_or(0, crate::relevance::conditions);
    Ok(Query {
        legend,
        filter,
        has_type,
        needs,
        ranking,
        funnel,
        narrowing,
    })
}

/// The steps of the funnel of a query (OUTPUT.md#steps): the items of a top-level `and`, the metadata ones first and
/// then the ones with content conditions, each in the order of the query; the branches of a top-level `or`; or the
/// whole query. Brackets around the whole query don't count.
fn funnel_steps(text: &str, tokens: &[Token], expr: &Expr) -> Funnel {
    let mut top = expr;
    let mut brackets = 0;
    while let Expr::Group(inner) = top {
        top = inner;
        brackets += 1;
    }
    let (kind, items, joint) = match top {
        Expr::And(items) => (FunnelKind::And, items.as_slice(), "and "),
        Expr::Or(items) => (FunnelKind::Or, items.as_slice(), "or "),
        _ => {
            // the brackets around the whole query are not written
            let range = item_tokens(top, tokens);
            let mut one = funnel_step(text, tokens, "", top, 0);
            one.condition = legend_text(text, tokens, range.start + brackets..range.end - brackets);
            one.text.clone_from(&one.condition);
            return Funnel {
                kind: FunnelKind::One,
                steps: vec![one],
            };
        }
    };
    let mut order: Vec<usize> = (0..items.len()).collect();
    if kind == FunnelKind::And {
        order.sort_by_key(|&i| has_content(&items[i]));
    }
    Funnel {
        kind,
        steps: order
            .into_iter()
            .enumerate()
            .map(|(n, item)| {
                funnel_step(
                    text,
                    tokens,
                    if n == 0 { "" } else { joint },
                    &items[item],
                    item,
                )
            })
            .collect(),
    }
}

/// The step of the item `index` of the top of a query, after `prefix`.
fn funnel_step(
    text: &str,
    tokens: &[Token],
    prefix: &str,
    item: &Expr,
    index: usize,
) -> FunnelStep {
    let condition = legend_text(text, tokens, item_tokens(item, tokens));
    let single = match item {
        Expr::Node(node) => match &node.kind {
            NodeKind::Condition(c) => Some((node, c)),
            _ => None,
        },
        _ => None,
    };
    FunnelStep {
        text: format!("{prefix}{condition}"),
        item: index,
        condition,
        target: single
            .filter(|(_, c)| matches!(c.target, Target::Text | Target::Name | Target::Path))
            .map(|(_, c)| c.target),
        variations: single.map_or_else(Vec::new, |(node, c)| variations(text, tokens, node, c)),
        lookup: single.and_then(|(_, c)| lookup(c)),
    }
}

/// The variations of a `text`, `name` or `path` condition with a string: `/i` (not with `/i`, `/n`, `~~`), `/n`
/// (with `=`, `has`, `starts`, `ends`, without `/e`, for a value of two words or more), without `/w`, `has` instead
/// of `=`.
fn variations(text: &str, tokens: &[Token], node: &Node, c: &Condition) -> Vec<String> {
    let (Some((op, op_span)), Value::Str(value)) = (&c.op, &c.value) else {
        return Vec::new();
    };
    if !matches!(c.target, Target::Text | Target::Name | Target::Path) {
        return Vec::new();
    }
    let has = |name: &str| c.modifiers.iter().any(|m| m.name == name);
    let base = legend_text(text, tokens, node.tokens.clone());
    let mut out = Vec::new();
    if !has("i") && !has("n") && *op != Op::Fuzzy {
        out.push(format!("{base} /i"));
    }
    if !has("n")
        && !has("e")
        && matches!(op, Op::Eq | Op::Has | Op::Starts | Op::Ends)
        && naming::words(&value.value).len() >= 2
    {
        out.push(format!("{base} /n"));
    }
    if has("w") {
        out.push(rewritten(
            text,
            tokens,
            node.tokens.clone(),
            |i| match &tokens[i].tok {
                lexer::Tok::Modifier(m) if m == "w" => Some(""),
                _ => None,
            },
        ));
    }
    if *op == Op::Eq {
        out.push(rewritten(text, tokens, node.tokens.clone(), |i| {
            (tokens[i].span == *op_span).then_some("has")
        }));
    }
    out
}

/// The tokens `range` as in the legend, with the tokens which `replace` gives a text for replaced (an empty text
/// leaves the token out).
fn rewritten(
    text: &str,
    tokens: &[Token],
    range: Range<usize>,
    replace: impl Fn(usize) -> Option<&'static str>,
) -> String {
    let mut out = String::new();
    let mut previous: Option<usize> = None;
    for i in range {
        let replacement = replace(i);
        if replacement == Some("") || matches!(tokens[i].tok, lexer::Tok::Label(_)) {
            continue;
        }
        if previous.is_some_and(|p| tokens[p].span.end != tokens[i].span.start) {
            out.push(' ');
        }
        out.push_str(replacement.unwrap_or(&text[tokens[i].span.clone()]));
        previous = Some(i);
    }
    out
}

/// What the hints look up for a condition: the name of `name = '...'`, the directory of `path = '...'` or
/// `path starts '...'`.
fn lookup(c: &Condition) -> Option<Lookup> {
    let (Some((op, _)), Value::Str(value)) = (&c.op, &c.value) else {
        return None;
    };
    match (c.target, op) {
        (Target::Name, Op::Eq) => Some(Lookup::Name(value.value.clone())),
        (Target::Path, Op::Starts) => {
            let dir = value.value.trim_end_matches('/');
            (!dir.is_empty()).then(|| Lookup::Dir(dir.to_string()))
        }
        (Target::Path, Op::Eq) => value
            .value
            .rsplit_once('/')
            .map(|(dir, _)| Lookup::Dir(dir.to_string())),
        _ => None,
    }
}

/// Whether an expression has a content condition: `text`, `len`, `line( ... )`, `near( ... )`.
fn has_content(expr: &Expr) -> bool {
    let mut content = false;
    expr.visit_nodes(&mut |node| {
        content |= match &node.kind {
            NodeKind::Condition(c) => matches!(c.target, Target::Text | Target::Len),
            NodeKind::Line(_) | NodeKind::Near(_) => true,
        };
    });
    content
}

/// The tokens of an expression: from its first to its last node, with the brackets around it and a `not` before it.
fn item_tokens(expr: &Expr, tokens: &[Token]) -> Range<usize> {
    let (mut start, mut end) = (usize::MAX, 0);
    expr.visit_nodes(&mut |node| {
        start = start.min(node.tokens.start);
        end = end.max(node.tokens.end);
    });
    let is = |i: usize, tok: &lexer::Tok| tokens.get(i).is_some_and(|t| t.tok == *tok);
    loop {
        if start > 0 && is(start - 1, &lexer::Tok::LParen) && is(end, &lexer::Tok::RParen) {
            start -= 1;
            end += 1;
        } else if start > 0
            && matches!(&tokens[start - 1].tok, lexer::Tok::Word(w) if w.eq_ignore_ascii_case("not"))
        {
            start -= 1;
        } else {
            return start..end;
        }
    }
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

/// Where a condition is: in file scope, inside `line( ... )` or inside `near( ... )`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scope {
    File,
    Line,
    Near,
}

impl Scope {
    /// The group as written in messages.
    fn group(self) -> &'static str {
        if self == Scope::Near {
            "near( ... )"
        } else {
            "line( ... )"
        }
    }
}

/// Checks the values and modifiers of all conditions.
fn validate(
    text: &str,
    expr: &Expr,
    scope: Scope,
    options: &ParseOptions,
    errors: &mut Vec<QueryError>,
) {
    match expr {
        Expr::Or(items) | Expr::Xor(items) | Expr::And(items) => {
            for item in items {
                validate(text, item, scope, options, errors);
            }
        }
        Expr::Not(inner) | Expr::Group(inner) => validate(text, inner, scope, options, errors),
        Expr::Node(node) => match &node.kind {
            NodeKind::Condition(c) => validate_condition(text, c, scope, options, errors),
            NodeKind::Line(_) | NodeKind::Near(_) if scope != Scope::File => {
                let inner = if matches!(node.kind, NodeKind::Line(_)) {
                    Scope::Line
                } else {
                    Scope::Near
                };
                let (message, hint) = match (scope, inner) {
                    (Scope::Line, Scope::Line) => (
                        "nested line( ... )".to_string(),
                        "line( ... ) can't be nested",
                    ),
                    (Scope::Line, _) => (
                        "near( ... ) inside line( ... )".to_string(),
                        "line( ... ) can't contain near( ... )",
                    ),
                    (_, Scope::Near) => (
                        "nested near( ... )".to_string(),
                        "near( ... ) can't contain line( ... ) or near( ... )",
                    ),
                    _ => (
                        "line( ... ) inside near( ... )".to_string(),
                        "near( ... ) can't contain line( ... ) or near( ... )",
                    ),
                };
                errors.push(
                    QueryError::new(ErrorCode::NestedLine, message, node.keyword.clone())
                        .hint(hint),
                );
            }
            NodeKind::Line(inner) => validate(text, inner, Scope::Line, options, errors),
            NodeKind::Near(near) => {
                validate(text, &near.a, Scope::Near, options, errors);
                validate(text, &near.b, Scope::Near, options, errors);
                validate_near_options(&near.options, errors);
            }
        },
    }
}

fn validate_condition(
    text: &str,
    c: &Condition,
    scope: Scope,
    options: &ParseOptions,
    errors: &mut Vec<QueryError>,
) {
    if scope == Scope::File && c.target == Target::Len {
        errors.push(
            QueryError::new(
                ErrorCode::LineTargetOutside,
                "'len' outside of line( ... )",
                c.target_span.clone(),
            )
            .hint("'len' works only inside line( ... ) or near( ... )"),
        );
    }
    if c.target == Target::Len {
        validate_lens(text, &c.value, errors);
    }
    if scope != Scope::File && !matches!(c.target, Target::Text | Target::Len) {
        let target = &text[c.target_span.clone()];
        let group = scope.group();
        errors.push(
            QueryError::new(
                ErrorCode::FileTargetInside,
                format!("'{target}' inside {group}"),
                c.target_span.clone(),
            )
            .hint(format!("move '{target}' out of {group}")),
        );
    }
    if let (Some((Op::Wildcard, _)), Value::Str(s), Target::Name | Target::Path) =
        (&c.op, &c.value, c.target)
        && let Err(e) = wildcard::compile(
            &path_value(c, &s.value, options),
            options.case_insensitive_paths,
        )
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
    validate_words(c, options, errors);
    if c.target == Target::Size {
        validate_sizes(text, &c.value, errors);
    }
    if matches!(
        c.target,
        Target::Modified | Target::Created | Target::Accessed
    ) {
        validate_dates(text, &c.value, options, errors);
    }
    if let (Some((Op::Regex, _)), Value::Str(s)) = (&c.op, &c.value)
        && let Err(hint) = user_regex(&s.value, false)
    {
        errors.push(
            QueryError::new(ErrorCode::InvalidRegex, "invalid regex", s.span.clone()).hint(hint),
        );
    }
    for modifier in &c.modifiers {
        if let Some((style, levels)) = &modifier.args {
            let before = errors.len();
            validate_escape(text, style, levels, options.max_escape_level, errors);
            if errors.len() == before
                && c.target == Target::Text
                && content_pattern(c, options.max_escape_level).is_none()
            {
                let highest =
                    escape_levels(text, levels, options.max_escape_level).map_or(0, |l| *l.end());
                errors.push(
                    QueryError::new(
                        ErrorCode::InvalidEscapeLevel,
                        format!("invalid escape level {highest}"),
                        levels.span.clone(),
                    )
                    .hint(format!(
                        "the escaped forms of this value are too many on level {highest}, use a lower one"
                    )),
                );
            }
        }
        if let Some(hint) = modifier_not_allowed(c, modifier, scope != Scope::File) {
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

/// A regex of the query (QUERY.md#regex); the error is the hint of `invalid_regex`.
fn user_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, String> {
    regex::RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .build()
        .map_err(|e| {
            let text = e.to_string();
            if text.contains("look-around") {
                "look-arounds are not supported".to_string()
            } else if text.contains("backreference") {
                "backreferences are not supported".to_string()
            } else {
                text.lines()
                    .find_map(|l| l.strip_prefix("error: "))
                    .unwrap_or("the regex can't be compiled")
                    .to_string()
            }
        })
}

/// The order options and the report options of `near( ... )`.
const NEAR_OPTIONS: [&[&str]; 2] = [&["any", "ordered"], &["all", "closest", "first"]];

/// `invalid_near_option` for an unknown option, `conflicting_options` for a second option of the same kind.
fn validate_near_options(options: &[Spanned], errors: &mut Vec<QueryError>) {
    let mut chosen: [Option<&Spanned>; 2] = [None, None];
    for option in options {
        let lower = option.text.to_lowercase();
        let Some(kind) = NEAR_OPTIONS
            .iter()
            .position(|group| group.contains(&lower.as_str()))
        else {
            errors.push(
                QueryError::new(
                    ErrorCode::InvalidNearOption,
                    format!("unknown option '{}'", option.text),
                    option.span.clone(),
                )
                .hint("options: any, ordered, all, closest, first"),
            );
            continue;
        };
        match chosen[kind] {
            Some(first) if !first.text.eq_ignore_ascii_case(&option.text) => errors.push(
                QueryError::new(
                    ErrorCode::ConflictingOptions,
                    format!("conflicting options '{}' and '{}'", first.text, option.text),
                    option.span.clone(),
                )
                .hint(format!("choose one of: {}", NEAR_OPTIONS[kind].join(", "))),
            ),
            Some(_) => {}
            None => chosen[kind] = Some(option),
        }
    }
}

/// `unknown_value` for the words of `type`, `git`, `kind` and `lang` conditions.
fn validate_words(c: &Condition, options: &ParseOptions, errors: &mut Vec<QueryError>) {
    let words = words(&c.value);
    if c.target == Target::Type {
        let names: Vec<&str> = EntryType::ALL.iter().map(|t| t.keyword()).collect();
        for word in words
            .iter()
            .filter(|w| EntryType::from_keyword(&w.text).is_none())
        {
            errors.push(unknown_value(word, &names));
        }
    }
    if c.target == Target::Git {
        let names: Vec<&str> = GitValue::ALL.iter().map(|v| v.keyword()).collect();
        for word in words
            .iter()
            .filter(|w| GitValue::from_keyword(&w.text).is_none())
        {
            errors.push(unknown_value(word, &names));
        }
    }
    if c.target == Target::Kind {
        let names: Vec<&str> = Kind::ALL.iter().map(|k| k.keyword()).collect();
        for word in words
            .iter()
            .filter(|w| Kind::from_keyword(&w.text).is_none())
        {
            errors.push(unknown_value(word, &names));
        }
    }
    if c.target == Target::Lang {
        let mut names = options.languages.names();
        names.push(UNKNOWN_LANG);
        for word in words
            .iter()
            .filter(|w| !names.iter().any(|n| n.eq_ignore_ascii_case(&w.text)))
        {
            errors.push(unknown_value(word, &names));
        }
    }
}

/// A non-negative integer for `len`, or the `unexpected_token` error.
fn validate_lens(text: &str, value: &Value, errors: &mut Vec<QueryError>) {
    let number = |s: &Spanned, errors: &mut Vec<QueryError>| {
        let n = s
            .text
            .parse::<u64>()
            .ok()
            .filter(|_| s.text.bytes().all(|b| b.is_ascii_digit()));
        if n.is_none() {
            errors.push(
                QueryError::new(
                    ErrorCode::UnexpectedToken,
                    format!("unexpected '{}'", s.text),
                    s.span.clone(),
                )
                .hint("len takes a number of characters, e.g. len > 120"),
            );
        }
        n
    };
    match value {
        Value::Scalar(s) => {
            number(s, errors);
        }
        Value::Range(from, to) => {
            if let (Some(a), Some(b)) = (number(from, errors), number(to, errors))
                && a > b
            {
                errors.push(invalid_range(text, from, to));
            }
        }
        _ => {}
    }
}

/// `invalid_size` for every value which is not a size, `invalid_range` for a range whose start is greater than its
/// end.
fn validate_sizes(text: &str, value: &Value, errors: &mut Vec<QueryError>) {
    let size = |s: &Spanned, errors: &mut Vec<QueryError>| {
        let parsed = value::parse_size(&s.text);
        if parsed.is_none() {
            errors.push(
                QueryError::new(
                    ErrorCode::InvalidSize,
                    format!("invalid size '{}'", s.text),
                    s.span.clone(),
                )
                .hint("units: B, K, M, G"),
            );
        }
        parsed
    };
    match value {
        Value::Scalar(s) => {
            size(s, errors);
        }
        Value::Range(from, to) => {
            if let (Some(a), Some(b)) = (size(from, errors), size(to, errors))
                && a > b
            {
                errors.push(invalid_range(text, from, to));
            }
        }
        _ => {}
    }
}

/// `invalid_date` for every value which is not a date, `invalid_range` for a range whose start is after its start.
fn validate_dates(text: &str, value: &Value, options: &ParseOptions, errors: &mut Vec<QueryError>) {
    let date = |s: &Spanned, errors: &mut Vec<QueryError>| {
        let parsed = value::parse_date(&s.text, options.now, &options.time_zone);
        if parsed.is_none() {
            let hint = if value::is_relative(&s.text) {
                "units: s, m, h, d, w"
            } else {
                "ISO 8601: 2026-10-01, 2026-10-01T14:30, 2026-10-01T14:30:00Z, 2026-10-01T14:30:00+02:00"
            };
            errors.push(
                QueryError::new(
                    ErrorCode::InvalidDate,
                    format!("invalid date '{}'", s.text),
                    s.span.clone(),
                )
                .hint(hint),
            );
        }
        parsed
    };
    match value {
        Value::Scalar(s) => {
            date(s, errors);
        }
        Value::Range(from, to) => {
            if let (Some(a), Some(b)) = (date(from, errors), date(to, errors))
                && a.start > b.start
            {
                errors.push(invalid_range(text, from, to));
            }
        }
        _ => {}
    }
}

/// The arguments of `/e(style, levels)`: `unknown_value` for the style, `invalid_escape_level` or `invalid_range` for
/// the levels.
fn validate_escape(
    text: &str,
    style: &Spanned,
    levels: &Spanned,
    max: usize,
    errors: &mut Vec<QueryError>,
) {
    if Style::from_keyword(&style.text).is_none() {
        let names: Vec<&str> = Style::ALL.iter().map(|s| s.keyword()).collect();
        errors.push(unknown_value(style, &names));
    }
    if let Err(e) = escape_levels(text, levels, max) {
        errors.push(e);
    }
}

/// The levels of `/e` as written: a number, a range `a..b` or `any` up to `max` (QUERY.md#levels).
fn escape_levels(
    text: &str,
    levels: &Spanned,
    max: usize,
) -> Result<RangeInclusive<u8>, QueryError> {
    if levels.text.trim().eq_ignore_ascii_case("any") {
        return Ok(0..=u8::try_from(max).unwrap_or(u8::MAX));
    }
    let not_levels = || {
        QueryError::new(
            ErrorCode::InvalidEscapeLevel,
            format!("invalid escape level '{}'", levels.text),
            levels.span.clone(),
        )
        .hint("levels: a number, a range like 0..2, or any")
    };
    let mut numbers = Vec::new();
    let mut offset = 0;
    for part in levels.text.split("..") {
        let trimmed = part.trim();
        let start = levels.span.start + offset + (part.len() - part.trim_start().len());
        offset += part.len() + 2;
        if trimmed.is_empty() || !trimmed.bytes().all(|b| b.is_ascii_digit()) {
            return Err(not_levels());
        }
        let number = Spanned {
            text: trimmed.to_string(),
            span: start..start + trimmed.len(),
        };
        match trimmed.parse::<u8>() {
            Ok(n) if usize::from(n) <= max => numbers.push((n, number)),
            _ => {
                return Err(QueryError::new(
                    ErrorCode::InvalidEscapeLevel,
                    format!("invalid escape level {trimmed}"),
                    number.span,
                )
                .hint(format!(
                    "max level is {max} (configuration escape.max_level)"
                )));
            }
        }
    }
    match numbers.as_slice() {
        [(n, _)] => Ok(*n..=*n),
        [(a, from), (b, to)] if a > b => Err(invalid_range(text, from, to)),
        [(a, _), (b, _)] => Ok(*a..=*b),
        _ => Err(not_levels()),
    }
}

/// `invalid_range` from the start of `from` to the end of `to`.
fn invalid_range(text: &str, from: &Spanned, to: &Spanned) -> QueryError {
    let span = from.span.start..to.span.end;
    QueryError::new(
        ErrorCode::InvalidRange,
        format!("invalid range '{}'", &text[span.clone()]),
        span,
    )
    .hint("the start must not be greater than the end")
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

/// The language of files without one.
pub const UNKNOWN_LANG: &str = "unknown";

/// The NFC form of a string.
pub fn nfc(s: &str) -> String {
    if unicode_normalization::is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

/// Whether `\` is a path separator besides `/` in queries and in the wildcards of paths: on Windows
/// (QUERY.md#targets).
pub const BACKSLASH_SEPARATOR: bool = cfg!(windows);

/// A path value with `/` for every separator: `\` too where it is one.
pub fn slashed(value: &str, backslash_separator: bool) -> String {
    if backslash_separator {
        value.replace('\\', "/")
    } else {
        value.to_string()
    }
}

/// A wildcard of paths from the configuration (`traversal.exclude`, `kinds`) as it is compiled: in NFC form, with `/`
/// for every separator.
pub fn path_wildcard(wildcard: &str) -> String {
    slashed(&nfc(wildcard), BACKSLASH_SEPARATOR)
}

/// Compiles a checked query: every condition is valid.
fn compile(expr: &Expr, options: &ParseOptions) -> Filter {
    let all = |items: &[Expr]| items.iter().map(|e| compile(e, options)).collect();
    match expr {
        Expr::Or(items) => Filter::Or(all(items)),
        Expr::Xor(items) => Filter::Xor(all(items)),
        Expr::And(items) => Filter::And(all(items)),
        Expr::Not(inner) => Filter::Not(Box::new(compile(inner, options))),
        Expr::Group(inner) => compile(inner, options),
        Expr::Node(node) => match &node.kind {
            NodeKind::Line(inner) => Filter::Line {
                id: node.id.clone(),
                order: node.tokens.start,
                inner: Box::new(compile(inner, options)),
            },
            NodeKind::Near(near) => {
                let has = |name: &str| {
                    near.options
                        .iter()
                        .any(|o| o.text.eq_ignore_ascii_case(name))
                };
                Filter::Near {
                    id: node.id.clone(),
                    order: node.tokens.start,
                    a: Box::new(compile(&near.a, options)),
                    b: Box::new(compile(&near.b, options)),
                    distance: near.distance.text.parse().unwrap_or(usize::MAX),
                    ordered: has("ordered"),
                    report: if has("closest") {
                        Report::Closest
                    } else if has("first") {
                        Report::First
                    } else {
                        Report::All
                    },
                }
            }
            NodeKind::Condition(_) => compile_node(node, options),
        },
    }
}

fn compile_node(node: &Node, options: &ParseOptions) -> Filter {
    let NodeKind::Condition(c) = &node.kind else {
        unreachable!("compile handles line( ... ) and near( ... )")
    };
    let id = node.id.clone();
    let order = node.tokens.start;
    match c.target {
        Target::Size => compile_size(c, id, order),
        Target::Len => {
            #[allow(clippy::cast_precision_loss)] // lengths are far below 2^53
            let len = |s: &Spanned| s.text.parse::<u64>().expect("lengths are validated") as f64;
            let bounds = match (&c.op, &c.value) {
                (Some((op, _)), Value::Scalar(v)) => value::bounds(*op, len(v), None),
                (Some((op, _)), Value::Range(a, b)) => value::bounds(*op, len(a), Some(len(b))),
                _ => unreachable!("len conditions have an operator and a value"),
            };
            Filter::Len { id, order, bounds }
        }
        Target::Modified | Target::Created | Target::Accessed => {
            compile_date(c, id, order, options)
        }
        Target::Empty => Filter::Empty { id, order },
        Target::Lang => Filter::Lang {
            id,
            order,
            values: words(&c.value)
                .iter()
                .map(|w| w.text.to_lowercase())
                .collect(),
            negated: matches!(c.op, Some((Op::Ne, _))),
        },
        Target::Git => Filter::Git {
            id,
            order,
            values: words(&c.value)
                .iter()
                .filter_map(|w| GitValue::from_keyword(&w.text))
                .collect(),
            negated: matches!(c.op, Some((Op::Ne, _))),
        },
        Target::Kind => Filter::Kind {
            id,
            order,
            values: words(&c.value)
                .iter()
                .filter_map(|w| Kind::from_keyword(&w.text))
                .collect(),
            negated: matches!(c.op, Some((Op::Ne, _))),
        },
        Target::Type => Filter::Type {
            id,
            order,
            values: words(&c.value)
                .iter()
                .filter_map(|w| EntryType::from_keyword(&w.text))
                .collect(),
            negated: matches!(c.op, Some((Op::Ne, _))),
        },
        _ => compile_text(c, id, order, options),
    }
}

/// The words of a value target: one, or the list of `in ( ... )`.
fn words(value: &Value) -> &[Spanned] {
    match value {
        Value::Word(w) => std::slice::from_ref(w),
        Value::Words(words) => words.as_slice(),
        _ => &[],
    }
}

fn compile_size(c: &Condition, id: CondId, order: usize) -> Filter {
    let size =
        |s: &Spanned| value::parse_size(&s.text).expect("sizes are validated before compiling");
    let bounds = match (&c.op, &c.value) {
        (Some((op, _)), Value::Scalar(v)) => value::bounds(*op, size(v), None),
        (Some((op, _)), Value::Range(a, b)) => value::bounds(*op, size(a), Some(size(b))),
        _ => unreachable!("size conditions have an operator and a value"),
    };
    Filter::Number {
        id,
        order,
        target: NumberTarget::Size,
        bounds,
    }
}

fn compile_date(c: &Condition, id: CondId, order: usize, options: &ParseOptions) -> Filter {
    let target = match c.target {
        Target::Created => NumberTarget::Created,
        Target::Accessed => NumberTarget::Accessed,
        _ => NumberTarget::Modified,
    };
    let date = |s: &Spanned| {
        value::parse_date(&s.text, options.now, &options.time_zone)
            .expect("dates are validated before compiling")
    };
    let bounds = match (&c.op, &c.value) {
        (Some((op, _)), Value::Scalar(v)) => value::date_bounds(*op, date(v), None),
        (Some((op, _)), Value::Range(a, b)) => value::date_bounds(*op, date(a), Some(date(b))),
        _ => unreachable!("date conditions have an operator and a value"),
    };
    Filter::Number {
        id,
        order,
        target,
        bounds,
    }
}

/// The pattern of a `text` condition: the regex of `=~`, otherwise the value — escaped (`/e`) or in any naming style
/// (`/n`) — with the anchors of its operator; `None` when the escaped forms are too many for the regex engine.
fn content_pattern(c: &Condition, max_escape_level: usize) -> Option<TextPattern> {
    let Value::Str(s) = &c.value else {
        unreachable!("text conditions have a string")
    };
    let has = |name: &str| c.modifiers.iter().any(|m| m.name == name);
    let whole_word = has("w");
    let skip_comments = has("c");
    let op = c.op.as_ref().map(|(op, _)| *op);
    if op == Some(Op::Regex) {
        let multiline = has("m");
        let regex = regex::RegexBuilder::new(&s.value)
            .case_insensitive(has("i"))
            .multi_line(multiline)
            .build()
            .expect("regexes are validated before compiling");
        return Some(TextPattern {
            regex,
            per_line: !multiline,
            whole_word,
            skip_comments,
            escape_levels: Vec::new(),
        });
    }
    let escape = c.modifiers.iter().find_map(|m| {
        let (style, levels) = m.args.as_ref()?;
        let style = Style::from_keyword(&style.text).expect("styles are validated");
        let levels = escape_levels("", levels, max_escape_level).expect("levels are validated");
        Some((style, levels))
    });
    let (escaped, escape_levels) = match escape {
        Some((style, levels)) => (
            escape::pattern(style, &s.value, levels.clone())?,
            levels.collect(),
        ),
        None if has("n") => (naming::pattern(&s.value), Vec::new()),
        None => (regex::escape(&s.value), Vec::new()),
    };
    let anchored = match op {
        Some(Op::Eq) => format!(r"\A{escaped}\z"),
        Some(Op::Starts) => format!(r"\A{escaped}"),
        Some(Op::Ends) => format!(r"{escaped}\z"),
        _ => escaped,
    };
    let flags = if has("n") || has("i") { "(?i)" } else { "" };
    Some(TextPattern {
        regex: Regex::new(&format!("{flags}{anchored}")).ok()?,
        per_line: false,
        whole_word,
        skip_comments,
        escape_levels,
    })
}

fn compile_text(c: &Condition, id: CondId, order: usize, options: &ParseOptions) -> Filter {
    if c.target == Target::Text {
        return Filter::Content {
            id,
            order,
            pattern: content_pattern(c, options.max_escape_level)
                .expect("the escaped forms are checked before compiling"),
        };
    }
    let (Some((op, _)), Value::Str(s)) = (&c.op, &c.value) else {
        unreachable!("name and path conditions have an operator and a string")
    };
    let value = path_value(c, &s.value, options);
    let identifier = c.modifiers.iter().any(|m| m.name == "n");
    let insensitive =
        options.case_insensitive_paths || identifier || c.modifiers.iter().any(|m| m.name == "i");
    if *op == Op::Fuzzy {
        return Filter::Text {
            id,
            order,
            target: text_target(c.target),
            matcher: TextMatcher::Fuzzy {
                fuzzy: Fuzzy::new(&value, c.target == Target::Path),
                min_score: options.fuzzy_min_score,
            },
        };
    }
    let regex = if *op == Op::Wildcard {
        wildcard::compile(&value, insensitive).expect("wildcards are validated before compiling")
    } else if *op == Op::Regex {
        user_regex(&value, insensitive).expect("regexes are validated before compiling")
    } else {
        let escaped = if identifier {
            naming::pattern(&value)
        } else {
            regex::escape(&value)
        };
        let pattern = match op {
            Op::Eq => format!("^{escaped}$"),
            Op::Starts => format!("^{escaped}"),
            Op::Ends => format!("{escaped}$"),
            _ => escaped,
        };
        let flags = if insensitive { "(?i)" } else { "" };
        Regex::new(&format!("{flags}{pattern}")).expect("an escaped string is a valid regex")
    };
    Filter::Text {
        id,
        order,
        target: text_target(c.target),
        matcher: TextMatcher::Regex(regex),
    }
}

/// The value of a `name` or `path` condition as it is compared: in NFC form; in a `path` value which isn't a regex
/// `\` is a separator where the options say so.
fn path_value(c: &Condition, value: &str, options: &ParseOptions) -> String {
    let regex = matches!(c.op, Some((Op::Regex, _)));
    if c.target == Target::Path && !regex {
        slashed(&nfc(value), options.backslash_separator)
    } else {
        nfc(value)
    }
}

/// The value a `name` or `path` condition looks at.
fn text_target(target: Target) -> TextTarget {
    if target == Target::Path {
        TextTarget::Path
    } else {
        TextTarget::Name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SENSITIVE: std::sync::LazyLock<ParseOptions> =
        std::sync::LazyLock::new(|| ParseOptions {
            case_insensitive_paths: false,
            now: jiff::Timestamp::UNIX_EPOCH,
            time_zone: jiff::tz::TimeZone::UTC,
            languages: std::sync::Arc::new(crate::languages::LanguageTable::builtin()),
            backslash_separator: false,
            max_escape_level: 3,
            fuzzy_min_score: 0.5,
        });

    fn funnel(q: &str) -> Option<(FunnelKind, Vec<String>)> {
        parse(q, &SENSITIVE)
            .unwrap()
            .funnel
            .map(|f| (f.kind, f.steps.into_iter().map(|s| s.text).collect()))
    }

    #[test]
    fn the_steps_of_the_funnel() {
        assert_eq!(
            funnel("text has 'parse' and lang = rust"),
            Some((
                FunnelKind::And,
                vec!["lang = rust".into(), "and text has 'parse'".into()]
            ))
        );
        assert_eq!(
            funnel("lang = rust and (text has 'a' or text has 'b') and not size > 1K"),
            Some((
                FunnelKind::And,
                vec![
                    "lang = rust".into(),
                    "and not size > 1K".into(),
                    "and (text has 'a' or text has 'b')".into()
                ]
            ))
        );
        assert_eq!(
            funnel("line(text has 'a')   and name ~ '*.rs'"),
            Some((
                FunnelKind::And,
                vec!["name ~ '*.rs'".into(), "and line(text has 'a')".into()]
            ))
        );
        assert_eq!(
            funnel("name = 'a' or not lang = rust"),
            Some((
                FunnelKind::Or,
                vec!["name = 'a'".into(), "or not lang = rust".into()]
            ))
        );
        assert_eq!(
            funnel("((size > 1K))"),
            Some((FunnelKind::One, vec!["size > 1K".into()]))
        );
        assert_eq!(
            funnel("name = 'a' xor name = 'b'"),
            Some((FunnelKind::One, vec!["name = 'a' xor name = 'b'".into()]))
        );
        assert_eq!(funnel(""), None);
        // the steps point at the items of the compiled filter
        let q = parse("text has 'x' and lang = rust", &SENSITIVE).unwrap();
        let order: Vec<usize> = q.funnel.unwrap().steps.iter().map(|s| s.item).collect();
        assert_eq!(order, [1, 0]);
    }

    fn variations(q: &str) -> Vec<(String, Vec<String>)> {
        parse(q, &SENSITIVE)
            .unwrap()
            .funnel
            .unwrap()
            .steps
            .into_iter()
            .map(|s| (s.condition, s.variations))
            .collect()
    }

    #[test]
    fn the_variations_of_a_step() {
        assert_eq!(
            variations("text has 'parseQuery'"),
            [(
                "text has 'parseQuery'".to_string(),
                vec![
                    "text has 'parseQuery' /i".to_string(),
                    "text has 'parseQuery' /n".to_string()
                ]
            )]
        );
        assert_eq!(
            variations("text has 'Query' /w")[0].1,
            ["text has 'Query' /w /i", "text has 'Query'"]
        );
        assert_eq!(
            variations("name = 'config.yml'")[0].1,
            [
                "name = 'config.yml' /i",
                "name = 'config.yml' /n",
                "name has 'config.yml'"
            ]
        );
        assert!(variations("text has 'x' /i")[0].1.is_empty());
        assert!(variations("size > 1K")[0].1.is_empty());
        assert!(variations("name ~~ 'cfg'")[0].1.is_empty());
        assert_eq!(
            variations("(text has 'a b')")[0].1,
            ["text has 'a b' /i", "text has 'a b' /n"]
        );
        let steps = variations("lang = rust and (text has 'a' or text has 'b')");
        assert_eq!(steps[1].0, "(text has 'a' or text has 'b')");
        assert!(steps[1].1.is_empty(), "a group has no variations");
    }

    #[test]
    fn what_narrows_a_query() {
        let narrowing = |q: &str| parse(q, &SENSITIVE).unwrap().narrowing;
        assert_eq!(
            narrowing("text has 'error'"),
            Narrowing {
                kind: false,
                whole_word: true
            }
        );
        assert_eq!(
            narrowing("text has 'error' /w and kind = source"),
            Narrowing {
                kind: true,
                whole_word: false
            }
        );
        assert!(!narrowing("text has 'error a' /n").whole_word);
        assert!(!narrowing("text = 'x' or name has 'x'").whole_word);
        assert!(
            !narrowing("not text has 'x'").whole_word,
            "nothing under not"
        );
    }

    fn legend(q: &str) -> Vec<String> {
        let lexed = lexer::lex(q);
        let mut expr = parser::parse(q, &lexed).unwrap().unwrap();
        let mut out = Vec::new();
        expr.visit_nodes_mut(&mut |n| out.push(legend_text(q, &lexed.tokens, n.tokens.clone())));
        out
    }

    #[test]
    fn escape_levels_up_to_the_configured_maximum() {
        let options = |max| ParseOptions {
            max_escape_level: max,
            ..SENSITIVE.clone()
        };
        assert!(parse("text has 'a' /e(xml, 5)", &options(5)).is_ok());
        let error = parse("text has 'a' /e(xml, 5)", &options(4)).unwrap_err();
        assert_eq!(
            error.hint.as_deref(),
            Some("max level is 4 (configuration escape.max_level)")
        );
        // any is 0 to the maximum, at most 255 for a bigger one
        assert!(parse("text has 'a' /e(xml, any)", &options(1000)).is_ok());
        let error = parse("text has \"it's\" /e(backslash, 0..20)", &options(20)).unwrap_err();
        assert_eq!(error.code, ErrorCode::InvalidEscapeLevel);
        assert_eq!(error.message, "invalid escape level 20");
        assert_eq!(
            error.hint.as_deref(),
            Some("the escaped forms of this value are too many on level 20, use a lower one")
        );
        let q = "text has \"it's\" /e(backslash, 0..20)";
        assert_eq!(&q[error.span.clone()], "0..20");
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
        let q = parse("name ~ '*.rs'", &SENSITIVE).unwrap();
        assert_eq!(q.legend, [(CondId::Number(1), "name ~ '*.rs'".to_string())]);
        let Some(Filter::Text { matcher, .. }) = q.filter else {
            panic!()
        };
        assert!(matcher.is_match("a.rs"));
        let Some(Filter::Text { matcher, .. }) = parse(
            "name = 'A.rs'",
            &ParseOptions {
                case_insensitive_paths: true,
                ..SENSITIVE.clone()
            },
        )
        .unwrap()
        .filter
        else {
            panic!()
        };
        assert!(matcher.is_match("a.RS"));
        let Some(Filter::Text { matcher, .. }) =
            parse("name = 'caf\u{e9}'", &SENSITIVE).unwrap().filter
        else {
            panic!()
        };
        assert!(matcher.is_match("caf\u{e9}"));
    }

    #[test]
    fn backslash_is_a_separator_where_asked() {
        let windows = ParseOptions {
            backslash_separator: true,
            ..SENSITIVE.clone()
        };
        let path = |q: &str, options: &ParseOptions| {
            let Some(Filter::Text { matcher, .. }) = parse(q, options).unwrap().filter else {
                panic!("{q}")
            };
            matcher
        };
        assert!(path(r"path ~ `src\**\*.rs`", &windows).is_match("src/a/b.rs"));
        assert!(path(r"path = `src\a.rs`", &windows).is_match("src/a.rs"));
        assert!(path(r"path starts `src\`", &windows).is_match("src/a.rs"));
        // a regex keeps its escapes
        assert!(path(r"path =~ `^src\.rs$`", &windows).is_match("src.rs"));
        // elsewhere `\` is a character of the path
        assert!(!path(r"path = `src\a.rs`", &SENSITIVE).is_match("src/a.rs"));
        assert!(path(r"path = `src\a.rs`", &SENSITIVE).is_match(r"src\a.rs"));
        assert_eq!(slashed(r"a\b/c", true), "a/b/c");
        assert_eq!(slashed(r"a\b", false), r"a\b");
    }

    fn matcher(q: &str, insensitive: bool) -> TextMatcher {
        let options = ParseOptions {
            case_insensitive_paths: insensitive,
            ..SENSITIVE.clone()
        };
        let Some(Filter::Text { matcher, .. }) = parse(q, &options).unwrap().filter else {
            panic!("{q}")
        };
        matcher
    }

    #[test]
    fn path_conditions() {
        let Some(Filter::Text {
            target, matcher, ..
        }) = parse("path ~ 'src/**/*.rs'", &SENSITIVE).unwrap().filter
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
        let e = parse(q, &SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::ModifierNotAllowed, "{q}: {e:?}");
        (e.message, e.span, e.hint.unwrap())
    }

    /// The hint of the first validation error.
    fn validation_hint(q: &str) -> String {
        let lexed = lexer::lex(q);
        let expr = parser::parse(q, &lexed).unwrap().unwrap();
        let mut errors = Vec::new();
        validate(q, &expr, Scope::File, &SENSITIVE, &mut errors);
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
    fn escape_errors() {
        let code_span = |q: &str| {
            let (code, _, span, _) = error(q);
            (code, span)
        };
        assert_eq!(
            code_span("text has 'a' /e(backslash, 4)"),
            (ErrorCode::InvalidEscapeLevel, 27..28)
        );
        assert_eq!(
            code_span("text has 'a' /e(xml, 1..9)"),
            (ErrorCode::InvalidEscapeLevel, 24..25)
        );
        assert_eq!(
            code_span("text has 'a' /e(xml, every)"),
            (ErrorCode::InvalidEscapeLevel, 21..26)
        );
        assert_eq!(
            code_span("text has 'a' /e(xml, 1.5)"),
            (ErrorCode::InvalidEscapeLevel, 21..24)
        );
        assert_eq!(
            code_span("text has 'a' /e(xml, 2..0)"),
            (ErrorCode::InvalidRange, 21..25)
        );
        assert_eq!(
            code_span("text has 'a' /e(html, 1)"),
            (ErrorCode::UnknownValue, 16..20)
        );
        assert!(parse("text has 'a' /e(Backslash, ANY) /w /c", &SENSITIVE).is_ok());
        assert!(parse("text = 'a' /e(url, 0..3) /i", &SENSITIVE).is_ok());
    }

    #[test]
    fn allowed_modifiers_pass_validation() {
        assert!(parse("text =~ 'a' /m /w /i /c", &SENSITIVE).is_ok());
        assert!(parse("text has 'a' /e(xml, 1) /w", &SENSITIVE).is_ok());
        assert!(parse("name has 'a b' /n", &SENSITIVE).is_ok());
        assert!(parse("line(text has 'a' /n /i)", &SENSITIVE).is_ok());
    }

    #[test]
    fn legend_keeps_modifiers_as_written() {
        assert_eq!(legend("name = 'a' /i/w"), ["name = 'a' /i/w"]);
        assert_eq!(legend("name = 'a'/i /w"), ["name = 'a'/i /w"]);
    }

    #[test]
    fn comments_only_is_the_empty_query() {
        let q = parse("# a\n  # b", &SENSITIVE).unwrap();
        assert!(q.filter.is_none());
        let q = parse("name = 'a#b' # c\nor name = 'd'", &SENSITIVE).unwrap();
        let texts: Vec<&str> = q.legend.iter().map(|(_, t)| t.as_str()).collect();
        assert_eq!(texts, ["name = 'a#b'", "name = 'd'"]);
    }

    fn matches(q: &str, name: &str, entry_type: EntryType) -> bool {
        let query = parse(q, &SENSITIVE).unwrap();
        let file = FileValues {
            name,
            path: name,
            entry_type,
            is_link: entry_type == EntryType::Link,
            empty: name.is_empty(),
            size: name.len() as u64,
            modified: Some(0.0),
            created: Some(0.0),
            accessed: Some(0.0),
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        query.filter.unwrap().matches(&file)
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
    fn empty_conditions() {
        assert!(matches("empty", "", EntryType::File));
        assert!(!matches("empty", "a", EntryType::File));
        assert!(matches("not empty and name = 'a'", "a", EntryType::Dir));
        assert!(
            parse("name = 'a' or empty", &SENSITIVE)
                .unwrap()
                .needs
                .empty
        );
        assert!(!parse("name = 'a'", &SENSITIVE).unwrap().needs.empty);
    }

    #[test]
    fn size_conditions() {
        assert!(matches("size = 3", "abc", EntryType::File));
        assert!(matches(
            "size in 1..3B and size != 2",
            "abc",
            EntryType::File
        ));
        assert!(!matches("size > 3", "abc", EntryType::File));
        assert!(parse("size > 1", &SENSITIVE).unwrap().needs.metadata);
        assert!(!parse("name = 'a'", &SENSITIVE).unwrap().needs.metadata);
    }

    #[test]
    fn size_errors() {
        let e = parse("size > 10MB", &SENSITIVE).unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.as_deref()),
            (
                ErrorCode::InvalidSize,
                "invalid size '10MB'",
                7..11,
                Some("units: B, K, M, G")
            )
        );
        let e = parse("size in 1K..1x", &SENSITIVE).unwrap_err();
        assert_eq!((e.code, e.span), (ErrorCode::InvalidSize, 12..14));
        let e = parse("size in 10M..1M", &SENSITIVE).unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.as_deref()),
            (
                ErrorCode::InvalidRange,
                "invalid range '10M..1M'",
                8..15,
                Some("the start must not be greater than the end")
            )
        );
    }

    #[test]
    fn created_conditions() {
        assert!(matches("created >= 1970-01-01", "a", EntryType::File));
        assert!(parse("created > -1d", &SENSITIVE).unwrap().needs.created);
        assert!(parse("created > -1d", &SENSITIVE).unwrap().needs.metadata);
        assert_eq!(
            parse("created > -1y", &SENSITIVE).unwrap_err().code,
            ErrorCode::InvalidDate
        );
    }

    #[test]
    fn accessed_conditions() {
        assert!(matches("accessed >= 1970-01-01", "a", EntryType::File));
        let q = parse("accessed < -30d", &SENSITIVE).unwrap();
        assert!(q.needs.accessed && q.needs.metadata && !q.needs.created);
    }

    #[test]
    fn lang_conditions() {
        let file = |lang| FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang,
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        let q = parse("lang in (rust, Python)", &SENSITIVE).unwrap();
        assert!(q.needs.lang);
        assert!(q.filter.as_ref().unwrap().matches(&file("python")));
        assert!(!q.filter.as_ref().unwrap().matches(&file("unknown")));
        let q = parse("lang != unknown", &SENSITIVE).unwrap();
        assert!(q.filter.unwrap().matches(&file("rust")));
        let e = parse("lang = rusty", &SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::UnknownValue);
        assert!(
            e.hint
                .unwrap()
                .starts_with("did you mean 'rust'? expected one of: c, cpp, ")
        );
    }

    #[test]
    fn kind_conditions() {
        let file = |kind| FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "unknown",
            kind,
            git: None,
            content: ContentState::Absent,
        };
        let q = parse("kind in (source, TEST)", &SENSITIVE).unwrap();
        assert!(q.needs.kind && q.needs.lang);
        assert!(q.filter.as_ref().unwrap().matches(&file(Kind::Test)));
        assert!(!q.filter.as_ref().unwrap().matches(&file(Kind::Generated)));
        let q = parse("kind != generated", &SENSITIVE).unwrap();
        assert!(q.filter.unwrap().matches(&file(Kind::Other)));
        let e = parse("kind = tests", &SENSITIVE).unwrap_err();
        assert_eq!(
            e.hint.as_deref(),
            Some(
                "did you mean 'test'? expected one of: source, test, config, doc, generated, other"
            )
        );
    }

    fn content_matches(q: &str, text: &str) -> bool {
        let content = crate::content::Content::from_bytes(text.as_bytes());
        let file = FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Text(&content),
        };
        parse(q, &SENSITIVE).unwrap().filter.unwrap().matches(&file)
    }

    #[test]
    fn whole_content_operators() {
        assert!(content_matches("text = 'a\nb\n'", "a\nb\n"));
        assert!(!content_matches("text = 'a\nb'", "a\nb\n"));
        assert!(content_matches("text starts '#!/bin'", "#!/bin/sh\n"));
        assert!(!content_matches("text starts 'bin'", "#!/bin/sh\n"));
        assert!(content_matches("text ends '\n'", "a\r\n"));
        assert!(!content_matches("text ends '\n'", "a"));
        assert!(content_matches("text ends 'A' /i", "xa"));
    }

    fn rust_matches(q: &str, text: &str) -> bool {
        let table = crate::languages::LanguageTable::builtin();
        let rust = table.detect("a.rs", || None).unwrap();
        let content = crate::content::Content::from_bytes(text.as_bytes()).with_comments(rust);
        let file = FileValues {
            name: "a.rs",
            path: "a.rs",
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "rust",
            kind: Kind::Other,
            git: None,
            content: ContentState::Text(&content),
        };
        parse(q, &SENSITIVE).unwrap().filter.unwrap().matches(&file)
    }

    #[test]
    fn skip_comments() {
        assert!(!rust_matches("text has 'x' /c", "// x\n/* x */"));
        assert!(rust_matches("text has 'x' /c", "x // x"));
        assert!(rust_matches("text has 'x' /c", "\"// x\""));
        assert!(rust_matches("text has 'x' /c /w", "// x\nx"));
        assert!(!rust_matches("text =~ `^ *x` /c", "  // x"));
        assert!(rust_matches("text =~ `x$` /c /m", "// x\nx"));
        assert!(!rust_matches("line(text has 'x' /c)", "/* a\nx */"));
        assert!(rust_matches("line(text has 'x')", "/* a\nx */"));
        // a match which starts before a comment counts
        assert!(rust_matches("text has 'a //' /c", "a // b"));
        assert!(rust_matches(
            "near(text has 'a' /c, text has 'b' /c, 2)",
            "a\n// b\nb"
        ));
        assert!(!rust_matches(
            "near(text has 'a' /c, text has 'b' /c, 1)",
            "a\n// b\nb"
        ));
        assert!(!rust_matches(
            "near(text has 'a' /c, text has 'b' /c, 0)",
            "a // b"
        ));
    }

    #[test]
    fn escaped_text() {
        let q = "text has `\"a\"` /e(backslash, 1)";
        assert!(content_matches(q, "x \\\"a\\\" y"));
        assert!(!content_matches(q, "x \"a\" y"));
        assert!(content_matches("text = 'a b' /e(url, any)", "a+b"));
        assert!(!content_matches("text = 'a b' /e(url, any)", "a+b\n"));
        assert!(content_matches("text starts '<' /e(xml, 1)", "&lt;p>"));
        assert!(content_matches("text ends '<' /e(xml, 1)", "p&lt;"));
        assert!(content_matches("text has '&' /e(xml, 2) /i", "&AMP;AMP;"));
        // /w checks the boundaries around the escaped match
        assert!(content_matches("text has 'a b' /e(url, 1) /w", "x a%20b"));
        assert!(!content_matches("text has 'a b' /e(url, 1) /w", "xa%20b"));
        assert!(content_matches("line(text has '<' /e(xml, 1))", "a\n&lt;"));
        assert!(!rust_matches(
            "text has '\"' /e(backslash, 1) /c",
            "// \\\"\nx"
        ));
        assert!(rust_matches(
            "text has '\"' /e(backslash, 1) /c",
            "x(\"\\\"\")"
        ));
    }

    fn name_matches(q: &str, path: &str) -> bool {
        let file = FileValues {
            name: path.rsplit('/').next().unwrap(),
            path,
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        parse(q, &SENSITIVE).unwrap().filter.unwrap().matches(&file)
    }

    #[test]
    fn naming_styles() {
        assert!(content_matches(
            "text has 'parse query' /n",
            "x = parse_query(1)"
        ));
        assert!(content_matches("text = 'parseQuery' /n", "PARSE_QUERY"));
        assert!(!content_matches("text = 'parseQuery' /n", "PARSE_QUERY\n"));
        assert!(content_matches(
            "text starts 'parse query' /n",
            "ParseQuery()"
        ));
        assert!(content_matches(
            "text ends 'parse query' /n",
            "x.parse-query"
        ));
        assert!(!content_matches(
            "text has 'parse query' /n",
            "parse__query"
        ));
        assert!(content_matches("text has 'user id' /n/w", "(user_id)"));
        assert!(!content_matches("text has 'user id' /n/w", "userIdentity"));
        assert!(content_matches("line(text = 'user id' /n)", "a\nUserId"));
        assert!(name_matches(
            "name has 'config loader' /n",
            "src/ConfigLoader.java"
        ));
        assert!(!name_matches(
            "name = 'config loader.rs' /n",
            "src/config_loader.rs"
        ));
        assert!(name_matches(
            "name starts 'config loader' /n",
            "src/config-loader.ts"
        ));
        assert!(!name_matches("path has 'src config' /n", "src/config.rs"));
        assert!(name_matches(
            "path ends 'config loader.rs' /n",
            "src/config.loader.rs"
        ));
    }

    #[test]
    fn fuzzy_names_and_paths() {
        assert!(name_matches("name ~~ 'cfgldr'", "src/config_loader.rs"));
        assert!(!name_matches(
            "name ~~ 'cfgldr'",
            "src/scaffold_bigolder.ts"
        ));
        assert!(!name_matches("name ~~ 'cfgx'", "src/config_loader.rs"));
        assert!(name_matches("path ~~ 'srv hndl'", "src/server/handlers.rs"));
        assert!(!name_matches(
            "name ~~ 'srv hndl'",
            "src/server/handlers.rs"
        ));
    }

    #[test]
    fn followed_links_are_links() {
        let file = FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            is_link: true,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Absent,
        };
        let matches = |q: &str| parse(q, &SENSITIVE).unwrap().filter.unwrap().matches(&file);
        assert!(matches("type = file and type = link"));
        assert!(matches("type in (dir, link)"));
        assert!(!matches("type != link"));
        assert!(!matches("type = dir"));
    }

    #[test]
    fn whole_words() {
        assert!(content_matches("text has 'id' /w", "x id y"));
        assert!(content_matches("text has 'id' /w", "id"));
        assert!(!content_matches(
            "text has 'id' /w",
            "id_2 valid id2 \u{e4}id"
        ));
        assert!(content_matches("text has 'id' /w", "(id)"));
        assert!(content_matches("text has '.unwrap(' /w", "(.unwrap()"));
        assert!(!content_matches("text has '.unwrap(' /w", "x.unwrap()"));
        assert!(content_matches("text has 'aa' /w", "aaa aa"));
        assert!(content_matches("text =~ `a+` /w", "ba aa"));
        assert!(content_matches("text starts 'id' /w", "id\nx"));
        assert!(!content_matches("text starts 'id' /w", "idx"));
    }

    #[test]
    fn multiline_regex() {
        assert!(content_matches("text =~ `a\\nb` /m", "a\nb"));
        assert!(content_matches("text =~ `^b$` /m", "a\nb\nc"));
        assert!(!content_matches("text =~ `a.b` /m", "a\nb"));
        assert!(content_matches("text =~ `(?s)a.b` /m", "a\nb"));
    }

    #[test]
    fn line_groups() {
        assert!(content_matches(
            "line(text has 'a' and text has 'b')",
            "a\nab\nb"
        ));
        assert!(!content_matches(
            "line(text has 'a' and text has 'b')",
            "a\nb"
        ));
        assert!(content_matches("line(not text has 'a')", "a\nb"));
        assert!(!content_matches("line(not text has 'a')", "a\nab"));
        assert!(content_matches("not line(text has 'x')", "a\nb"));
        assert!(content_matches("line(text = 'ab')", "x\nab\ny"));
        assert!(content_matches(
            "line(text starts 'b' and text ends 'c')",
            "a\nbxc"
        ));
        assert!(content_matches("line(text =~ `^b`)", "a\nb"));
    }

    fn error(q: &str) -> (ErrorCode, String, Range<usize>, String) {
        let e = parse(q, &SENSITIVE).unwrap_err();
        (e.code, e.message, e.span, e.hint.unwrap_or_default())
    }

    #[test]
    fn len_in_lines() {
        assert!(content_matches("line(len = 2)", "\tx\nabc"));
        assert!(content_matches(
            "line(len = 3 and text has '\u{e4}')",
            "\u{e4}\u{f6}\u{fc}"
        ));
        assert!(content_matches("line(len in 0..0)", "a\n\nb"));
        assert!(!content_matches("line(len > 3)", "abc\nab"));
    }

    #[test]
    fn len_errors() {
        assert_eq!(
            error("len > 100"),
            (
                ErrorCode::LineTargetOutside,
                "'len' outside of line( ... )".into(),
                0..3,
                "'len' works only inside line( ... ) or near( ... )".into()
            )
        );
        assert_eq!(
            error("line(len > 1K)"),
            (
                ErrorCode::UnexpectedToken,
                "unexpected '1K'".into(),
                11..13,
                "len takes a number of characters, e.g. len > 120".into()
            )
        );
        assert_eq!(error("line(len in 10..1)").0, ErrorCode::InvalidRange);
    }

    #[test]
    fn near_matches() {
        assert!(content_matches(
            "near(text has 'a', text has 'b', 1)",
            "a\nb"
        ));
        assert!(content_matches(
            "near(text has 'a', text has 'b', 1)",
            "b\na"
        ));
        assert!(!content_matches(
            "near(text has 'a', text has 'b', 1, ordered)",
            "b\na"
        ));
        assert!(!content_matches(
            "near(text has 'a', text has 'b', 1)",
            "a\nx\nb"
        ));
        assert!(content_matches("near(text has 'a', text has 'b', 0)", "ba"));
        assert!(!content_matches(
            "near(text has 'a', text has 'b', 0, ordered)",
            "ba"
        ));
        assert!(content_matches(
            "near(text has 'a', text has 'b', 0, ordered)",
            "ab"
        ));
        assert!(content_matches(
            "near(text has 'a', len = 0, 1, first)",
            "a\n\nb"
        ));
    }

    #[test]
    fn near_option_errors() {
        assert_eq!(
            error("near(text has 'a', text has 'b', 3, nearest)"),
            (
                ErrorCode::InvalidNearOption,
                "unknown option 'nearest'".into(),
                36..43,
                "options: any, ordered, all, closest, first".into()
            )
        );
        assert_eq!(
            error("near(text has 'a', text has 'b', 3, closest, all)"),
            (
                ErrorCode::ConflictingOptions,
                "conflicting options 'closest' and 'all'".into(),
                45..48,
                "choose one of: all, closest, first".into()
            )
        );
        assert_eq!(
            error("near(name = 'a', text has 'b', 1)").1,
            "'name' inside near( ... )"
        );
    }

    #[test]
    fn line_group_errors() {
        assert_eq!(
            error("line(name ~ '*.rs')"),
            (
                ErrorCode::FileTargetInside,
                "'name' inside line( ... )".into(),
                5..9,
                "move 'name' out of line( ... )".into()
            )
        );
        assert_eq!(
            error("line(text has 'a' and size > 1)").1,
            "'size' inside line( ... )"
        );
        assert_eq!(
            error("line(line(text has 'a'))"),
            (
                ErrorCode::NestedLine,
                "nested line( ... )".into(),
                5..9,
                "line( ... ) can't be nested".into()
            )
        );
        assert_eq!(
            error("line(near(text has 'a', text has 'b', 1))").3,
            "line( ... ) can't contain near( ... )"
        );
    }

    #[test]
    fn regex_per_line() {
        assert!(content_matches("text =~ `^b$`", "a\nb\nc"));
        assert!(!content_matches("text =~ `a\\s+b`", "a\nb"));
        assert!(content_matches("text =~ `A\\d` /i", "xa1"));
        assert!(matcher("name =~ `^[a-c]\\.rs$`", false).is_match("b.rs"));
        assert!(matcher("name =~ `README` /i", false).is_match("readme.md"));
        assert!(matcher("path =~ `src/` ", false).is_match("a/src/b"));
    }

    #[test]
    fn invalid_regexes() {
        let hint = |q: &str| {
            let e = parse(q, &SENSITIVE).unwrap_err();
            assert_eq!(e.code, ErrorCode::InvalidRegex, "{q}");
            e.hint.unwrap()
        };
        assert_eq!(hint("text =~ `(?<=a)b`"), "look-arounds are not supported");
        assert_eq!(hint("text =~ `(a)\\1`"), "backreferences are not supported");
        assert_eq!(hint("name =~ `(x`"), "unclosed group");
        let e = parse("name =~ `(x`", &SENSITIVE).unwrap_err();
        assert_eq!((e.message.as_str(), e.span), ("invalid regex", 8..12));
    }

    #[test]
    fn unknown_content() {
        let q = parse("text has 'x'", &SENSITIVE).unwrap();
        let file = FileValues {
            name: "a",
            path: "a",
            entry_type: EntryType::File,
            is_link: false,
            empty: false,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            lang: "unknown",
            kind: Kind::Other,
            git: None,
            content: ContentState::Unknown,
        };
        assert_eq!(q.filter.unwrap().eval(&file), Truth::Unknown);
    }

    #[test]
    fn has_type_anywhere() {
        assert!(!parse("name = 'a'", &SENSITIVE).unwrap().has_type);
        assert!(
            parse("name = 'a' or not type = dir", &SENSITIVE)
                .unwrap()
                .has_type
        );
        assert!(!parse("", &SENSITIVE).unwrap().has_type);
    }

    #[test]
    fn unknown_type_values() {
        let e = parse("type = folder", &SENSITIVE).unwrap_err();
        assert_eq!(
            (e.code, e.message.as_str(), e.span, e.hint.as_deref()),
            (
                ErrorCode::UnknownValue,
                "unknown value 'folder'",
                7..13,
                Some("did you mean 'dir'? expected one of: file, dir, link")
            )
        );
        let e = parse("type in (file, sym)", &SENSITIVE).unwrap_err();
        assert_eq!(e.span, 15..18);
        assert_eq!(e.hint.as_deref(), Some("expected one of: file, dir, link"));
    }

    #[test]
    fn the_empty_query_has_no_filter_and_no_legend() {
        let q = parse(" \n ", &SENSITIVE).unwrap();
        assert!(q.filter.is_none());
        assert!(q.legend.is_empty());
    }

    #[test]
    fn invalid_wildcard() {
        let e = parse("name ~ '[a-'", &SENSITIVE).unwrap_err();
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
            &SENSITIVE,
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
        let q = parse("name = 'a' @x or name = 'b'", &SENSITIVE).unwrap();
        let ids: Vec<String> = q.legend.iter().map(|(id, _)| id.to_string()).collect();
        assert_eq!(ids, ["@x", "#1"]);
    }

    #[test]
    fn the_earliest_error_wins() {
        let e = parse("name ~ '[a-' and path ~ 'x'", &SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::InvalidWildcard);
        let e = parse("name ~~ 'x' and name ~ '[a-'", &SENSITIVE).unwrap_err();
        assert_eq!(e.code, ErrorCode::InvalidWildcard);
    }
}
