//! The write tools: their arguments, the selection, acting on it and the response (TOOLS.md#write-tools).

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use crate::cache::Stamp;
use crate::error::{ErrorCode, Report};
use crate::output::{dir_name, display_path, split};
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

/// The fields of `rename` and of `copy`, in the order of their input schemas.
pub(crate) const RENAME_FIELDS: [&str; 14] = [
    "version",
    "result",
    "query",
    "dirs",
    "traversal",
    "kinds",
    "languages",
    "to",
    "pattern",
    "on_conflict",
    "expect",
    "dry_run",
    "timeout",
    "output",
];

/// The fields which select files together with `query`.
const QUERY_FIELDS: [&str; 4] = ["dirs", "traversal", "kinds", "languages"];

/// The fields of `output` of `delete`, `rename` and `copy`.
pub(crate) const OUTPUT_FIELDS: [&str; 2] = ["format", "group_dirs"];

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

/// Checks the arguments of a write tool with the fields `fields` and the fields `output_fields` of its `output`.
pub(crate) fn parse(
    arguments: &Value,
    fields: &[&str],
    output_fields: &[&str],
) -> Result<WriteRequest, Report> {
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
    let mut output = output_object(object.get("output"), output_fields)?;
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

/// The `output` object of a write tool with the fields `fields`.
fn output_object(output: Option<&Value>, fields: &[&str]) -> Result<Map<String, Value>, Report> {
    match output {
        None => Ok(Map::new()),
        Some(Value::Object(output)) => {
            if let Some(unknown) = output.keys().find(|k| !fields.contains(&k.as_str())) {
                let expected = format!("expected one of: {}", fields.join(", "));
                let hint = match did_you_mean(unknown, fields) {
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
    /// The working directory.
    pub dir: PathBuf,
    /// The path as `find` shows it: the prefix of the working directory and the path in it.
    pub shown: PathBuf,
    /// The prefix of the working directory in `shown`.
    pub prefix: PathBuf,
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

impl Entry {
    /// The path relative to the working directory.
    pub(crate) fn rel(&self) -> &Path {
        self.shown.strip_prefix(&self.prefix).unwrap_or(&self.shown)
    }
}

/// An entry acted on, or which failed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Acted {
    pub shown: PathBuf,
    pub is_dir: bool,
    /// The kind of the error of a failed entry.
    pub error: Option<String>,
    /// `rename`: the target as it is shown.
    pub to: Option<PathBuf>,
    /// `rename`: an existing target was replaced.
    pub replaced: bool,
    /// `rename`: the target is the old path; `replace`: the file has no match or its text stays the same.
    pub unchanged: bool,
    /// `replace`: the number of replaced matches of a changed file.
    pub matches: Option<usize>,
    /// `replace`: the changed lines.
    pub changes: Vec<crate::replace::Change>,
}

impl Acted {
    /// A done or failed entry without a target.
    pub(crate) fn of(entry: &Entry, error: Option<String>) -> Self {
        Self {
            shown: entry.shown.clone(),
            is_dir: entry.is_dir,
            error,
            ..Self::default()
        }
    }
}

/// What a write tool did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub acted: Vec<Acted>,
    pub skipped: usize,
    pub warnings: Warnings,
    pub dry_run: bool,
    pub permanent: bool,
    /// The number of entries of the selection.
    pub selected: usize,
    /// `replace`: the most changed lines shown; `None` for the other tools.
    pub max_lines: Option<usize>,
    /// The only working directory, written once in the `in:` line (TOOLS.md#one-working-directory-of-a-write-tool).
    pub in_dir: Option<PathBuf>,
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
    pub permanent: bool,
}

/// Whether a canonical path may be changed: inside an allowed root and not an allowed root itself.
pub(crate) fn may_change(path: &Path, roots: &[PathBuf]) -> bool {
    roots.iter().any(|root| path.starts_with(root)) && !roots.iter().any(|root| path == root)
}

/// The checks of an entry before it is changed: its checked path (`None` when it no longer exists), or the warning
/// it is skipped with.
pub(crate) fn check(
    entry: &Entry,
    allowed: &dyn Fn(&Path) -> bool,
) -> Result<Option<PathBuf>, WarningCode> {
    if entry.member.is_some() {
        return Err(WarningCode::ArchiveMember);
    }
    let checked = checked_path(&entry.path);
    if checked.as_deref().is_some_and(|p| !allowed(p)) {
        return Err(WarningCode::PermissionDenied);
    }
    if entry.from_result && (entry.stamp.is_none() || Stamp::of(&entry.path) != entry.stamp) {
        return Err(WarningCode::FileChanged);
    }
    Ok(checked)
}

/// The error of an entry which no longer exists.
pub(crate) fn not_found() -> String {
    std::io::ErrorKind::NotFound.to_string()
}

impl Outcome {
    /// Skips an entry with the warning `code`.
    pub(crate) fn skip(&mut self, entry: &Entry, code: WarningCode) {
        match &entry.member {
            Some(member) => {
                self.warnings
                    .add_member(code, entry.shown.clone(), member.clone());
            }
            None => self.warnings.add(code, entry.shown.clone(), entry.is_dir),
        }
        self.skipped += 1;
    }
}

/// Acts on the entries in their order, skipping the ones which can't be changed, until the call must stop.
pub(crate) fn act(entries: &[Entry], how: &Act<'_>) -> Outcome {
    let mut outcome = Outcome {
        dry_run: how.dry_run,
        permanent: how.permanent,
        selected: entries.len(),
        ..Outcome::default()
    };
    for entry in entries {
        if (how.stop)() {
            outcome.warnings.add_global(WarningCode::Timeout);
            break;
        }
        let checked = match check(entry, how.allowed) {
            Ok(checked) => checked,
            Err(code) => {
                outcome.skip(entry, code);
                continue;
            }
        };
        let done = match checked {
            None => Err(TrashError::Failed(not_found())),
            Some(_) if how.dry_run => Ok(()),
            Some(path) => (how.operation)(&path),
        };
        let error = match done {
            Ok(()) => None,
            Err(TrashError::Unavailable) => {
                outcome.skip(entry, WarningCode::TrashUnavailable);
                continue;
            }
            Err(TrashError::Failed(error)) => Some(error),
        };
        outcome.acted.push(Acted::of(entry, error));
    }
    outcome
}

/// Deletes the entry at `path` permanently: a directory with everything in it, a symlink itself.
pub(crate) fn remove(path: &Path) -> Result<(), TrashError> {
    let meta = std::fs::symlink_metadata(path).map_err(|e| crate::recycle::failed(&e))?;
    let removed = if meta.file_type().is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        // a symlink to a directory is a directory on Windows
        std::fs::remove_file(path).or_else(|e| {
            if cfg!(windows) && meta.file_type().is_symlink() {
                std::fs::remove_dir(path)
            } else {
                Err(e)
            }
        })
    };
    removed.map_err(|e| crate::recycle::failed(&e))
}

/// The path of an existing entry with its directory canonical and its name not resolved: a symlink is the link
/// itself; `None` when it doesn't exist.
pub(crate) fn checked_path(path: &Path) -> Option<PathBuf> {
    std::fs::symlink_metadata(path).ok()?;
    let name = path.file_name()?;
    let parent = crate::paths::canonicalize(path.parent()?).ok()?;
    Some(parent.join(name))
}

impl Outcome {
    /// The entries acted on without an error, the unchanged ones too.
    fn done(&self) -> usize {
        self.acted.iter().filter(|a| a.error.is_none()).count()
    }

    /// The unchanged entries of `rename`.
    fn unchanged(&self) -> usize {
        self.acted.iter().filter(|a| a.unchanged).count()
    }

    /// `replace`: the matches replaced in the changed files.
    fn matches(&self) -> usize {
        self.acted.iter().filter_map(|a| a.matches).sum()
    }

    /// The entries which failed.
    fn failed(&self) -> usize {
        self.acted.len() - self.done()
    }

    /// `isError`: the selection has entries and none of them was done.
    pub(crate) fn is_error(&self) -> bool {
        self.selected > 0 && self.done() == 0
    }

    /// The paths of the entries, the targets and the warnings relative to the only working directory `dir`, a target
    /// outside of it with `..`; `line`: the directory is written once in the `in:` line.
    pub(crate) fn relative_to(&mut self, dir: &Path, line: bool) {
        for acted in &mut self.acted {
            acted.shown = relative(&acted.shown, dir);
            if let Some(to) = &acted.to {
                acted.to = Some(relative(to, dir));
            }
        }
        self.warnings.strip_prefix(dir);
        self.in_dir = line.then(|| dir.to_path_buf());
    }

    /// The working directory of the `in:` line: when the response has entries or warnings about entries.
    fn shown_dir(&self) -> Option<&Path> {
        self.in_dir
            .as_deref()
            .filter(|_| !self.acted.is_empty() || self.warnings.has_paths())
    }

    /// The text format of the spec version `spec`: the header with the count named `verb` (`deleted`), the entries,
    /// the warnings. In spec 2 the header of a dry run starts with `dry run  would delete:` (TOOLS.md#dry-run).
    pub(crate) fn to_text(&self, verb: &str, group_dirs: bool, spec: u32) -> String {
        let count = self.done() - self.unchanged();
        let ahead = self.dry_run && spec >= 2;
        let mut out = if ahead {
            format!("dry run  would {}: {count}", infinitive(verb))
        } else {
            format!("{verb}: {count}")
        };
        if self.max_lines.is_some() {
            let _ = write!(out, "  matches: {}", self.matches());
        }
        if self.unchanged() > 0 {
            let _ = write!(out, "  unchanged: {}", self.unchanged());
        }
        if self.skipped > 0 {
            let _ = write!(out, "  skipped: {}", self.skipped);
        }
        if self.failed() > 0 {
            let _ = write!(out, "  failed: {}", self.failed());
        }
        if self.permanent {
            out.push_str("  permanent");
        }
        if self.dry_run && !ahead {
            out.push_str("  dry run");
        }
        out.push('\n');
        if let Some(dir) = self.shown_dir() {
            let _ = writeln!(out, "in: {}", dir_name(dir));
        }
        if !self.acted.is_empty() {
            out.push('\n');
            let mut lines = self.max_lines.unwrap_or(usize::MAX);
            let mut more = 0;
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
                if let Some(to) = &acted.to {
                    out.push_str(" -> ");
                    out.push_str(&display_path(to, acted.is_dir));
                }
                if acted.replaced {
                    out.push_str("  replaced");
                }
                if acted.unchanged {
                    out.push_str("  unchanged");
                }
                if let Some(matches) = acted.matches {
                    let _ = write!(out, "  matches: {matches}");
                }
                if let Some(error) = &acted.error {
                    let _ = write!(out, "  failed: {error}");
                }
                out.push('\n');
                for change in &acted.changes {
                    let old = change
                        .old
                        .iter()
                        .enumerate()
                        .map(|(i, l)| ('-', change.line + i, l));
                    let new = change
                        .new
                        .iter()
                        .enumerate()
                        .map(|(i, l)| ('+', change.new_line + i, l));
                    for (sign, n, line) in old.chain(new) {
                        if lines == 0 {
                            more += 1;
                        } else {
                            lines -= 1;
                            let _ = writeln!(out, "{sign}{n}:{line}");
                        }
                    }
                }
            }
            if more > 0 {
                let _ = writeln!(out, "… {more} more lines");
            }
        }
        if !self.warnings.is_empty() {
            out.push('\n');
            out.push_str(&self.warnings.to_text());
        }
        out
    }

    /// The JSON format of the spec version `version`, compact; empty and default values omitted.
    pub(crate) fn to_json(&self, verb: &str, version: u32) -> String {
        let mut doc = Map::new();
        doc.insert("version".into(), json!(version));
        doc.insert(verb.into(), json!(self.done() - self.unchanged()));
        if self.max_lines.is_some() {
            doc.insert("matches".into(), json!(self.matches()));
        }
        if self.unchanged() > 0 {
            doc.insert("unchanged".into(), json!(self.unchanged()));
        }
        if self.skipped > 0 {
            doc.insert("skipped".into(), json!(self.skipped));
        }
        if self.failed() > 0 {
            doc.insert("failed".into(), json!(self.failed()));
        }
        if self.permanent {
            doc.insert("permanent".into(), json!(true));
        }
        if self.dry_run {
            doc.insert("dry_run".into(), json!(true));
        }
        if let Some(dir) = self.shown_dir() {
            doc.insert("dirs".into(), json!([dir_name(dir)]));
        }
        let mut lines = self.max_lines.unwrap_or(usize::MAX);
        let mut more = 0;
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
                    if let Some(to) = &acted.to {
                        file.insert("to".into(), json!(display_path(to, acted.is_dir)));
                    }
                    if acted.replaced {
                        file.insert("replaced".into(), json!(true));
                    }
                    if acted.unchanged {
                        file.insert("unchanged".into(), json!(true));
                    }
                    if let Some(matches) = acted.matches {
                        file.insert("matches".into(), json!(matches));
                    }
                    let changes: Vec<Value> = acted
                        .changes
                        .iter()
                        .filter_map(|change| {
                            let mut take = |all: &[String]| -> Vec<String> {
                                let shown = all.len().min(lines);
                                lines -= shown;
                                more += all.len() - shown;
                                all[..shown].to_vec()
                            };
                            let old = take(&change.old);
                            let new = take(&change.new);
                            (!old.is_empty() || !new.is_empty()).then(|| {
                                json!({ "line": change.line, "old": old, "new_line": change.new_line, "new": new })
                            })
                        })
                        .collect();
                    if !changes.is_empty() {
                        file.insert("changes".into(), Value::Array(changes));
                    }
                    if let Some(error) = &acted.error {
                        file.insert("error".into(), json!(error));
                    }
                    Value::Object(file)
                })
                .collect();
            doc.insert("files".into(), Value::Array(files));
        }
        if more > 0 {
            doc.insert("more_lines".into(), json!(more));
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

/// `path` relative to `base`, both without the file system: stripped of it, or with `..` for every component of
/// `base` which `path` doesn't share.
pub(crate) fn relative(path: &Path, base: &Path) -> PathBuf {
    if let Ok(rel) = path.strip_prefix(base) {
        return rel.to_path_buf();
    }
    let path = crate::paths::normalize(path);
    let base = crate::paths::normalize(base);
    let common = path
        .components()
        .zip(base.components())
        .take_while(|(a, b)| a == b)
        .count();
    let mut out: PathBuf = base.components().skip(common).map(|_| "..").collect();
    out.extend(path.components().skip(common));
    out
}

/// The verb of a count of a write tool: `deleted` → `delete`, `copied` → `copy`.
fn infinitive(count: &str) -> String {
    match count.strip_suffix("ied") {
        Some(stem) => format!("{stem}y"),
        None => count.strip_suffix('d').unwrap_or(count).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, is_dir: bool) -> Entry {
        Entry {
            path: PathBuf::from("/w").join(path),
            dir: PathBuf::from("/w"),
            shown: PathBuf::from(path),
            prefix: PathBuf::new(),
            member: None,
            is_dir,
            from_result: false,
            stamp: None,
        }
    }

    fn text(arguments: &Value) -> String {
        parse(arguments, &DELETE_FIELDS, &OUTPUT_FIELDS)
            .unwrap_err()
            .to_text()
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
        let checked = parse(
            &json!({"result": "r3", "dry_run": true}),
            &DELETE_FIELDS,
            &OUTPUT_FIELDS,
        )
        .unwrap();
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
                    ..Acted::default()
                },
                Acted {
                    shown: PathBuf::from("src/b.rs"),
                    error: Some("permission denied".into()),
                    ..Acted::default()
                },
            ],
            selected: 3,
            ..Outcome::default()
        };
        outcome
            .warnings
            .add(WarningCode::FileChanged, PathBuf::from("c.rs"), false);
        outcome.skipped = 1;
        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(
            outcome.to_text("deleted", true, 1),
            format!(
                "deleted: 1  skipped: 1  failed: 1\n\nsrc{sep}\na.rs\nb.rs  failed: permission denied\n\nwarning: file_changed (1): c.rs\n"
            )
        );
        assert!(!outcome.is_error());
        let json: Value = serde_json::from_str(&outcome.to_json("deleted", 1)).unwrap();
        assert_eq!(json["failed"], json!(1));
        assert_eq!(json["files"][1]["error"], json!("permission denied"));
        outcome.acted.remove(0);
        assert!(outcome.is_error());
        let empty = Outcome {
            dry_run: true,
            ..Outcome::default()
        };
        assert_eq!(empty.to_text("deleted", true, 1), "deleted: 0  dry run\n");
        assert_eq!(
            empty.to_text("deleted", true, 2),
            "dry run  would delete: 0\n"
        );
        for (count, verb) in [
            ("renamed", "rename"),
            ("copied", "copy"),
            ("replaced", "replace"),
        ] {
            assert_eq!(
                empty.to_text(count, true, 2),
                format!("dry run  would {verb}: 0\n")
            );
        }
        assert!(!empty.is_error());
    }

    #[test]
    fn nothing_done_of_a_selection_is_an_error() {
        let mut skipped = Outcome {
            skipped: 2,
            selected: 2,
            permanent: true,
            ..Outcome::default()
        };
        skipped
            .warnings
            .add(WarningCode::TrashUnavailable, PathBuf::from("a.log"), false);
        assert!(skipped.is_error());
        assert!(
            skipped
                .to_text("deleted", true, 1)
                .starts_with("deleted: 0  skipped: 2  permanent\n")
        );
        let json: Value = serde_json::from_str(&skipped.to_json("deleted", 1)).unwrap();
        assert_eq!(json["permanent"], json!(true));
        let done = Outcome {
            acted: vec![Acted {
                shown: PathBuf::from("a.log"),
                ..Acted::default()
            }],
            skipped: 1,
            selected: 2,
            ..Outcome::default()
        };
        assert!(!done.is_error());
    }

    #[test]
    fn targets_replaced_and_unchanged_entries() {
        let outcome = Outcome {
            acted: vec![
                Acted {
                    shown: PathBuf::from("src/a.ts"),
                    to: Some(PathBuf::from("src/a.tsx")),
                    ..Acted::default()
                },
                Acted {
                    shown: PathBuf::from("src/b.ts"),
                    to: Some(PathBuf::from("src/b.tsx")),
                    replaced: true,
                    ..Acted::default()
                },
                Acted {
                    shown: PathBuf::from("src/c.tsx"),
                    unchanged: true,
                    ..Acted::default()
                },
            ],
            selected: 3,
            ..Outcome::default()
        };
        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(
            outcome.to_text("renamed", true, 1),
            format!(
                "renamed: 2  unchanged: 1\n\nsrc{sep}\na.ts -> src{sep}a.tsx\nb.ts -> src{sep}b.tsx  replaced\nc.tsx  unchanged\n"
            )
        );
        let json: Value = serde_json::from_str(&outcome.to_json("renamed", 1)).unwrap();
        assert_eq!(json["renamed"], json!(2));
        assert_eq!(json["unchanged"], json!(1));
        assert_eq!(json["files"][1]["replaced"], json!(true));
        assert_eq!(
            json["files"][2],
            json!({"path": format!("src{sep}c.tsx"), "unchanged": true})
        );
        let only_unchanged = Outcome {
            acted: vec![outcome.acted[2].clone()],
            skipped: 1,
            selected: 2,
            ..Outcome::default()
        };
        assert!(!only_unchanged.is_error());
    }

    #[test]
    fn changed_lines_of_replace_cut_by_max_lines() {
        let change = crate::replace::Change {
            line: 2,
            old: vec!["a".into(), "b".into()],
            new_line: 2,
            new: vec!["A".into()],
            ..crate::replace::Change::default()
        };
        let outcome = Outcome {
            acted: vec![
                Acted {
                    shown: PathBuf::from("x.txt"),
                    matches: Some(2),
                    changes: vec![change.clone()],
                    ..Acted::default()
                },
                Acted {
                    shown: PathBuf::from("y.txt"),
                    matches: Some(1),
                    changes: vec![change],
                    ..Acted::default()
                },
                Acted {
                    shown: PathBuf::from("z.txt"),
                    unchanged: true,
                    ..Acted::default()
                },
            ],
            selected: 3,
            max_lines: Some(4),
            ..Outcome::default()
        };
        assert_eq!(
            outcome.to_text("replaced", false, 1),
            "replaced: 2  matches: 3  unchanged: 1\n\nx.txt  matches: 2\n-2:a\n-3:b\n+2:A\ny.txt  matches: 1\n-2:a\nz.txt  unchanged\n… 2 more lines\n"
        );
        let json: Value = serde_json::from_str(&outcome.to_json("replaced", 1)).unwrap();
        assert_eq!(json["matches"], json!(3));
        assert_eq!(
            json["files"][1],
            json!({"path": "y.txt", "matches": 1, "changes": [{"line": 2, "old": ["a"], "new_line": 2, "new": []}]})
        );
        assert_eq!(json["more_lines"], json!(2));
        assert!(json["files"][2].get("changes").is_none());
    }

    #[test]
    fn long_changed_lines_cut_in_text_only() {
        let old = format!("{}old{}", "a".repeat(30), "b".repeat(30));
        let new = format!("{}new text{}", "a".repeat(30), "b".repeat(30));
        let change = crate::replace::Change {
            line: 1,
            old: vec![old.clone(), "short".into()],
            old_parts: vec![vec![30..33], vec![0..5]],
            new_line: 1,
            new: vec![new.clone()],
            new_parts: vec![vec![30..38]],
        };
        let mut outcome = Outcome {
            acted: vec![Acted {
                shown: PathBuf::from("x.txt"),
                matches: Some(1),
                changes: vec![change],
                ..Acted::default()
            }],
            selected: 1,
            max_lines: Some(100),
            max_line_length: Some(23),
            ..Outcome::default()
        };
        // (23 − 3) / 2 = 10 and (23 − 8) / 2 = 7 characters on both sides
        assert_eq!(
            outcome.to_text("replaced", false, 2),
            "replaced: 1  matches: 1\n\nx.txt  matches: 1\n-1:…aaaaaaaaaaoldbbbbbbbbbb…\n-2:short\n+1:…aaaaaaanew textbbbbbbb…\n"
        );
        let json: Value = serde_json::from_str(&outcome.to_json("replaced", 2)).unwrap();
        assert_eq!(json["files"][0]["changes"][0]["old"][0], json!(old));
        assert_eq!(json["files"][0]["changes"][0]["new"][0], json!(new));
        outcome.max_line_length = None;
        assert!(outcome.to_text("replaced", false, 2).contains(&format!("+1:{new}\n")));
    }

    #[test]
    fn one_working_directory_is_written_once() {
        let dir = PathBuf::from("w/proj");
        let outcome = || {
            let mut outcome = Outcome {
                acted: vec![
                    Acted {
                        shown: dir.join("src/a.rs"),
                        to: Some(PathBuf::from("w/other/a.rs")),
                        ..Acted::default()
                    },
                    Acted {
                        shown: dir.join("src/b.rs"),
                        to: Some(dir.join("lib/b.rs")),
                        ..Acted::default()
                    },
                ],
                skipped: 1,
                selected: 3,
                ..Outcome::default()
            };
            outcome
                .warnings
                .add(WarningCode::FileChanged, dir.join("src/c.rs"), false);
            outcome
        };
        let sep = std::path::MAIN_SEPARATOR;
        let mut once = outcome();
        once.relative_to(&dir, true);
        assert_eq!(
            once.to_text("renamed", true, 2),
            format!(
                "renamed: 2  skipped: 1\nin: w{sep}proj{sep}\n\nsrc{sep}\na.rs -> ..{sep}other{sep}a.rs\nb.rs -> lib{sep}b.rs\n\nwarning: file_changed (1): src{sep}c.rs\n"
            )
        );
        let json: Value = serde_json::from_str(&once.to_json("renamed", 2)).unwrap();
        assert_eq!(json["dirs"], json!([format!("w{sep}proj{sep}")]));
        assert_eq!(json["files"][0]["path"], json!(format!("src{sep}a.rs")));
        assert_eq!(
            json["files"][0]["to"],
            json!(format!("..{sep}other{sep}a.rs"))
        );
        let mut without_line = outcome();
        without_line.relative_to(&dir, false);
        assert!(
            without_line
                .to_text("renamed", true, 2)
                .starts_with(&format!("renamed: 2  skipped: 1\n\nsrc{sep}\n"))
        );
        assert!(
            serde_json::from_str::<Value>(&without_line.to_json("renamed", 2)).unwrap()["dirs"]
                .is_null()
        );
        let mut empty = Outcome::default();
        empty.relative_to(&dir, true);
        assert_eq!(empty.to_text("deleted", true, 2), "deleted: 0\n");
        assert!(
            serde_json::from_str::<Value>(&empty.to_json("deleted", 2)).unwrap()["dirs"].is_null()
        );
    }

    #[test]
    fn relative_targets() {
        let base = Path::new("w/proj");
        assert_eq!(
            relative(Path::new("w/proj/src/a.rs"), base),
            Path::new("src/a.rs")
        );
        assert_eq!(
            relative(Path::new("w/other/a.rs"), base),
            Path::new("../other/a.rs")
        );
        assert_eq!(
            relative(Path::new("x/a.rs"), base),
            Path::new("../../x/a.rs")
        );
        assert_eq!(
            relative(Path::new("w/proj/a.rs"), Path::new("w/x/../proj")),
            Path::new("a.rs")
        );
    }

    #[test]
    fn an_allowed_root_itself_is_never_changed() {
        let roots = [PathBuf::from("/p"), PathBuf::from("/p/q")];
        assert!(may_change(Path::new("/p/a.txt"), &roots));
        assert!(may_change(Path::new("/p/q/b"), &roots));
        assert!(!may_change(Path::new("/p/q"), &roots));
        assert!(!may_change(Path::new("/p"), &roots));
        assert!(!may_change(Path::new("/other/a.txt"), &roots));
    }
}
