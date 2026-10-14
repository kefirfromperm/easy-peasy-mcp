//! The `replace` tool: the matches of groups of the query replaced by templates (TOOLS.md#replace).

use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::content::{self, Content, DecodeOptions};
use crate::error::{ErrorCode, QueryError, Report};
use crate::output::Fragment;
use crate::query::Filter;
use crate::query::Query;
use crate::query::ast::CondId;
use crate::query::escape::{self, Style};
use crate::query::naming;
use crate::search::FileMatches;
use crate::warning::WarningCode;
use crate::write::{self, Acted, Entry, Outcome};

/// The fields of `replace`, in the order of its input schema.
pub(crate) const REPLACE_FIELDS: [&str; 13] = [
    "version",
    "result",
    "query",
    "dirs",
    "traversal",
    "kinds",
    "languages",
    "replace",
    "expect",
    "expect_matches",
    "dry_run",
    "timeout",
    "output",
];

/// The fields of `output` of `replace`.
pub(crate) const OUTPUT_FIELDS: [&str; 3] = ["format", "group_dirs", "max_lines"];

/// The fields of the `find` of a result which select its files again (TOOLS.md#replace).
const RESULT_FIELDS: [&str; 7] = [
    "version",
    "query",
    "traversal",
    "kinds",
    "languages",
    "escape",
    "query_options",
];

/// The hint of a missing or wrong `replace`.
const HINT: &str = r#"the groups of the query and their new text, e.g. {"@old": "new"}"#;

/// A key of `replace` as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeySpec {
    pub key: String,
    pub template: String,
    /// `/n` matches get the new text in their naming style.
    pub naming: bool,
}

/// The checked fields of `replace`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Spec {
    /// In the order of their names.
    pub keys: Vec<KeySpec>,
    pub expect_matches: Option<usize>,
}

/// Checks `replace` and `expect_matches`.
pub(crate) fn parse(arguments: &Value) -> Result<Spec, Report> {
    let replace = arguments
        .get("replace")
        .ok_or_else(|| Report::new(ErrorCode::InvalidOption, "'replace' is required").hint(HINT))?;
    let Some(map) = replace.as_object().filter(|m| !m.is_empty()) else {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'replace' must be an object of groups and templates",
        )
        .hint(HINT));
    };
    let mut keys = Vec::new();
    for (key, value) in map {
        let invalid = || {
            Report::new(
                ErrorCode::InvalidOption,
                format!(
                    r#"'replace.{key}' must be a template or {{"template": "...", "naming": false}}"#
                ),
            )
        };
        let (template, naming) = match value {
            Value::String(template) => (template.clone(), true),
            Value::Object(object) => {
                if object.keys().any(|k| k != "template" && k != "naming") {
                    return Err(invalid());
                }
                let Some(Value::String(template)) = object.get("template") else {
                    return Err(invalid());
                };
                let naming = match object.get("naming") {
                    None => true,
                    Some(Value::Bool(naming)) => *naming,
                    Some(_) => {
                        return Err(Report::new(
                            ErrorCode::InvalidOption,
                            format!("'replace.{key}.naming' must be a boolean"),
                        ));
                    }
                };
                (template.clone(), naming)
            }
            _ => return Err(invalid()),
        };
        keys.push(KeySpec {
            key: key.clone(),
            template,
            naming,
        });
    }
    let expect_matches = match arguments.get("expect_matches") {
        None => None,
        Some(value) => Some(
            value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| {
                    Report::new(
                        ErrorCode::InvalidOption,
                        "'expect_matches' must be a whole number",
                    )
                })?,
        ),
    };
    Ok(Spec {
        keys,
        expect_matches,
    })
}

/// The arguments of the `find` which searches the files of the kept result `id` again: the query and the options of
/// the result's `find`, the timeout and the output of the selection of `replace`.
pub(crate) fn result_request(kept: &Value, id: u64, selection: &Value) -> Value {
    let mut find = serde_json::Map::new();
    for field in RESULT_FIELDS {
        if let Some(value) = kept.get(field) {
            find.insert(field.into(), value.clone());
        }
    }
    for field in ["timeout", "output"] {
        if let Some(value) = selection.get(field) {
            find.insert(field.into(), value.clone());
        }
    }
    find.insert("within".into(), Value::from(format!("r{id}")));
    Value::Object(find)
}

