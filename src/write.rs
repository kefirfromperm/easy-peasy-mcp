//! The write tools: their arguments, the selection, acting on it and the response (TOOLS.md#write-tools).

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use crate::cache::Stamp;
use crate::error::{ErrorCode, Report};
use crate::output::{display_path, split};
use crate::query::did_you_mean;
use crate::recycle::TrashError;
use crate::request;
use crate::warning::{WarningCode, Warnings};

/// The fields of `delete`, in the order of its input schema.
pub(crate) const DELETE_FIELDS: [&str; 12] = [
    "version",
    "result",
    "query",
    "dirs",
    "traversal",
    "kinds",
    "languages",
    "expect",
    "dry_run",
    "permanent",
    "timeout",
    "output",
];

/// The fields which select files together with `query`.
const QUERY_FIELDS: [&str; 4] = ["dirs", "traversal", "kinds", "languages"];

/// The fields of `output` of a write tool.
const OUTPUT_FIELDS: [&str; 2] = ["format", "group_dirs"];

/// What a write tool acts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Selection {
    /// The files of a kept result `rN`.
    Result(u64),
    /// The entries of the query of `find`.
    Query,
}

/// A checked request of a write tool.
#[derive(Debug, Clone)]
pub(crate) struct WriteRequest {
    pub selection: Selection,
    /// The arguments of the `find` request which selects the entries (or carries the version, the timeout and the
    /// output options of a result).
    pub find: Value,
    /// `find` checked.
    pub request: request::Request,
    pub expect: Option<usize>,
    pub dry_run: bool,
    /// `delete`: `permanent`.
    pub permanent: bool,
}

/// Checks the arguments of a write tool with the fields `fields`.
pub(crate) fn parse(arguments: &Value, fields: &[&str]) -> Result<WriteRequest, Report> {
    let empty = Map::new();
    let object = match arguments {
        Value::Null => &empty,
        Value::Object(object) => object,
        _ => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "the arguments must be an object",
            ));
        }
    };
    if let Some(unknown) = object.keys().find(|k| !fields.contains(&k.as_str())) {
        return Err(unknown_field(unknown, fields));
    }
    let selection = match (object.get("result"), object.contains_key("query")) {
        (Some(_), true) => {
            return Err(Report::new(
                ErrorCode::InvalidOption,
                "'result' and 'query' exclude each other",
            ));
        }
        (None, false) => {
            return Err(
                Report::new(ErrorCode::InvalidOption, "'result' or 'query' is required")
                    .hint("pass the result id of a find, or a query"),
            );
        }
        (Some(result), false) => {
            if let Some(field) = QUERY_FIELDS.iter().find(|f| object.contains_key(**f)) {
                return Err(Report::new(
                    ErrorCode::InvalidOption,
                    format!("'{field}' works only with 'query'"),
                ));
            }
            Selection::Result(request::result_id("result", result)?)
        }
        (None, true) => Selection::Query,
    };
    let expect = match object.get("expect") {
        None => None,
        Some(value) => Some(
            value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| {
                    Report::new(ErrorCode::InvalidOption, "'expect' must be a whole number")
                })?,
        ),
    };
    let flag = |name: &str| match object.get(name) {
        None => Ok(false),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => Err(Report::new(
            ErrorCode::InvalidOption,
            format!("'{name}' must be a boolean"),
        )),
    };
    let dry_run = flag("dry_run")?;
    let permanent = flag("permanent")?;
    let mut output = output_fields(object.get("output"))?;
    // the selection is all the matched entries in path order, without the funnel
    output.insert("detail".into(), json!("files"));
    output.insert("sort".into(), json!("path asc"));
    output.insert("funnel".into(), json!("never"));
    let mut find = Map::new();
    for field in [
        "version",
        "query",
        "dirs",
        "traversal",
        "kinds",
        "languages",
        "timeout",
    ] {
        if let Some(value) = object.get(field) {
            find.insert(field.into(), value.clone());
        }
    }
    find.insert("output".into(), Value::Object(output));
    let find = Value::Object(find);
    let request = request::parse(&find)?;
    Ok(WriteRequest {
        selection,
        find,
        request,
        expect,
        dry_run,
        permanent,
    })
}

