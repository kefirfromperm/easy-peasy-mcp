//! Walks the working directory and collects the files which match the query (CONFIG.md#traversal).

mod kinds;

use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

use ignore::{WalkBuilder, WalkState};

pub use kinds::KindRules;

use crate::content::{Content, is_binary};
use crate::git::{GitIndex, GitStatus};
use crate::languages::LanguageTable;
use crate::output::{Fragment, matched_lines, merge_identical};
use crate::query::UNKNOWN_LANG;
use crate::query::{
    ContentState, EntryType, FileValues, Filter, Kind, Query, Truth, ast::CondId, nfc,
};
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
    /// The content fragments, if any content condition made the file match.
    pub matches: Option<FileMatches>,
}

/// The content fragments of a matched file and its counts (OUTPUT.md#counts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMatches {
    /// The content, for the matched lines of the output.
    pub content: Content,
    /// Identical fragments merged, sorted by position.
    pub fragments: Vec<Fragment>,
    /// The number of lines touched by fragments.
    pub lines: usize,
}

/// The fragments of the contributing content conditions; `None` when there is none.
fn file_matches(content: &Content, leaves: &[&Filter]) -> Option<FileMatches> {
    let mut fragments = Vec::new();
    for leaf in leaves {
        if let Filter::Content { id, order, regex } = leaf {
            fragments.extend(regex.find_iter(content.text()).map(|m| Fragment {
                range: m.range(),
                ids: vec![(*order, id.clone())],
            }));
        }
    }
    if fragments.is_empty() {
        return None;
    }
    let fragments = merge_identical(fragments);
    let lines = matched_lines(content, &fragments);
    Some(FileMatches {
        content: content.clone(),
        fragments,
        lines,
    })
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
    let walk = Walk {
        root,
        query,
        options,
        kinds: KindRules::builtin(options.case_insensitive_paths),
        git: GitIndex::new(!options.git_global_excludes),
        found: Mutex::default(),
        warnings: Mutex::default(),
    };
    builder.build_parallel().run(|| {
        let walk = &walk;
        Box::new(move |entry| {
            if let Ok(entry) = entry
                && let Some(item) = walk.visit(&entry)
            {
                walk.found
                    .lock()
                    .expect("no thread panics while holding the lock")
                    .push(item);
            }
            WalkState::Continue
        })
    });
    let mut found = walk.found.into_inner().expect("the walk is finished");
    found.sort_by(|a, b| a.0.rel.cmp(&b.0.rel));
    let mut seen = std::collections::HashSet::new();
    let files = found
        .into_iter()
        .filter(|(_, id)| id.is_none_or(|id| seen.insert(id)))
        .map(|(f, _)| f)
        .collect();
    SearchResult {
        files,
        warnings: walk.warnings.into_inner().expect("the walk is finished"),
    }
}

/// The state of one walk, shared by its threads.
struct Walk<'a> {
    root: &'a Path,
    query: &'a Query,
    options: &'a SearchOptions,
    kinds: KindRules,
    git: GitIndex,
    found: Mutex<Vec<(Found, Option<FileId>)>>,
    warnings: Mutex<Warnings>,
}

/// The dates of an entry in seconds since the Unix epoch.
struct Dates {
    modified: Option<f64>,
    created: Option<f64>,
    accessed: Option<f64>,
}