/// A part of a template.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Part {
    Text(String),
    /// The match of the group (`None`) or one of its sub-groups by its reference, `@def.1`.
    Ref(Option<String>),
}

/// A checked template of a group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Template {
    parts: Vec<Part>,
}

impl Template {
    /// Parses `text`, the template of the group `id` whose sub-groups are `subs`; the error has its byte range in
    /// `text`.
    fn parse(text: &str, id: &str, subs: &[String]) -> Result<Self, QueryError> {
        let hint = || {
            let placeholders: Vec<String> = std::iter::once(id)
                .chain(subs.iter().map(String::as_str))
                .map(|r| format!("{{{r}}}"))
                .collect();
            format!("the placeholders of '{id}' are {}", placeholders.join(", "))
        };
        let mut parts = Vec::new();
        let mut literal = String::new();
        let mut rest = text.char_indices().peekable();
        while let Some((i, c)) = rest.next() {
            match c {
                '{' if rest.peek().is_some_and(|(_, n)| *n == '{') => {
                    rest.next();
                    literal.push('{');
                }
                '}' if rest.peek().is_some_and(|(_, n)| *n == '}') => {
                    rest.next();
                    literal.push('}');
                }
                '}' => {
                    return Err(QueryError::new(
                        ErrorCode::InvalidTemplate,
                        "unmatched '}'",
                        i..i + 1,
                    )
                    .hint("write '}}' for a brace"));
                }
                '{' => {
                    let Some(len) = text[i + 1..].find('}') else {
                        return Err(QueryError::new(
                            ErrorCode::InvalidTemplate,
                            "unclosed '{'",
                            i..text.len(),
                        )
                        .hint("write '{{' for a brace"));
                    };
                    let end = i + 1 + len;
                    let name = &text[i + 1..end];
                    let span = i..end + 1;
                    while rest.peek().is_some_and(|(j, _)| *j <= end) {
                        rest.next();
                    }
                    let part = if name.is_empty() {
                        return Err(QueryError::new(
                            ErrorCode::InvalidTemplate,
                            "empty placeholder",
                            span,
                        )
                        .hint(hint()));
                    } else if name == id {
                        Part::Ref(None)
                    } else if subs.iter().any(|s| s == name) {
                        Part::Ref(Some(name.to_string()))
                    } else if name.starts_with(['@', '#']) {
                        return Err(QueryError::new(
                            ErrorCode::InvalidTemplate,
                            format!("'{{{name}}}' is not '{id}' or its sub-group"),
                            span,
                        )
                        .hint(hint()));
                    } else {
                        return Err(QueryError::new(
                            ErrorCode::InvalidTemplate,
                            format!("unknown placeholder '{name}'"),
                            span,
                        )
                        .hint(hint()));
                    };
                    if !literal.is_empty() {
                        parts.push(Part::Text(std::mem::take(&mut literal)));
                    }
                    parts.push(part);
                }
                c => literal.push(c),
            }
        }
        if !literal.is_empty() {
            parts.push(Part::Text(literal));
        }
        Ok(Self { parts })
    }

    /// The new text of `fragment` of `content`: the literal parts escaped `escape` (a style and a level) and the
    /// references as matched.
    fn expand(
        &self,
        content: &Content,
        fragment: &Fragment,
        escape: Option<(Style, u8)>,
    ) -> String {
        let text = content.text();
        let mut out = String::new();
        for part in &self.parts {
            match part {
                Part::Text(literal) => match escape {
                    Some((style, level)) => {
                        out.push_str(&escape::escape_first(style, literal, level));
                    }
                    None => out.push_str(literal),
                },
                Part::Ref(None) => out.push_str(&text[fragment.range.clone()]),
                Part::Ref(Some(sub)) => {
                    if let Some(group) = fragment.groups.iter().find(|g| g.ids.contains(sub)) {
                        out.push_str(&text[group.range.clone()]);
                    }
                }
            }
        }
        out
    }
}

