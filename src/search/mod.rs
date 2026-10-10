//! Walks the working directory and collects the files which match the query (CONFIG.md#traversal).

mod kinds;

use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

use ignore::{WalkBuilder, WalkState};

pub use kinds::KindRules;

use crate::languages::LanguageTable;
use crate::query::UNKNOWN_LANG;
use crate::query::{EntryType, FileValues, Kind, Query, ast::CondId, nfc};
use crate::warning::{WarningCode, Warnings};

/// The ignore files respected in every directory, a later one wins at the same level.
pub const IGNORE_FILES: [&str; 3] = [".gitignore", ".ignore", ".aiignore"];

/// Settings of one search.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Whether the global git excludes file is respected.
    pub git_global_excludes: bool,
    /// `path_case`: whether paths and names are compared case-insensitively.
    pub case_insensitive_paths: bool,
    /// The language table.
    pub languages: std::sync::Arc<LanguageTable>,
}

/// A matched file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// The path relative to the working directory.
    pub rel: PathBuf,
    /// Whether the entry is a directory, written with a trailing separator.
    pub is_dir: bool,
    /// The ids of the conditions which made the file match.
    pub conditions: Vec<CondId>,
}

/// The result of a search: the matched entries and the warnings.
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    pub files: Vec<Found>,
    pub warnings: Warnings,
}

/// A device and inode pair which identifies a file reachable by several hard links.
type FileId = (u64, u64);

/// Searches `root` and returns the matched entries in path order, every file once; without a `type` condition only
/// regular files.
pub fn search(root: &Path, query: &Query, options: &SearchOptions) -> SearchResult {
    let kinds = KindRules::builtin(options.case_insensitive_paths);
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(true)
        .parents(true)
        .ignore(false)
        .git_ignore(false)
        .git_global(options.git_global_excludes)
        .git_exclude(true)
        .require_git(false)
        .follow_links(false);
    for name in IGNORE_FILES {
        builder.add_custom_ignore_filename(name);
    }
    let found: Mutex<Vec<(Found, Option<FileId>)>> = Mutex::new(Vec::new());
    let warnings = Mutex::new(Warnings::default());
    builder.build_parallel().run(|| {
        let found = &found;
        let warnings = &warnings;
        let kinds = &kinds;
        Box::new(move |entry| {
            if let Ok(entry) = entry
                && let Some(item) = visit(&entry, root, query, options, kinds, warnings)
            {
                found
                    .lock()
                    .expect("no thread panics while holding the lock")
                    .push(item);
            }
            WalkState::Continue
        })
    });
    let mut found = found.into_inner().expect("the walk is finished");
    found.sort_by(|a, b| a.0.rel.cmp(&b.0.rel));
    let mut seen = std::collections::HashSet::new();
    let files = found
        .into_iter()
        .filter(|(_, id)| id.is_none_or(|id| seen.insert(id)))
        .map(|(f, _)| f)
        .collect();
    SearchResult {
        files,
        warnings: warnings.into_inner().expect("the walk is finished"),
    }
}

/// A regular file of size 0 or a directory without any entry on disk; a symlink is never empty.
fn is_empty(path: &Path, entry_type: EntryType, metadata: &std::fs::Metadata) -> bool {
    match entry_type {
        EntryType::File => metadata.len() == 0,
        EntryType::Dir => std::fs::read_dir(path).is_ok_and(|mut d| d.next().is_none()),
        EntryType::Link => false,
    }
}

