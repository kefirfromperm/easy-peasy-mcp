//! The `rename` and `copy` tools: the target of every entry, the conflicts of the plan, the renames or the copies
//! (TOOLS.md#rename, TOOLS.md#copy).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{ErrorCode, QueryError, Report};
use crate::output::display_path;
use crate::recycle::TrashError;
use crate::template::{self, Groups, PLACEHOLDERS, Template};
use crate::warning::WarningCode;
use crate::write::{self, Acted, Entry, Outcome};

/// What is done with a target which conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OnConflict {
    Fail,
    Skip,
    Overwrite,
}

/// Whether the entries are moved or copied to their targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    Rename,
    Copy,
}

impl Mode {
    /// The count of the response: `renamed`, `copied`.
    pub(crate) fn count(self) -> &'static str {
        match self {
            Self::Rename => "renamed",
            Self::Copy => "copied",
        }
    }
}

/// The fields of `rename` and `copy` besides the shared ones of the write tools.
#[derive(Debug, Clone)]
pub(crate) struct RenameRequest {
    pub to: Template,
    /// `pattern`, anchored at both ends.
    pub pattern: Option<regex::Regex>,
    pub on_conflict: OnConflict,
}

/// At most this many conflicts in the text of `rename_conflict`.
const MAX_CONFLICTS: usize = 10;

/// Checks `to`, `pattern` and `on_conflict` of the arguments.
pub(crate) fn parse(arguments: &Value) -> Result<RenameRequest, Report> {
    let invalid = |message: &str| Report::new(ErrorCode::InvalidOption, message);
    let to = match arguments.get("to") {
        None => {
            return Err(invalid("'to' is required").hint("the new path, e.g. '{dir}/{stem}.bak'"));
        }
        Some(Value::String(to)) if !to.is_empty() => to,
        Some(_) => return Err(invalid("'to' must be a non-empty string")),
    };
    let pattern = match arguments.get("pattern") {
        None => None,
        Some(Value::String(pattern)) => Some(compile(pattern)?),
        Some(_) => return Err(invalid("'pattern' must be a string")),
    };
    let on_conflict = match arguments.get("on_conflict") {
        None => OnConflict::Fail,
        Some(value) => match value.as_str() {
            Some("fail") => OnConflict::Fail,
            Some("skip") => OnConflict::Skip,
            Some("overwrite") => OnConflict::Overwrite,
            _ => {
                return Err(invalid(
                    "'on_conflict' must be one of: fail, skip, overwrite",
                ));
            }
        },
    };
    let groups = pattern.as_ref().map(Groups::of).unwrap_or_default();
    let to = Template::parse(to, &groups).map_err(|e| e.into_field_report("to", to))?;
    Ok(RenameRequest {
        to,
        pattern,
        on_conflict,
    })
}

/// `pattern` checked like `=~` and anchored; a named group may not have the name of a placeholder.
fn compile(pattern: &str) -> Result<regex::Regex, Report> {
    crate::query::user_regex(pattern, false).map_err(|hint| {
        QueryError::new(ErrorCode::InvalidRegex, "invalid regex", 0..pattern.len())
            .hint(hint)
            .into_field_report("pattern", pattern)
    })?;
    let anchored = regex::Regex::new(&format!("^(?:{pattern})$"))
        .map_err(|e| Report::new(ErrorCode::InvalidRegex, "invalid regex").hint(e.to_string()))?;
    if let Some(name) = anchored
        .capture_names()
        .flatten()
        .find(|name| PLACEHOLDERS.contains(name))
    {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            format!("the group '{name}' of 'pattern' has the name of a placeholder"),
        ));
    }
    Ok(anchored)
}

/// Why a target conflicts (TOOLS.md#conflicts).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    Exists,
    SameTarget,
    Selected,
    Directory,
    /// `copy`: a directory into itself.
    Inside,
}