/// A checked key of `replace`: its group, its template, whether `/n` matches get their naming style.
#[derive(Debug, Clone)]
pub(crate) struct Rule {
    id: CondId,
    template: Template,
    naming: bool,
}

/// Checks the keys and templates of `spec` against the groups of `query`, in the order of the keys.
pub(crate) fn compile(spec: &Spec, query: &Query) -> Result<Vec<Rule>, Report> {
    let mut rules = Vec::new();
    for key in &spec.keys {
        let group = query
            .group(&key.key)
            .map_err(|e| e.into_field_report("replace", &key.key))?;
        if group.sub.is_some() {
            let base = key.key.split_once('.').map_or(key.key.as_str(), |(b, _)| b);
            return Err(Report::new(
                ErrorCode::InvalidOption,
                format!("'{}' is a sub-group; replace the whole group", key.key),
            )
            .hint(format!(r#"e.g. {{"{base}": "...{{{}}}..."}}"#, key.key)));
        }
        let has_text = query
            .filter
            .as_ref()
            .and_then(|f| f.find(&group.id))
            .is_some_and(Filter::has_text);
        if !has_text {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                format!("the group '{}' has no text condition", key.key),
            )
            .hint("replace changes the text of files; rename changes their names"));
        }
        let id = group.id.to_string();
        let template = Template::parse(&key.template, &id, &query.sub_group_names(&group.id))
            .map_err(|e| e.into_field_report("replace", &key.template))?;
        rules.push(Rule {
            id: group.id,
            template,
            naming: key.naming,
        });
    }
    Ok(rules)
}

/// A run of changed lines of a file (TOOLS.md#result-5): the old lines from `line`, the new ones from `new_line`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Change {
    pub line: usize,
    pub old: Vec<String>,
    pub new_line: usize,
    pub new: Vec<String>,
}

/// A match to replace: its range in the normalized text and its new text, with `\n` line breaks.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Replacement {
    range: Range<usize>,
    text: String,
}

/// The replacements of the matches of the rules among the fragments of a file; `replace_conflict` when matches of
/// two rules overlap.
fn replacements(
    rules: &[Rule],
    fragments: &[Fragment],
    content: &Content,
    filter: Option<&Filter>,
) -> Result<Vec<Replacement>, WarningCode> {
    let mut chosen: Vec<(usize, &Fragment)> = Vec::new();
    for (k, rule) in rules.iter().enumerate() {
        let mut own: Vec<&Fragment> = fragments
            .iter()
            .filter(|f| f.ids.iter().any(|(_, id)| *id == rule.id))
            .collect();
        own.sort_by_key(|f| (f.range.start, std::cmp::Reverse(f.range.end)));
        let mut end = None;
        for f in own {
            if end.is_none_or(|end| f.range.start >= end) {
                end = Some(f.range.end);
                chosen.push((k, f));
            }
        }
    }
    chosen.sort_by_key(|(_, f)| (f.range.start, f.range.end));
    for pair in chosen.windows(2) {
        let (a, b) = (&pair[0].1.range, &pair[1].1.range);
        if b.start < a.end || a.start == b.start {
            return Err(WarningCode::ReplaceConflict);
        }
    }
    Ok(chosen
        .into_iter()
        .map(|(k, f)| {
            let rule = &rules[k];
            let pattern = f.ids.iter().find_map(|(_, id)| match filter?.find(id)? {
                Filter::Content { pattern, .. } => Some(pattern),
                _ => None,
            });
            let escape = pattern.and_then(|p| p.escape_style).zip(f.escape);
            let mut text = rule.template.expand(content, f, escape);
            if rule.naming && pattern.is_some_and(|p| p.naming) {
                text = naming::restyle(&text, &content.text()[f.range.clone()]);
            }
            Replacement {
                range: f.range.clone(),
                text,
            }
        })
        .collect())
}

/// What `replace` does with a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Planned {
    /// No match, or the new text is the old one.
    Unchanged,
    /// The new bytes of the file, the number of matches and the changed lines.
    Change {
        bytes: Vec<u8>,
        matches: usize,
        changes: Vec<Change>,
    },
}