impl Walk<'_> {
    /// The matched entry and its hard link identity, or `None` if the entry doesn't match or is skipped.
    fn visit(&self, entry: &ignore::DirEntry) -> Option<(Found, Option<FileId>)> {
        let query = self.query;
        if entry.depth() == 0 {
            return None;
        }
        let entry_type = entry_type(entry)?;
        if !query.has_type && entry_type != EntryType::File {
            return None;
        }
        let rel = entry.path().strip_prefix(self.root).ok()?;
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
        let dates = self.dates(metadata.as_ref(), rel, entry_type);
        let language = (query.needs.lang && entry_type != EntryType::Dir)
            .then(|| {
                self.options
                    .languages
                    .detect(&name, || first_line(entry.path()))
            })
            .flatten();
        let kind = if query.needs.kind {
            self.kinds.classify(
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
            size: metadata.as_ref().map_or(0, std::fs::Metadata::len),
            modified: dates.modified,
            created: dates.created,
            accessed: dates.accessed,
            lang: language.map_or(UNKNOWN_LANG, |l| l.name.as_str()),
            kind,
            git: self.git_status(entry.path(), rel, entry_type),
            content: if query.needs.content && entry_type == EntryType::File {
                ContentState::NotRead
            } else {
                ContentState::Absent
            },
        };
        let (conditions, matches) = match &query.filter {
            None => (Vec::new(), None),
            Some(filter) => self.evaluate(filter, file, entry.path(), rel)?,
        };
        let is_file = entry_type == EntryType::File;
        if !query.needs.kind && is_file && self.kinds.is_generated(&path, entry.path()) {
            return None;
        }
        let id = is_file
            .then(|| entry.metadata().ok().and_then(|m| file_id(&m)))
            .flatten();
        let found = Found {
            rel: rel.to_path_buf(),
            is_dir: entry_type == EntryType::Dir,
            conditions,
            matches,
        };
        Some((found, id))
    }

    /// The conditions which made the file match and its content fragments, or `None` if it doesn't match. The file
    /// is read only when the conditions without the content don't decide; a binary file's content is unknown.
    fn evaluate(
        &self,
        filter: &Filter,
        file: FileValues<'_>,
        path: &Path,
        rel: &Path,
    ) -> Option<(Vec<CondId>, Option<FileMatches>)> {
        if filter.eval(&file) == Truth::False {
            return None;
        }
        let content: Content;
        // a copy with a shorter lifetime, which can borrow the content read here
        let mut file = file;
        if matches!(file.content, ContentState::NotRead) {
            let bytes = std::fs::read(path).ok()?;
            if is_binary(&bytes) {
                file.content = ContentState::Unknown;
                self.warn(WarningCode::BinarySkipped, rel, false);
            } else {
                content = Content::from_bytes(&bytes);
                file.content = ContentState::Text(&content);
            }
        }
        if !filter.matches(&file) {
            return None;
        }
        let leaves = filter.contributing_leaves(&file);
        let conditions = leaves
            .iter()
            .filter_map(|f| f.id())
            .map(|(_, id)| id.clone())
            .collect();
        let matches = match file.content {
            ContentState::Text(content) => file_matches(content, &leaves),
            _ => None,
        };
        Some((conditions, matches))
    }

    /// The dates of an entry; `date_unsupported` when the query needs a date the file system doesn't provide.
    fn dates(
        &self,
        metadata: Option<&std::fs::Metadata>,
        rel: &Path,
        entry_type: EntryType,
    ) -> Dates {
        let date = |get: fn(&std::fs::Metadata) -> std::io::Result<std::time::SystemTime>| {
            metadata.and_then(|m| get(m).ok()).map(system_seconds)
        };
        let dates = Dates {
            modified: date(std::fs::Metadata::modified),
            created: date(std::fs::Metadata::created),
            accessed: date(std::fs::Metadata::accessed),
        };
        let needs = self.query.needs;
        if (needs.created && dates.created.is_none())
            || (needs.accessed && dates.accessed.is_none())
        {
            self.warn(
                WarningCode::DateUnsupported,
                rel,
                entry_type == EntryType::Dir,
            );
        }
        dates
    }

    /// The git status of a file; `not_a_git_repo` when the query has `git` and the file is outside of a repository.
    fn git_status(&self, path: &Path, rel: &Path, entry_type: EntryType) -> Option<GitStatus> {
        if !self.query.needs.git || entry_type == EntryType::Dir {
            return None;
        }
        let status = self.git.status(path);
        if status.is_none() && !self.git.in_repo(path) {
            self.warn(WarningCode::NotAGitRepo, rel, false);
        }
        status
    }

    fn warn(&self, code: WarningCode, rel: &Path, is_dir: bool) {
        self.warnings
            .lock()
            .expect("no thread panics while holding the lock")
            .add(code, rel.to_path_buf(), is_dir);
    }
}

/// The type of an entry, symlinks not followed; `None` for other entries (sockets, devices).
fn entry_type(entry: &ignore::DirEntry) -> Option<EntryType> {
    let t = entry.file_type()?;
    if t.is_symlink() {
        Some(EntryType::Link)
    } else if t.is_dir() {
        Some(EntryType::Dir)
    } else if t.is_file() {
        Some(EntryType::File)
    } else {
        None
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