impl Reason {
    fn code(self) -> &'static str {
        match self {
            Self::Exists => "exists",
            Self::SameTarget => "same_target",
            Self::Selected => "selected",
            Self::Directory => "directory",
            Self::Inside => "inside",
        }
    }

    fn text(self) -> &'static str {
        match self {
            Self::Exists => "exists",
            Self::SameTarget => "the same target as another file",
            Self::Selected => "the target is another selected file",
            Self::Directory => "a directory is never replaced",
            Self::Inside => "the target is inside the copied directory",
        }
    }
}

/// An entry with its target.
#[derive(Debug)]
struct Move<'a> {
    entry: &'a Entry,
    /// The old path, its directory canonical.
    from: PathBuf,
    /// The target, its existing directories canonical.
    to: PathBuf,
    /// The target as it is shown.
    shown: PathBuf,
    conflict: Option<Reason>,
    /// An existing target is replaced (`overwrite`).
    replace: bool,
}

/// What is done with an entry, in path order.
#[derive(Debug)]
enum Step<'a> {
    /// It no longer exists.
    Missing(&'a Entry),
    Unchanged(&'a Entry),
    Move(Move<'a>),
}

/// How `rename` and `copy` act.
pub(crate) struct How<'a> {
    pub roots: &'a [PathBuf],
    /// Whether the call must stop: its deadline, its cancellation.
    pub stop: &'a dyn Fn() -> bool,
    /// Moves a replaced target to the trash.
    pub trash: &'a dyn Fn(&Path) -> Result<(), TrashError>,
    pub dry_run: bool,
    pub mode: Mode,
}

/// Renames or copies the entries (in path order, not inside one another): the plan, its conflicts, then the renames
/// or the copies until the call must stop.
pub(crate) fn rename(
    entries: &[Entry],
    request: &RenameRequest,
    how: &How<'_>,
) -> Result<Outcome, Report> {
    let mut outcome = Outcome {
        dry_run: how.dry_run,
        selected: entries.len(),
        ..Outcome::default()
    };
    let allowed = |path: &Path| write::may_change(path, how.roots);
    let mut steps = Vec::new();
    for entry in entries {
        match plan(entry, request, &allowed) {
            Ok(step) => steps.push(step),
            Err(code) => outcome.skip(entry, code),
        }
    }
    conflicts(&mut steps, entries, request.on_conflict, how.mode)?;
    for step in &steps {
        if let Step::Move(m) = step
            && m.conflict.is_some()
        {
            outcome.skip(m.entry, WarningCode::TargetConflict);
        }
    }
    for step in steps {
        if (how.stop)() {
            outcome.warnings.add_global(WarningCode::Timeout);
            break;
        }
        match step {
            Step::Missing(entry) => outcome
                .acted
                .push(Acted::of(entry, Some(write::not_found()))),
            Step::Unchanged(entry) => outcome.acted.push(Acted {
                unchanged: true,
                ..Acted::of(entry, None)
            }),
            Step::Move(m) if m.conflict.is_some() => {}
            Step::Move(m) => {
                let error = if how.dry_run {
                    None
                } else {
                    match act(&m, how) {
                        Ok(()) => None,
                        Err(Failure::Appeared) => {
                            outcome.skip(m.entry, WarningCode::TargetConflict);
                            continue;
                        }
                        Err(Failure::Trash(TrashError::Unavailable)) => {
                            outcome.skip(m.entry, WarningCode::TrashUnavailable);
                            continue;
                        }
                        Err(Failure::Trash(TrashError::Failed(e))) => Some(e),
                    }
                };
                outcome.acted.push(Acted {
                    to: Some(m.shown),
                    replaced: m.replace,
                    ..Acted::of(m.entry, error)
                });
            }
        }
    }
    Ok(outcome)
}

/// The step of an entry, or the warning it is skipped with.
fn plan<'a>(
    entry: &'a Entry,
    request: &RenameRequest,
    allowed: &dyn Fn(&Path) -> bool,
) -> Result<Step<'a>, WarningCode> {
    let Some(from) = write::check(entry, allowed)? else {
        return Ok(Step::Missing(entry));
    };
    let rel = slash_path(entry.rel());
    let captures = match &request.pattern {
        Some(pattern) => Some(pattern.captures(&rel).ok_or(WarningCode::PatternMismatch)?),
        None => None,
    };
    let target = request.to.expand(&rel, captures.as_ref());
    let segments = template::normalize(&target).ok_or(WarningCode::InvalidTarget)?;
    let path = segments
        .iter()
        .fold(entry.dir.clone(), |path, segment| path.join(segment));
    let (Some(parent), Some(name)) = (path.parent(), path.file_name()) else {
        return Err(WarningCode::InvalidTarget);
    };
    let to = crate::paths::canonical_prefix(parent).join(name);
    if !allowed(&to) {
        return Err(WarningCode::PermissionDenied);
    }
    if to == from {
        return Ok(Step::Unchanged(entry));
    }
    let shown = crate::paths::normalize(
        &segments
            .iter()
            .fold(entry.prefix.clone(), |path, segment| path.join(segment)),
    );
    Ok(Step::Move(Move {
        entry,
        from,
        to,
        shown,
        conflict: None,
        replace: false,
    }))
}