/// The new content of the file at `path` with the matches of the rules among the fragments of the search replaced,
/// written in its encoding; the warning the file is skipped with.
pub(crate) fn plan(
    path: &Path,
    found: Option<&FileMatches>,
    rules: &[Rule],
    filter: Option<&Filter>,
    decode: &DecodeOptions,
) -> Result<Planned, Plan> {
    let meta = std::fs::symlink_metadata(path).map_err(|e| Plan::Failed(e.kind().to_string()))?;
    let Some(found) = found.filter(|_| meta.is_file()) else {
        return Ok(Planned::Unchanged);
    };
    let replacements =
        replacements(rules, &found.fragments, &found.content, filter).map_err(Plan::Skip)?;
    if replacements.is_empty() {
        return Ok(Planned::Unchanged);
    }
    let bytes = std::fs::read(path).map_err(|e| Plan::Failed(e.kind().to_string()))?;
    let raw = content::decode_raw(&bytes, decode).ok_or(Plan::Skip(WarningCode::FileChanged))?;
    if raw.errors {
        return Err(Plan::Skip(WarningCode::Unencodable));
    }
    if Content::from_text(raw.text.clone()).text() != found.content.text() {
        return Err(Plan::Skip(WarningCode::FileChanged));
    }
    if content::encode(&raw.text, raw.encoding).as_deref() != Some(&bytes[raw.bom.len()..]) {
        return Err(Plan::Skip(WarningCode::Unencodable));
    }
    let new_raw = apply_raw(&raw.text, &replacements);
    if new_raw == raw.text {
        return Ok(Planned::Unchanged);
    }
    let mut new_bytes = raw.bom.clone();
    new_bytes.extend(
        content::encode(&new_raw, raw.encoding).ok_or(Plan::Skip(WarningCode::Unencodable))?,
    );
    Ok(Planned::Change {
        bytes: new_bytes,
        matches: replacements.len(),
        changes: changes(&found.content, &replacements),
    })
}

/// Why a file is not planned: a warning, or the error of the operating system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Plan {
    Skip(WarningCode),
    Failed(String),
}

/// The line break of a raw text: its first one, `\n` without one.
fn line_break(raw: &str) -> &'static str {
    match raw.find('\n') {
        Some(i) if raw[..i].ends_with('\r') => "\r\n",
        _ => "\n",
    }
}

/// The raw text with the replacements (ranges of the normalized text) applied: the `\r` of the `\r\n` line breaks
/// counted, the line breaks of the new text written like the ones of the file.
fn apply_raw(raw: &str, replacements: &[Replacement]) -> String {
    // the normalized offsets of the `\n` which follow a `\r`
    let crlf: Vec<usize> = raw
        .match_indices("\r\n")
        .enumerate()
        .map(|(removed, (i, _))| i - removed)
        .collect();
    let to_raw = |offset: usize| offset + crlf.partition_point(|&p| p < offset);
    let newline = line_break(raw);
    let mut out = String::with_capacity(raw.len());
    let mut at = 0;
    for r in replacements {
        let (start, end) = (to_raw(r.range.start), to_raw(r.range.end));
        out.push_str(&raw[at..start]);
        out.push_str(&r.text.replace('\n', newline));
        at = end;
    }
    out.push_str(&raw[at..]);
    out
}