/// The `output` object of a write tool: only `format` and `group_dirs`.
fn output_fields(output: Option<&Value>) -> Result<Map<String, Value>, Report> {
    match output {
        None => Ok(Map::new()),
        Some(Value::Object(output)) => {
            if let Some(unknown) = output.keys().find(|k| !OUTPUT_FIELDS.contains(&k.as_str())) {
                let expected = format!("expected one of: {}", OUTPUT_FIELDS.join(", "));
                let hint = match did_you_mean(unknown, &OUTPUT_FIELDS) {
                    Some(close) => format!("did you mean '{close}'? {expected}"),
                    None => expected,
                };
                return Err(Report::new(
                    ErrorCode::InvalidOption,
                    format!("unknown field 'output.{unknown}'"),
                )
                .hint(hint));
            }
            Ok(output.clone())
        }
        Some(_) => Err(Report::new(
            ErrorCode::InvalidOption,
            "'output' must be an object",
        )),
    }
}

/// `invalid_option` for a field the tool doesn't have.
fn unknown_field(unknown: &str, fields: &[&str]) -> Report {
    let expected = format!("expected one of: {}", fields.join(", "));
    let hint = match did_you_mean(unknown, fields) {
        Some(close) => format!("did you mean '{close}'? {expected}"),
        None => expected,
    };
    Report::new(
        ErrorCode::InvalidOption,
        format!("unknown field '{unknown}'"),
    )
    .hint(hint)
}

/// A selected entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    /// The path in the file system: its working directory and its path in it.
    pub path: PathBuf,
    /// The path as `find` shows it.
    pub shown: PathBuf,
    /// The path of a member of an archive.
    pub member: Option<String>,
    pub is_dir: bool,
    /// Whether it is an entry of a result, whose stamp is compared before acting.
    pub from_result: bool,
    /// Of an entry of a result: its stamp when the result was kept.
    pub stamp: Option<Stamp>,
}

/// The `count_mismatch` of `expect`, before anything is changed.
pub(crate) fn check_expect(count: usize, expect: Option<usize>) -> Result<(), Report> {
    match expect {
        Some(expected) if expected != count => Err(Report::new(
            ErrorCode::CountMismatch,
            format!("the selection has {count} files, expected {expected}"),
        )
        .hint("nothing was changed; check the selection with dry_run: true")),
        _ => Ok(()),
    }
}

/// The entries to act on: in path order, without the entries inside a selected directory.
pub(crate) fn select(mut entries: Vec<Entry>) -> Vec<Entry> {
    entries.sort_by(|a, b| (&a.shown, &a.member).cmp(&(&b.shown, &b.member)));
    entries.dedup_by(|a, b| a.path == b.path && a.member == b.member);
    let dirs: Vec<PathBuf> = entries
        .iter()
        .filter(|e| e.is_dir && e.member.is_none())
        .map(|e| e.path.clone())
        .collect();
    entries
        .into_iter()
        .filter(|e| {
            !dirs
                .iter()
                .any(|dir| e.path.starts_with(dir) && (e.path != *dir || e.member.is_some()))
        })
        .collect()
}

/// An entry acted on, or which failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Acted {
    pub shown: PathBuf,
    pub is_dir: bool,
    /// The kind of the error of a failed entry.
    pub error: Option<String>,
}

/// What a write tool did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub acted: Vec<Acted>,
    pub skipped: usize,
    pub warnings: Warnings,
    pub dry_run: bool,
}

/// How a write tool acts on the entries.
pub(crate) struct Act<'a> {
    /// Whether a canonical path is inside the allowed roots.
    pub allowed: &'a dyn Fn(&Path) -> bool,
    /// Whether the call must stop: its deadline, its cancellation.
    pub stop: &'a dyn Fn() -> bool,
    /// The operation on a checked path (its directory canonical).
    pub operation: &'a dyn Fn(&Path) -> Result<(), TrashError>,
    pub dry_run: bool,
}