/// The matched entry and its hard link identity, or `None` if the entry doesn't match or is skipped.
fn visit(
    entry: &ignore::DirEntry,
    root: &Path,
    query: &Query,
    options: &SearchOptions,
    kinds: &KindRules,
    warnings: &Mutex<Warnings>,
) -> Option<(Found, Option<FileId>)> {
    if entry.depth() == 0 {
        return None;
    }
    let entry_type = entry.file_type().and_then(|t| {
        if t.is_symlink() {
            Some(EntryType::Link)
        } else if t.is_dir() {
            Some(EntryType::Dir)
        } else if t.is_file() {
            Some(EntryType::File)
        } else {
            None
        }
    })?;
    if !query.has_type && entry_type != EntryType::File {
        return None;
    }
    let Ok(rel) = entry.path().strip_prefix(root) else {
        return None;
    };
    let path = slash_path(rel);
    let name = nfc(&entry.file_name().to_string_lossy());
    let nfc_path = nfc(&path);
    let metadata = (query.needs.empty || query.needs.metadata)
        .then(|| entry.metadata().ok())
        .flatten();
    let empty = query.needs.empty
        && metadata
            .as_ref()
            .is_some_and(|m| is_empty(entry.path(), entry_type, m));
    let size = metadata.as_ref().map_or(0, std::fs::Metadata::len);
    let modified = metadata
        .as_ref()
        .and_then(|m| m.modified().ok())
        .map(system_seconds);
    let created = metadata
        .as_ref()
        .and_then(|m| m.created().ok())
        .map(system_seconds);
    let accessed = metadata
        .as_ref()
        .and_then(|m| m.accessed().ok())
        .map(system_seconds);
    if (query.needs.created && created.is_none()) || (query.needs.accessed && accessed.is_none()) {
        warnings
            .lock()
            .expect("no thread panics while holding the lock")
            .add(
                WarningCode::DateUnsupported,
                rel.to_path_buf(),
                entry_type == EntryType::Dir,
            );
    }
    let language = (query.needs.lang && entry_type != EntryType::Dir)
        .then(|| options.languages.detect(&name, || first_line(entry.path())))
        .flatten();
    let lang = language.map_or(UNKNOWN_LANG, |l| l.name.as_str());
    let kind = if query.needs.kind {
        kinds.classify(
            &path,
            entry.path(),
            entry_type == EntryType::File,
            language.is_some_and(|l| l.source),
        )
    } else {
        Kind::Other
    };
    let file = FileValues {
        name: &name,
        path: &nfc_path,
        entry_type,
        empty,
        size,
        modified,
        created,
        accessed,
        lang,
        kind,
    };
    let conditions = match &query.filter {
        None => Vec::new(),
        Some(filter) if filter.matches(&file) => filter.contributors(&file),
        Some(_) => return None,
    };
    if !query.needs.kind && entry_type == EntryType::File && kinds.is_generated(&path, entry.path())
    {
        return None;
    }
    let id = (entry_type == EntryType::File)
        .then(|| entry.metadata().ok().and_then(|m| file_id(&m)))
        .flatten();
    let item = (
        Found {
            rel: rel.to_path_buf(),
            is_dir: entry_type == EntryType::Dir,
            conditions,
        },
        id,
    );
    Some(item)
}

/// The first line of a file, at most 1K, for the shebang; `None` if it can't be read.
fn first_line(path: &Path) -> Option<String> {
    use std::io::{BufRead, Read};
    let file = std::fs::File::open(path).ok()?;
    let mut line = Vec::new();
    std::io::BufReader::new(file.take(1024))
        .read_until(b'\n', &mut line)
        .ok()?;
    Some(String::from_utf8_lossy(&line).trim_end().to_string())
}

/// Seconds since the Unix epoch, negative before it.
fn system_seconds(time: std::time::SystemTime) -> f64 {
    match time.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs_f64(),
        Err(e) => -e.duration().as_secs_f64(),
    }
}

/// A relative path with `/` separators.
pub fn slash_path(rel: &Path) -> String {
    let parts: Vec<String> = rel
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();
    parts.join("/")
}

#[cfg(unix)]
fn file_id(metadata: &std::fs::Metadata) -> Option<FileId> {
    use std::os::unix::fs::MetadataExt;
    (metadata.nlink() > 1).then(|| (metadata.dev(), metadata.ino()))
}

#[cfg(not(unix))]
fn file_id(_metadata: &std::fs::Metadata) -> Option<FileId> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptiness() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir(root.join("empty")).unwrap();
        std::fs::create_dir(root.join("hidden")).unwrap();
        std::fs::write(root.join("hidden/.keep"), "").unwrap();
        std::fs::write(root.join("zero"), "").unwrap();
        std::fs::write(root.join("one"), "1").unwrap();
        let meta = |p: &str| std::fs::metadata(root.join(p)).unwrap();
        assert!(is_empty(
            &root.join("empty"),
            EntryType::Dir,
            &meta("empty")
        ));
        assert!(!is_empty(
            &root.join("hidden"),
            EntryType::Dir,
            &meta("hidden")
        ));
        assert!(is_empty(&root.join("zero"), EntryType::File, &meta("zero")));
        assert!(!is_empty(&root.join("one"), EntryType::File, &meta("one")));
    }

    #[test]
    fn slash_paths() {
        let rel: PathBuf = ["src", "parser", "lexer.rs"].iter().collect();
        assert_eq!(slash_path(&rel), "src/parser/lexer.rs");
        assert_eq!(slash_path(Path::new("a.rs")), "a.rs");
    }
}