/// The runs of changed lines: the old lines touched by the replacements, consecutive ones joined, and the new lines
/// in their place.
fn changes(content: &Content, replacements: &[Replacement]) -> Vec<Change> {
    let old = content.text();
    let mut new = String::with_capacity(old.len());
    let mut at = 0;
    for r in replacements {
        new.push_str(&old[at..r.range.start]);
        new.push_str(&r.text);
        at = r.range.end;
    }
    new.push_str(&old[at..]);
    let mut runs: Vec<Range<usize>> = Vec::new();
    for r in replacements {
        let (first, last) = (content.line_of(r.range.start), content.line_of(r.range.end));
        match runs.last_mut() {
            Some(run) if first <= run.end + 1 => run.end = run.end.max(last),
            _ => runs.push(first..last),
        }
    }
    // the length of the new text minus the old one of the replacements before an offset
    let delta = |offset: usize, inclusive: bool| -> isize {
        replacements
            .iter()
            .filter(|r| r.range.start < offset || (inclusive && r.range.start == offset))
            .map(|r| r.text.len().cast_signed() - r.range.len().cast_signed())
            .sum()
    };
    runs.into_iter()
        .map(|run| {
            let (start, _) = content.line_range(run.start);
            let (_, end) = content.line_range(run.end);
            let new_start = start.saturating_add_signed(delta(start, false));
            let new_end = end.saturating_add_signed(delta(end, true));
            Change {
                line: run.start + 1,
                old: (run.start..=run.end)
                    .map(|i| content.line(i).to_string())
                    .collect(),
                new_line: new[..new_start].matches('\n').count() + 1,
                new: new[new_start..new_end]
                    .split('\n')
                    .map(str::to_string)
                    .collect(),
            }
        })
        .collect()
}

/// How `replace` acts on the entries.
pub(crate) struct How<'a> {
    /// Whether a canonical path is inside the allowed roots.
    pub allowed: &'a dyn Fn(&Path) -> bool,
    /// Whether the call must stop: its deadline, its cancellation.
    pub stop: &'a dyn Fn() -> bool,
    pub decode: &'a DecodeOptions,
    pub filter: Option<&'a Filter>,
    pub dry_run: bool,
    pub expect_matches: Option<usize>,
    pub max_lines: usize,
}

/// The matches of the search by the shown path and the member of a file.
pub(crate) type Matches = HashMap<(PathBuf, Option<String>), FileMatches>;

/// Plans every entry in its order, checks `expect_matches`, then writes the changed files until the call must stop.
pub(crate) fn replace(
    entries: &[Entry],
    matches: &Matches,
    rules: &[Rule],
    how: &How<'_>,
) -> Result<Outcome, Report> {
    let mut outcome = Outcome {
        dry_run: how.dry_run,
        selected: entries.len(),
        max_lines: Some(how.max_lines),
        ..Outcome::default()
    };
    let mut planned = Vec::new();
    for entry in entries {
        if (how.stop)() {
            // nothing is written when the plan is incomplete
            outcome.warnings.add_global(WarningCode::Timeout);
            return Ok(outcome);
        }
        let path = match write::check(entry, how.allowed) {
            Ok(Some(path)) => path,
            Ok(None) => {
                planned.push((entry, None, Err(write::not_found())));
                continue;
            }
            Err(code) => {
                outcome.skip(entry, code);
                continue;
            }
        };
        let found = matches.get(&(entry.shown.clone(), entry.member.clone()));
        match plan(&path, found, rules, how.filter, how.decode) {
            Ok(step) => planned.push((entry, Some(path), Ok(step))),
            Err(Plan::Skip(code)) => outcome.skip(entry, code),
            Err(Plan::Failed(error)) => planned.push((entry, None, Err(error))),
        }
    }
    let total: usize = planned
        .iter()
        .map(|(_, _, step)| match step {
            Ok(Planned::Change { matches, .. }) => *matches,
            _ => 0,
        })
        .sum();
    if let Some(expected) = how.expect_matches.filter(|e| *e != total) {
        return Err(Report::new(
            ErrorCode::CountMismatch,
            format!("the selection has {total} matches, expected {expected}"),
        )
        .hint("nothing was changed; check the selection with dry_run: true"));
    }
    for (entry, path, step) in planned {
        if (how.stop)() {
            outcome.warnings.add_global(WarningCode::Timeout);
            break;
        }
        let acted = match step {
            Err(error) => Acted::of(entry, Some(error)),
            Ok(Planned::Unchanged) => Acted {
                unchanged: true,
                ..Acted::of(entry, None)
            },
            Ok(Planned::Change {
                bytes,
                matches,
                changes,
            }) => {
                let written = match &path {
                    Some(path) if !how.dry_run => write_file(path, &bytes).err(),
                    _ => None,
                };
                match written {
                    Some(error) => Acted::of(entry, Some(error.kind().to_string())),
                    None => Acted {
                        matches: Some(matches),
                        changes,
                        ..Acted::of(entry, None)
                    },
                }
            }
        };
        outcome.acted.push(acted);
    }
    Ok(outcome)
}