/// Acts on the entries in their order, skipping the ones which can't be changed, until the call must stop.
pub(crate) fn act(entries: &[Entry], how: &Act<'_>) -> Outcome {
    let mut outcome = Outcome {
        dry_run: how.dry_run,
        ..Outcome::default()
    };
    for entry in entries {
        if (how.stop)() {
            outcome.warnings.add_global(WarningCode::Timeout);
            break;
        }
        let skip = |outcome: &mut Outcome, code| {
            outcome
                .warnings
                .add(code, entry.shown.clone(), entry.is_dir);
            outcome.skipped += 1;
        };
        if let Some(member) = &entry.member {
            outcome.warnings.add_member(
                WarningCode::ArchiveMember,
                entry.shown.clone(),
                member.clone(),
            );
            outcome.skipped += 1;
            continue;
        }
        let checked = checked_path(&entry.path);
        if checked.as_deref().is_some_and(|p| !(how.allowed)(p)) {
            skip(&mut outcome, WarningCode::PermissionDenied);
            continue;
        }
        if entry.from_result && (entry.stamp.is_none() || Stamp::of(&entry.path) != entry.stamp) {
            skip(&mut outcome, WarningCode::FileChanged);
            continue;
        }
        let done = match checked {
            None => Err(TrashError::Failed(std::io::ErrorKind::NotFound.to_string())),
            Some(_) if how.dry_run => Ok(()),
            Some(path) => (how.operation)(&path),
        };
        let error = match done {
            Ok(()) => None,
            Err(TrashError::Unavailable) => {
                skip(&mut outcome, WarningCode::TrashUnavailable);
                continue;
            }
            Err(TrashError::Failed(error)) => Some(error),
        };
        outcome.acted.push(Acted {
            shown: entry.shown.clone(),
            is_dir: entry.is_dir,
            error,
        });
    }
    outcome
}

/// The path of an existing entry with its directory canonical and its name not resolved: a symlink is the link
/// itself; `None` when it doesn't exist.
fn checked_path(path: &Path) -> Option<PathBuf> {
    std::fs::symlink_metadata(path).ok()?;
    let name = path.file_name()?;
    let parent = crate::paths::canonicalize(path.parent()?).ok()?;
    Some(parent.join(name))
}

impl Outcome {
    /// The entries acted on without an error.
    fn done(&self) -> usize {
        self.acted.iter().filter(|a| a.error.is_none()).count()
    }

    /// The entries which failed.
    fn failed(&self) -> usize {
        self.acted.len() - self.done()
    }

    /// `isError`: something failed and nothing was done.
    pub(crate) fn is_error(&self) -> bool {
        self.failed() > 0 && self.done() == 0
    }

    /// The text format: the header with the count named `verb`, the entries, the warnings.
    pub(crate) fn to_text(&self, verb: &str, group_dirs: bool) -> String {
        let mut out = format!("{verb}: {}", self.done());
        if self.skipped > 0 {
            let _ = write!(out, "  skipped: {}", self.skipped);
        }
        if self.failed() > 0 {
            let _ = write!(out, "  failed: {}", self.failed());
        }
        if self.dry_run {
            out.push_str("  dry run");
        }
        out.push('\n');
        if !self.acted.is_empty() {
            out.push('\n');
            let mut current: Option<String> = None;
            for acted in &self.acted {
                if group_dirs {
                    let (dir, name) = split(&acted.shown);
                    if current.as_deref() != Some(dir.as_str()) {
                        out.push_str(&dir);
                        out.push('\n');
                        current = Some(dir);
                    }
                    out.push_str(&name);
                    if acted.is_dir {
                        out.push(std::path::MAIN_SEPARATOR);
                    }
                } else {
                    out.push_str(&display_path(&acted.shown, acted.is_dir));
                }
                if let Some(error) = &acted.error {
                    let _ = write!(out, "  failed: {error}");
                }
                out.push('\n');
            }
        }
        if !self.warnings.is_empty() {
            out.push('\n');
            out.push_str(&self.warnings.to_text());
        }
        out
    }

    /// The JSON format, compact; empty and default values omitted.
    pub(crate) fn to_json(&self, verb: &str) -> String {
        let mut doc = Map::new();
        doc.insert("version".into(), json!(crate::SPEC_VERSION));
        doc.insert(verb.into(), json!(self.done()));
        if self.skipped > 0 {
            doc.insert("skipped".into(), json!(self.skipped));
        }
        if self.failed() > 0 {
            doc.insert("failed".into(), json!(self.failed()));
        }
        if self.dry_run {
            doc.insert("dry_run".into(), json!(true));
        }
        if !self.acted.is_empty() {
            let files: Vec<Value> = self
                .acted
                .iter()
                .map(|acted| {
                    let mut file = Map::new();
                    file.insert(
                        "path".into(),
                        json!(display_path(&acted.shown, acted.is_dir)),
                    );
                    if let Some(error) = &acted.error {
                        file.insert("error".into(), json!(error));
                    }
                    Value::Object(file)
                })
                .collect();
            doc.insert("files".into(), Value::Array(files));
        }
        if !self.warnings.is_empty() {
            let warnings: Vec<Value> = self
                .warnings
                .groups()
                .into_iter()
                .map(|(code, count, paths)| match count {
                    Some(count) => json!({ "code": code, "count": count, "paths": paths }),
                    None => json!({ "code": code }),
                })
                .collect();
            doc.insert("warnings".into(), Value::Array(warnings));
        }
        Value::Object(doc).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, is_dir: bool) -> Entry {
        Entry {
            path: PathBuf::from("/w").join(path),
            shown: PathBuf::from(path),
            member: None,
            is_dir,
            from_result: false,
            stamp: None,
        }
    }