/// The path with `/` between its components.
fn slash_path(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Marks the conflicts of the moves; `rename_conflict` or `copy_conflict` when `on_conflict` doesn't resolve them.
fn conflicts(
    steps: &mut [Step<'_>],
    entries: &[Entry],
    on_conflict: OnConflict,
    mode: Mode,
) -> Result<(), Report> {
    let old: HashSet<PathBuf> = entries
        .iter()
        .filter(|e| e.member.is_none())
        .filter_map(|e| write::checked_path(&e.path))
        .collect();
    let mut targets: HashMap<PathBuf, usize> = HashMap::new();
    for step in steps.iter() {
        if let Step::Move(m) = step {
            *targets.entry(m.to.clone()).or_default() += 1;
        }
    }
    let mut unresolved = Vec::new();
    for step in steps.iter_mut() {
        let Step::Move(m) = step else { continue };
        let reason = if targets[&m.to] > 1 {
            Some(Reason::SameTarget)
        } else if old.contains(&m.to) {
            Some(Reason::Selected)
        } else if mode == Mode::Copy && m.entry.is_dir && m.to.starts_with(&m.from) {
            Some(Reason::Inside)
        } else {
            std::fs::symlink_metadata(&m.to).ok().map(|meta| {
                if on_conflict == OnConflict::Overwrite && (meta.is_dir() || m.entry.is_dir) {
                    Reason::Directory
                } else {
                    Reason::Exists
                }
            })
        };
        let Some(reason) = reason else { continue };
        match on_conflict {
            OnConflict::Skip => m.conflict = Some(reason),
            OnConflict::Overwrite if reason == Reason::Exists => m.replace = true,
            _ => unresolved.push((&*m, reason)),
        }
    }
    if unresolved.is_empty() {
        return Ok(());
    }
    let line = |m: &Move<'_>| {
        (
            display_path(&m.entry.shown, m.entry.is_dir),
            display_path(&m.shown, m.entry.is_dir),
        )
    };
    let mut lines: Vec<String> = unresolved
        .iter()
        .take(MAX_CONFLICTS)
        .map(|(m, reason)| {
            let (path, to) = line(m);
            format!("{path} -> {to}: {}", reason.text())
        })
        .collect();
    if unresolved.len() > MAX_CONFLICTS {
        lines.push(format!("… {} more", unresolved.len() - MAX_CONFLICTS));
    }
    let json: Vec<Value> = unresolved
        .iter()
        .map(|(m, reason)| {
            let (path, to) = line(m);
            json!({ "path": path, "to": to, "reason": reason.code() })
        })
        .collect();
    let (code, verb) = match mode {
        Mode::Rename => (ErrorCode::RenameConflict, "renames"),
        Mode::Copy => (ErrorCode::CopyConflict, "copies"),
    };
    let hint = match on_conflict {
        OnConflict::Overwrite => format!(r#"on_conflict = "skip" {verb} the other files"#),
        _ => format!(
            r#"on_conflict = "skip" {verb} the other files, "overwrite" replaces the existing ones"#
        ),
    };
    Err(Report::new(code, "targets conflict; nothing was changed")
        .details(lines, "conflicts", Value::Array(json))
        .hint(hint))
}

/// Why an entry wasn't renamed or copied.
#[derive(Debug)]
enum Failure {
    /// The target appeared after the check.
    Appeared,
    /// The replaced target can't be moved to the trash, or the entry can't be moved.
    Trash(TrashError),
}

/// Renames or copies one entry: the replaced target to the trash, the missing directories, the move or the copy.
fn act(m: &Move<'_>, how: &How<'_>) -> Result<(), Failure> {
    if how.mode == Mode::Copy {
        return copy_into_place(m, how);
    }
    let exists = std::fs::symlink_metadata(&m.to).is_ok();
    if exists && !m.replace {
        return Err(Failure::Appeared);
    }
    if exists {
        (how.trash)(&m.to).map_err(Failure::Trash)?;
    }
    if let Some(parent) = m.to.parent() {
        std::fs::create_dir_all(parent).map_err(|e| Failure::Trash(crate::recycle::failed(&e)))?;
    }
    move_entry(&m.from, &m.to).map_err(Failure::Trash)
}

/// Moves `from` to `to`: an atomic rename, across file systems a copy and the source deleted.
fn move_entry(from: &Path, to: &Path) -> Result<(), TrashError> {
    match std::fs::rename(from, to) {
        Err(e) if e.kind() == std::io::ErrorKind::CrossesDevices => {
            copy_moved(from, to)?;
            write::remove(from)
        }
        done => done.map_err(|e| crate::recycle::failed(&e)),
    }
}

/// Copies one entry under a temporary name next to its target, then renames it into place; the replaced target goes
/// to the trash right before. A copy which isn't put into place is removed.
fn copy_into_place(m: &Move<'_>, how: &How<'_>) -> Result<(), Failure> {
    let failed = |e: &std::io::Error| Failure::Trash(crate::recycle::failed(e));
    let (Some(parent), Some(name)) = (m.to.parent(), m.to.file_name()) else {
        return Err(failed(&std::io::Error::from(
            std::io::ErrorKind::InvalidInput,
        )));
    };
    std::fs::create_dir_all(parent).map_err(|e| failed(&e))?;
    let temporary = parent.join(temporary_name(name));
    if std::fs::symlink_metadata(&temporary).is_ok() {
        return Err(failed(&std::io::Error::from(
            std::io::ErrorKind::AlreadyExists,
        )));
    }
    copy_moved(&m.from, &temporary).map_err(Failure::Trash)?;
    let discard = |failure: Failure| {
        let _ = write::remove(&temporary);
        failure
    };
    if std::fs::symlink_metadata(&m.to).is_ok() {
        if !m.replace {
            return Err(discard(Failure::Appeared));
        }
        (how.trash)(&m.to).map_err(|e| discard(Failure::Trash(e)))?;
    }
    std::fs::rename(&temporary, &m.to).map_err(|e| discard(failed(&e)))
}

/// A hidden name for the copy of `name` which is not in place yet: `.<name>.<pid>-<n>.tmp`.
pub(crate) fn temporary_name(name: &std::ffi::OsStr) -> std::ffi::OsString {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut temporary = std::ffi::OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".{}-{n}.tmp", std::process::id()));
    temporary
}