/// Writes `bytes` to a temporary file next to `path` with its permissions and renames it into place; a failed write
/// leaves `path` untouched and removes the temporary file.
fn write_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
    let temporary = path.with_file_name(crate::rename::temporary_name(name));
    let permissions = std::fs::metadata(path)?.permissions();
    let written = std::fs::write(&temporary, bytes)
        .and_then(|()| std::fs::set_permissions(&temporary, permissions))
        .and_then(|()| std::fs::rename(&temporary, path));
    if written.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::SubMatch;
    use serde_json::json;

    fn text(arguments: &Value) -> String {
        parse(arguments).unwrap_err().to_text()
    }

    #[test]
    fn replace_and_expect_matches_which_are_rejected() {
        assert!(text(&json!({})).starts_with("error[invalid_option]: 'replace' is required\n"));
        for value in [json!({}), json!("x"), json!(["@a"])] {
            assert!(text(&json!({ "replace": value })).starts_with(
                "error[invalid_option]: 'replace' must be an object of groups and templates\n"
            ));
        }
        assert_eq!(
            text(&json!({"replace": {"@a": 1}})),
            "error[invalid_option]: 'replace.@a' must be a template or {\"template\": \"...\", \"naming\": false}\n"
        );
        assert!(
            text(&json!({"replace": {"@a": {"naming": false}}})).contains("'replace.@a' must be")
        );
        assert!(
            text(&json!({"replace": {"@a": {"template": "x", "x": 1}}}))
                .contains("'replace.@a' must be")
        );
        assert_eq!(
            text(&json!({"replace": {"@a": {"template": "x", "naming": "no"}}})),
            "error[invalid_option]: 'replace.@a.naming' must be a boolean\n"
        );
        assert_eq!(
            text(&json!({"replace": {"@a": "x"}, "expect_matches": 1.5})),
            "error[invalid_option]: 'expect_matches' must be a whole number\n"
        );
        let spec = parse(&json!({"replace": {"@b": "y", "@a": {"template": "x", "naming": false}}, "expect_matches": 3})).unwrap();
        assert_eq!(spec.expect_matches, Some(3));
        let keys: Vec<(&str, &str, bool)> = spec
            .keys
            .iter()
            .map(|k| (k.key.as_str(), k.template.as_str(), k.naming))
            .collect();
        assert_eq!(keys, [("@a", "x", false), ("@b", "y", true)]);
    }

    fn template_error(text: &str) -> String {
        Template::parse(text, "@def", &["@def.1".into(), "@def.fname".into()])
            .unwrap_err()
            .into_field_report("replace", text)
            .to_text()
    }

    #[test]
    fn templates_and_their_errors() {
        let template = Template::parse(
            "{{fn}} {@def.fname}_v2({@def.1}) {@def}",
            "@def",
            &["@def.1".into(), "@def.fname".into()],
        )
        .unwrap();
        let content = Content::from_text("fn load(a)".into());
        let fragment = Fragment {
            range: 0..10,
            ids: vec![(1, CondId::Label("def".into()))],
            escape: None,
            groups: vec![SubMatch {
                range: 3..7,
                ids: vec!["@def.fname".into()],
            }],
        };
        assert_eq!(
            template.expand(&content, &fragment, None),
            "{fn} load_v2() fn load(a)"
        );
        let hint = "  hint: the placeholders of '@def' are {@def}, {@def.1}, {@def.fname}\n";
        assert_eq!(
            template_error("{@x}_v2"),
            format!(
                "error[invalid_template]: '{{@x}}' is not '@def' or its sub-group\n  {{@x}}_v2\n  ^^^^\n{hint}"
            )
        );
        assert_eq!(
            template_error("a{name}"),
            format!(
                "error[invalid_template]: unknown placeholder 'name'\n  a{{name}}\n   ^^^^^^\n{hint}"
            )
        );
        assert!(template_error("{}").starts_with("error[invalid_template]: empty placeholder\n"));
        assert_eq!(
            template_error("x{@def"),
            "error[invalid_template]: unclosed '{'\n  x{@def\n   ^^^^^\n  hint: write '{{' for a brace\n"
        );
        assert_eq!(
            template_error("@def}"),
            "error[invalid_template]: unmatched '}'\n  @def}\n      ^\n  hint: write '}}' for a brace\n"
        );
    }

    #[test]
    fn literal_text_is_escaped_like_the_match() {
        let template = Template::parse(r#""role": "{@r}""#, "@r", &[]).unwrap();
        let content = Content::from_text("x".into());
        let fragment = Fragment {
            range: 0..1,
            ids: Vec::new(),
            escape: Some(1),
            groups: Vec::new(),
        };
        assert_eq!(
            template.expand(&content, &fragment, Some((Style::Backslash, 1))),
            r#"\"role\": \"x\""#
        );
    }

    fn fragment(range: Range<usize>, ids: &[&str]) -> Fragment {
        Fragment {
            range,
            ids: ids
                .iter()
                .enumerate()
                .map(|(i, id)| (i, CondId::Label((*id).into())))
                .collect(),
            escape: None,
            groups: Vec::new(),
        }
    }

    fn rule(id: &str, template: &str) -> Rule {
        let id = format!("@{id}");
        Rule {
            id: CondId::Label(id[1..].into()),
            template: Template::parse(template, &id, &[]).unwrap(),
            naming: true,
        }
    }

    #[test]
    fn overlapping_matches() {
        let content = Content::from_text("serde_json serde".into());
        // in one group the first start wins, at the same start the longer one
        let found = replacements(
            &[rule("ser", "X")],
            &[
                fragment(0..5, &["ser"]),
                fragment(0..10, &["ser"]),
                fragment(6..10, &["ser"]),
                fragment(11..16, &["ser"]),
            ],
            &content,
            None,
        )
        .unwrap();
        let ranges: Vec<Range<usize>> = found.iter().map(|r| r.range.clone()).collect();
        assert_eq!(ranges, [0..10, 11..16]);
        // between two groups a conflict
        assert_eq!(
            replacements(
                &[rule("a", "X"), rule("b", "Y")],
                &[fragment(0..10, &["a"]), fragment(6..10, &["b"])],
                &content,
                None,
            ),
            Err(WarningCode::ReplaceConflict)
        );
        // a group and a condition inside it: the same fragment
        assert_eq!(
            replacements(
                &[rule("g", "X"), rule("c", "Y")],
                &[fragment(0..5, &["g", "c"])],
                &content,
                None,
            ),
            Err(WarningCode::ReplaceConflict)
        );
        // adjacent matches don't overlap
        assert_eq!(
            replacements(
                &[rule("a", "X"), rule("b", "Y")],
                &[fragment(0..5, &["a"]), fragment(5..10, &["b"])],
                &content,
                None,
            )
            .map(|r| r.len()),
            Ok(2)
        );
    }

    fn replacement(range: Range<usize>, text: &str) -> Replacement {
        Replacement {
            range,
            text: text.into(),
        }
    }

    #[test]
    fn crlf_line_breaks_are_kept() {
        let raw = "one\r\ntwo\r\nthree\r\n";
        // `two` is at 4..7 of the normalized text
        assert_eq!(
            apply_raw(raw, &[replacement(4..7, "2\n2b")]),
            "one\r\n2\r\n2b\r\nthree\r\n"
        );
        // a match with the line break of `one`
        assert_eq!(
            apply_raw(raw, &[replacement(0..4, "1 ")]),
            "1 two\r\nthree\r\n"
        );
        assert_eq!(
            apply_raw("a\nb\r\nc", &[replacement(4..5, "C")]),
            "a\nb\r\nC"
        );
        assert_eq!(apply_raw("a\rb", &[replacement(2..3, "B")]), "a\rB");
    }

    #[test]
    fn runs_of_changed_lines() {
        let content = Content::from_text("a1\nb\nc1\nd1\ne\n".into());
        let runs = changes(
            &content,
            &[
                replacement(1..2, "X"),
                replacement(6..7, "Y\nZ"),
                replacement(9..10, ""),
            ],
        );
        assert_eq!(
            runs,
            [
                Change {
                    line: 1,
                    old: vec!["a1".into()],
                    new_line: 1,
                    new: vec!["aX".into()],
                },
                Change {
                    line: 3,
                    old: vec!["c1".into(), "d1".into()],
                    new_line: 3,
                    new: vec!["cY".into(), "Z".into(), "d".into()],
                },
            ]
        );
        // a match over a line break joins two lines
        let content = Content::from_text("one\ntwo\nthree\n".into());
        assert_eq!(
            changes(&content, &[replacement(2..5, "e-t")]),
            [Change {
                line: 1,
                old: vec!["one".into(), "two".into()],
                new_line: 1,
                new: vec!["one-two".into()],
            }]
        );
        // an insertion at the end of a line
        assert_eq!(
            changes(&content, &[replacement(13..13, "!")])[0].new,
            ["three!"]
        );
    }

    #[test]
    fn files_are_written_in_their_encoding() {
        let dir = tempfile::tempdir().unwrap();
        let decode = DecodeOptions::default();
        let rules = [rule("w", "there")];
        let run = |name: &str, bytes: &[u8], decode: &DecodeOptions| {
            let path = dir.path().join(name);
            std::fs::write(&path, bytes).unwrap();
            let raw = content::decode_raw(bytes, decode).unwrap();
            let content = Content::from_text(raw.text);
            let start = content.text().find("world").unwrap();
            let found = FileMatches {
                fragments: vec![fragment(start..start + 5, &["w"])],
                lines: 1,
                content,
            };
            plan(&path, Some(&found), &rules, None, decode)
        };
        let bytes = |planned: Result<Planned, Plan>| match planned {
            Ok(Planned::Change { bytes, .. }) => bytes,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            bytes(run("bom.txt", b"\xef\xbb\xbfhello world", &decode)),
            b"\xef\xbb\xbfhello there"
        );
        let utf16be: Vec<u8> = b"\xfe\xff"
            .iter()
            .copied()
            .chain("hello world".encode_utf16().flat_map(u16::to_be_bytes))
            .collect();
        let expected: Vec<u8> = b"\xfe\xff"
            .iter()
            .copied()
            .chain("hello there".encode_utf16().flat_map(u16::to_be_bytes))
            .collect();
        assert_eq!(bytes(run("be.txt", &utf16be, &decode)), expected);
        let latin = DecodeOptions {
            encoding: Some(encoding_rs::WINDOWS_1252),
            ..DecodeOptions::default()
        };
        assert_eq!(
            bytes(run("latin.txt", b"caf\xe9 world", &latin)),
            b"caf\xe9 there"
        );
        // invalid UTF-8 can't be written back
        assert_eq!(
            run("bad.txt", b"\xff world", &decode),
            Err(Plan::Skip(WarningCode::Unencodable))
        );
        // a character windows-1252 doesn't have
        let path = dir.path().join("latin.txt");
        std::fs::write(&path, b"world").unwrap();
        let found = FileMatches {
            fragments: vec![fragment(0..5, &["w"])],
            lines: 1,
            content: Content::from_text("world".into()),
        };
        assert_eq!(
            plan(&path, Some(&found), &[rule("w", "мир")], None, &latin),
            Err(Plan::Skip(WarningCode::Unencodable))
        );
        // a file whose text changed since the search
        assert_eq!(
            plan(&path, Some(&found), &rules, None, &decode).map(|_| ()),
            Ok(())
        );
        std::fs::write(&path, b"world!").unwrap();
        assert_eq!(
            plan(&path, Some(&found), &rules, None, &decode),
            Err(Plan::Skip(WarningCode::FileChanged))
        );
        // the template gives the old text
        std::fs::write(&path, b"world").unwrap();
        assert_eq!(
            plan(&path, Some(&found), &[rule("w", "{@w}")], None, &decode),
            Ok(Planned::Unchanged)
        );
    }

    #[test]
    fn a_written_file_keeps_its_permissions_and_leaves_no_temporary_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.txt");
        std::fs::write(&path, "old").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
        }
        write_file(&path, b"new").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o640);
        }
        let names: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(names, ["a.txt"]);
    }
}
