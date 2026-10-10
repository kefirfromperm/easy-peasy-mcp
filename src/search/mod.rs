//! Walks the working directory and collects the files which match the query (CONFIG.md#traversal).

mod generated;

use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

use ignore::{WalkBuilder, WalkState};

pub use generated::GeneratedRules;

use crate::query::{Filter, Query, TextTarget, ast::CondId, nfc};

/// The ignore files respected in every directory, a later one wins at the same level.
pub const IGNORE_FILES: [&str; 3] = [".gitignore", ".ignore", ".aiignore"];

/// Settings of one search.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Whether the global git excludes file is respected.
    pub git_global_excludes: bool,
    /// `path_case`: whether paths and names are compared case-insensitively.
    pub case_insensitive_paths: bool,
}

/// A matched file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// The path relative to the working directory.
    pub rel: PathBuf,
    /// The ids of the conditions which made the file match.
    pub conditions: Vec<CondId>,
}

/// A device and inode pair which identifies a file reachable by several hard links.
type FileId = (u64, u64);

/// Searches `root` and returns the matched files in path order, every file once.
pub fn search(root: &Path, query: &Query, options: &SearchOptions) -> Vec<Found> {
    let generated = GeneratedRules::builtin(options.case_insensitive_paths);
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
    builder.build_parallel().run(|| {
        let found = &found;
        let generated = &generated;
        Box::new(move |entry| {
            let Ok(entry) = entry else {
                return WalkState::Continue;
            };
            if !entry.file_type().is_some_and(|t| t.is_file()) {
                return WalkState::Continue;
            }
            let Ok(rel) = entry.path().strip_prefix(root) else {
                return WalkState::Continue;
            };
            let path = slash_path(rel);
            let file = FileInfo {
                name: nfc(&entry.file_name().to_string_lossy()),
                path: nfc(&path),
            };
            let Some(conditions) = evaluate(query.filter.as_ref(), &file) else {
                return WalkState::Continue;
            };
            if generated.is_generated(&path, entry.path()) {
                return WalkState::Continue;
            }
            let id = entry.metadata().ok().and_then(|m| file_id(&m));
            let item = (
                Found {
                    rel: rel.to_path_buf(),
                    conditions,
                },
                id,
            );
            found
                .lock()
                .expect("no thread panics while holding the lock")
                .push(item);
            WalkState::Continue
        })
    });
    let mut found = found.into_inner().expect("the walk is finished");
    found.sort_by(|a, b| a.0.rel.cmp(&b.0.rel));
    let mut seen = std::collections::HashSet::new();
    found
        .into_iter()
        .filter(|(_, id)| id.is_none_or(|id| seen.insert(id)))
        .map(|(f, _)| f)
        .collect()
}

/// The values of a file which conditions look at.
struct FileInfo {
    /// The NFC file name.
    name: String,
    /// The NFC path relative to the working directory, `/` as the separator.
    path: String,
}

/// The ids of the conditions which made the file match, or `None` if it doesn't match; no filter matches all.
fn evaluate(filter: Option<&Filter>, file: &FileInfo) -> Option<Vec<CondId>> {
    match filter {
        None => Some(Vec::new()),
        Some(Filter::Text {
            id,
            target,
            matcher,
        }) => {
            let value = match target {
                TextTarget::Name => &file.name,
                TextTarget::Path => &file.path,
            };
            matcher.is_match(value).then(|| vec![id.clone()])
        }
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
    fn slash_paths() {
        let rel: PathBuf = ["src", "parser", "lexer.rs"].iter().collect();
        assert_eq!(slash_path(&rel), "src/parser/lexer.rs");
        assert_eq!(slash_path(Path::new("a.rs")), "a.rs");
    }
}