/// Copies `from` to `to`; a failed copy is removed.
fn copy_moved(from: &Path, to: &Path) -> Result<(), TrashError> {
    copy_tree(from, to).map_err(|e| {
        if std::fs::symlink_metadata(to).is_ok() {
            let _ = write::remove(to);
        }
        crate::recycle::failed(&e)
    })
}

/// Copies an entry: a directory with everything in it, a symlink as a link; the permissions and the modification
/// times are kept (of a symlink where the platform can set it).
fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
    let meta = std::fs::symlink_metadata(from)?;
    let modified = filetime::FileTime::from_last_modification_time(&meta);
    if meta.file_type().is_symlink() {
        let target = std::fs::read_link(from)?;
        symlink(&target, to, from)?;
        let _ = filetime::set_symlink_file_times(to, filetime::FileTime::now(), modified);
        return Ok(());
    }
    if meta.is_dir() {
        std::fs::create_dir(to)?;
        for child in std::fs::read_dir(from)? {
            let child = child?;
            copy_tree(&child.path(), &to.join(child.file_name()))?;
        }
        std::fs::set_permissions(to, meta.permissions())?;
        return filetime::set_file_mtime(to, modified);
    }
    std::fs::copy(from, to)?;
    filetime::set_file_mtime(to, modified)
}