    fn text(arguments: &Value) -> String {
        parse(arguments, &DELETE_FIELDS).unwrap_err().to_text()
    }

    #[test]
    fn selections_which_are_rejected() {
        assert_eq!(
            text(&json!({"result": "r1", "query": ""})),
            "error[invalid_option]: 'result' and 'query' exclude each other\n"
        );
        assert!(
            text(&json!({}))
                .starts_with("error[invalid_option]: 'result' or 'query' is required\n")
        );
        assert_eq!(
            text(&json!({"result": "r1", "traversal": {}})),
            "error[invalid_option]: 'traversal' works only with 'query'\n"
        );
        assert_eq!(
            text(&json!({"query": "", "expect": -1})),
            "error[invalid_option]: 'expect' must be a whole number\n"
        );
        assert_eq!(
            text(&json!({"query": "", "dry_run": 1})),
            "error[invalid_option]: 'dry_run' must be a boolean\n"
        );
        assert_eq!(
            text(&json!({"query": "", "output": {"detail": "files"}})),
            "error[invalid_option]: unknown field 'output.detail'\n  hint: expected one of: format, group_dirs\n"
        );
        assert_eq!(
            text(&json!({"query": "", "timeout": "x"})),
            "error[invalid_option]: 'timeout' must be a duration like \"10s\" (units ms, s, m)\n"
        );
        let checked = parse(&json!({"result": "r3", "dry_run": true}), &DELETE_FIELDS).unwrap();
        assert_eq!(checked.selection, Selection::Result(3));
        assert!(checked.dry_run && !checked.permanent);
    }

    #[test]
    fn entries_inside_a_selected_directory_go_with_it() {
        let selected = select(vec![
            entry("build/out.txt", false),
            entry("build-x", false),
            entry("build", true),
            entry("a", false),
        ]);
        let shown: Vec<&Path> = selected.iter().map(|e| e.shown.as_path()).collect();
        assert_eq!(
            shown,
            [Path::new("a"), Path::new("build"), Path::new("build-x")]
        );
        assert_eq!(check_expect(4, Some(4)), Ok(()));
        assert_eq!(
            check_expect(4, Some(3)).unwrap_err().to_text(),
            "error[count_mismatch]: the selection has 4 files, expected 3\n  hint: nothing was changed; check the selection with dry_run: true\n"
        );
    }

    #[test]
    fn failed_entries_and_the_formats() {
        let mut outcome = Outcome {
            acted: vec![
                Acted {
                    shown: PathBuf::from("src/a.rs"),
                    is_dir: false,
                    error: None,
                },
                Acted {
                    shown: PathBuf::from("src/b.rs"),
                    is_dir: false,
                    error: Some("permission denied".into()),
                },
            ],
            ..Outcome::default()
        };
        outcome
            .warnings
            .add(WarningCode::FileChanged, PathBuf::from("c.rs"), false);
        outcome.skipped = 1;
        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(
            outcome.to_text("deleted", true),
            format!(
                "deleted: 1  skipped: 1  failed: 1\n\nsrc{sep}\na.rs\nb.rs  failed: permission denied\n\nwarning: file_changed (1): c.rs\n"
            )
        );
        assert!(!outcome.is_error());
        let json: Value = serde_json::from_str(&outcome.to_json("deleted")).unwrap();
        assert_eq!(json["failed"], json!(1));
        assert_eq!(json["files"][1]["error"], json!("permission denied"));
        outcome.acted.remove(0);
        assert!(outcome.is_error());
        let empty = Outcome {
            dry_run: true,
            ..Outcome::default()
        };
        assert_eq!(empty.to_text("deleted", true), "deleted: 0  dry run\n");
        assert!(!empty.is_error());
    }
}