#[cfg(unix)]
fn symlink(target: &Path, link: &Path, _original: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink(target: &Path, link: &Path, original: &Path) -> std::io::Result<()> {
    if std::fs::metadata(original).is_ok_and(|m| m.is_dir()) {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(dir: &Path, rel: &str, is_dir: bool) -> Entry {
        Entry {
            path: dir.join(rel),
            dir: dir.to_path_buf(),
            shown: PathBuf::from(rel),
            prefix: PathBuf::new(),
            member: None,
            is_dir,
            from_result: false,
            stamp: None,
        }
    }

    fn request(arguments: &Value) -> RenameRequest {
        parse(arguments).unwrap()
    }

    fn run(
        root: &Path,
        entries: &[Entry],
        arguments: &Value,
        trashed: &std::cell::RefCell<Vec<PathBuf>>,
    ) -> Result<Outcome, String> {
        run_as(Mode::Rename, root, entries, arguments, trashed)
    }

    fn run_as(
        mode: Mode,
        root: &Path,
        entries: &[Entry],
        arguments: &Value,
        trashed: &std::cell::RefCell<Vec<PathBuf>>,
    ) -> Result<Outcome, String> {
        let roots = [root.to_path_buf()];
        let trash = |path: &Path| {
            trashed.borrow_mut().push(path.to_path_buf());
            std::fs::remove_file(path).map_err(|e| crate::recycle::failed(&e))
        };
        rename(
            entries,
            &request(arguments),
            &How {
                roots: &roots,
                stop: &|| false,
                trash: &trash,
                dry_run: false,
                mode,
            },
        )
        .map_err(|r| r.to_text())
    }

    fn tree() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let root = crate::paths::canonicalize(tmp.path()).unwrap();
        for file in ["a.js", "b.js", "c.ts", "c.tsx"] {
            std::fs::write(root.join(file), file).unwrap();
        }
        std::fs::create_dir(root.join("d")).unwrap();
        (tmp, root)
    }

    #[test]
    fn the_conflicts_of_the_plan() {
        let (_tmp, root) = tree();
        let trashed = std::cell::RefCell::default();
        let js = [entry(&root, "a.js", false), entry(&root, "b.js", false)];
        let error = run(&root, &js, &json!({"to": "all.js"}), &trashed).unwrap_err();
        assert!(
            error.contains("a.js -> all.js: the same target as another file\n"),
            "{error}"
        );
        let error = run(&root, &js, &json!({"to": "b.js"}), &trashed).unwrap_err();
        assert!(
            error.contains("  a.js -> b.js: the target is another selected file\n"),
            "{error}"
        );
        let ts = [entry(&root, "c.ts", false)];
        let error = run(&root, &ts, &json!({"to": "{stem}.tsx"}), &trashed).unwrap_err();
        assert!(error.contains("  c.ts -> c.tsx: exists\n"), "{error}");
        let dir = [entry(&root, "d", true)];
        let error = run(
            &root,
            &dir,
            &json!({"to": "c.ts", "on_conflict": "overwrite"}),
            &trashed,
        )
        .unwrap_err();
        let sep = std::path::MAIN_SEPARATOR;
        assert!(
            error.contains(&format!(
                "  d{sep} -> c.ts{sep}: a directory is never replaced\n"
            )),
            "{error}"
        );
        assert!(trashed.borrow().is_empty());
        assert!(root.join("a.js").exists() && root.join("d").is_dir());
    }

    #[test]
    fn skip_and_overwrite() {
        let (_tmp, root) = tree();
        let trashed = std::cell::RefCell::default();
        let entries = [entry(&root, "a.js", false), entry(&root, "c.ts", false)];
        let outcome = run(
            &root,
            &entries,
            &json!({"to": "{stem}.tsx", "on_conflict": "skip"}),
            &trashed,
        )
        .unwrap();
        assert_eq!(outcome.skipped, 1);
        assert!(root.join("a.tsx").exists() && root.join("c.ts").exists());
        let outcome = run(
            &root,
            &[entry(&root, "c.ts", false)],
            &json!({"to": "sub/../{stem}.tsx", "on_conflict": "overwrite"}),
            &trashed,
        )
        .unwrap();
        assert!(outcome.acted[0].replaced);
        assert_eq!(*trashed.borrow(), [root.join("c.tsx")]);
        assert_eq!(std::fs::read_to_string(root.join("c.tsx")).unwrap(), "c.ts");
    }

    #[test]
    fn a_copy_across_file_systems_keeps_links_and_trees() {
        let (_tmp, root) = tree();
        std::fs::write(root.join("d/x.txt"), "x").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("x.txt", root.join("d/link")).unwrap();
        copy_moved(&root.join("d"), &root.join("e")).unwrap();
        assert_eq!(std::fs::read_to_string(root.join("e/x.txt")).unwrap(), "x");
        #[cfg(unix)]
        assert_eq!(
            std::fs::read_link(root.join("e/link")).unwrap(),
            PathBuf::from("x.txt")
        );
        // a failed copy is removed
        assert!(copy_moved(&root.join("missing"), &root.join("f")).is_err());
        assert!(copy_moved(&root.join("d"), &root.join("e/x.txt/y")).is_err());
        assert!(!root.join("f").exists());
    }

    /// The names in `dir`, sorted.
    fn names(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn a_copy_keeps_the_source_contents_and_times() {
        let (_tmp, root) = tree();
        std::fs::write(root.join("d/x.txt"), "x").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("x.txt", root.join("d/link")).unwrap();
        let old = filetime::FileTime::from_unix_time(1_790_000_000, 0);
        filetime::set_file_mtime(root.join("d/x.txt"), old).unwrap();
        filetime::set_file_mtime(root.join("d"), old).unwrap();
        filetime::set_file_mtime(root.join("a.js"), old).unwrap();
        let trashed = std::cell::RefCell::default();
        let entries = [entry(&root, "a.js", false), entry(&root, "d", true)];
        let outcome = run_as(
            Mode::Copy,
            &root,
            &entries,
            &json!({"to": "copy/{name}"}),
            &trashed,
        )
        .unwrap();
        assert_eq!(outcome.acted.len(), 2);
        let mtime = |path: &Path| {
            filetime::FileTime::from_last_modification_time(
                &std::fs::symlink_metadata(path).unwrap(),
            )
        };
        for path in ["a.js", "d", "d/x.txt"] {
            assert!(root.join(path).exists(), "the source {path} stays");
            assert_eq!(mtime(&root.join("copy").join(path)), old, "{path}");
        }
        assert_eq!(
            std::fs::read_to_string(root.join("copy/a.js")).unwrap(),
            "a.js"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("copy/d/x.txt")).unwrap(),
            "x"
        );
        #[cfg(unix)]
        assert_eq!(
            std::fs::read_link(root.join("copy/d/link")).unwrap(),
            PathBuf::from("x.txt")
        );
        // no temporary copy is left
        assert_eq!(names(&root.join("copy")), ["a.js", "d"]);
    }

    #[cfg(unix)]
    #[test]
    fn a_copy_keeps_the_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let (_tmp, root) = tree();
        std::fs::set_permissions(root.join("a.js"), std::fs::Permissions::from_mode(0o750))
            .unwrap();
        let trashed = std::cell::RefCell::default();
        run_as(
            Mode::Copy,
            &root,
            &[entry(&root, "a.js", false)],
            &json!({"to": "{stem}.bak.{ext}"}),
            &trashed,
        )
        .unwrap();
        let mode = std::fs::metadata(root.join("a.bak.js"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o750);
    }

    #[test]
    fn a_failed_copy_leaves_nothing() {
        let (_tmp, root) = tree();
        let trashed: std::cell::RefCell<Vec<PathBuf>> = std::cell::RefCell::default();
        // the source is gone after the plan: copy_into_place fails and removes its temporary copy
        let m = Move {
            entry: &entry(&root, "a.js", false),
            from: root.join("missing.js"),
            to: root.join("out/a.js"),
            shown: PathBuf::from("out/a.js"),
            conflict: None,
            replace: false,
        };
        let trash = |path: &Path| {
            trashed.borrow_mut().push(path.to_path_buf());
            Ok(())
        };
        let how = How {
            roots: std::slice::from_ref(&root),
            stop: &|| false,
            trash: &trash,
            dry_run: false,
            mode: Mode::Copy,
        };
        assert!(act(&m, &how).is_err());
        assert_eq!(names(&root.join("out")).len(), 0);
    }

    #[test]
    fn copy_overwrite_keeps_the_source() {
        let (_tmp, root) = tree();
        let trashed = std::cell::RefCell::default();
        let outcome = run_as(
            Mode::Copy,
            &root,
            &[entry(&root, "c.ts", false)],
            &json!({"to": "{stem}.tsx", "on_conflict": "overwrite"}),
            &trashed,
        )
        .unwrap();
        assert!(outcome.acted[0].replaced);
        assert_eq!(*trashed.borrow(), [root.join("c.tsx")]);
        assert_eq!(std::fs::read_to_string(root.join("c.tsx")).unwrap(), "c.ts");
        assert_eq!(std::fs::read_to_string(root.join("c.ts")).unwrap(), "c.ts");
    }

    #[test]
    fn a_directory_copied_into_itself() {
        let (_tmp, root) = tree();
        let trashed = std::cell::RefCell::default();
        let dir = [entry(&root, "d", true)];
        let error = run_as(
            Mode::Copy,
            &root,
            &dir,
            &json!({"to": "d/sub/d", "on_conflict": "overwrite"}),
            &trashed,
        )
        .unwrap_err();
        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(
            error,
            format!(
                "error[copy_conflict]: targets conflict; nothing was changed\n  d{sep} -> d{sep}sub{sep}d{sep}: the target is inside the copied directory\n  hint: on_conflict = \"skip\" copies the other files\n"
            )
        );
        // a sibling which starts with the same name is not inside
        let error = run_as(
            Mode::Copy,
            &root,
            &dir,
            &json!({"to": "{name}.bak"}),
            &trashed,
        );
        assert!(error.is_ok());
        assert!(root.join("d.bak").is_dir() && root.join("d").is_dir());
    }

    #[test]
    fn the_options_of_rename() {
        let text = |arguments: Value| parse(&arguments).unwrap_err().to_text();
        assert_eq!(
            text(json!({"to": "{name}", "pattern": 1})),
            "error[invalid_option]: 'pattern' must be a string\n"
        );
        assert_eq!(
            text(json!({"to": 1})),
            "error[invalid_option]: 'to' must be a non-empty string\n"
        );
        assert!(
            text(json!({"to": "{name}", "pattern": "(a"}))
                .starts_with("error[invalid_regex]: invalid regex\n  (a\n  ^^\n")
        );
        let json: Value =
            serde_json::from_str(&parse(&json!({"to": "{x}"})).unwrap_err().to_json()).unwrap();
        assert_eq!(json["error"]["to"], "{x}");
    }
}
