//! Walks the working directory and collects the files which match the query (CONFIG.md#traversal).

mod kinds;

use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};

use ignore::{WalkBuilder, WalkState};

pub use kinds::KindRules;

use crate::content::{Content, Decoded, Encoding, decode};
use crate::git::{GitIndex, GitStatus};
use crate::languages::{Language, LanguageTable};
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
    /// When the search must stop; `None` — no limit.
    pub deadline: Option<Instant>,
    /// The threads of the walk and the file search; `0` — the number of CPU cores.
    pub threads: usize,
    /// Entries closer to the working directory are not returned; its children are depth 1.
    pub min_depth: usize,
    /// The walk doesn't descend deeper; `None` — unlimited.
    pub max_depth: Option<usize>,
    /// Descend into symlinked directories and read symlinked files.
    pub follow_symlinks: bool,
    /// Include hidden entries: names starting with `.`.
    pub hidden: bool,
    /// The canonical allowed roots: a followed symlink must point inside them.
    pub allowed_roots: std::sync::Arc<Vec<PathBuf>>,
    /// A test hook: every read of a file's content takes this long, cut short at the deadline.
    pub read_delay: Option<Duration>,
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
    /// The encoding of a file whose content was read, if it is not UTF-8.
    pub encoding: Option<Encoding>,
    /// The identity in the file system, to return an entry reachable several times once.
    pub id: Option<FileId>,
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

/// The fragments of a `line( ... )` group or one side of a `near( ... )` in the given lines: the matches of the
/// contributing text conditions of `inner` with the ids of the condition and the group, or the whole line with the
/// id of the group.
fn line_fragments(
    content: &Content,
    group: (usize, &CondId),
    inner: &Filter,
    lines: &[usize],
) -> Vec<Fragment> {
    let group = (group.0, group.1.clone());
    let mut fragments = Vec::new();
    for &i in lines {
        let line = content.line_ref(i);
        let (start, end) = content.line_range(i);
        let before = fragments.len();
        for leaf in inner.contributing_in_line(line) {
            if let Filter::Content { id, order, pattern } = leaf {
                fragments.extend(pattern.line_matches(line).into_iter().map(|r| Fragment {
                    escape: pattern.escape_level(line.text, r.start),
                    range: start + r.start..start + r.end,
                    ids: vec![group.clone(), (*order, id.clone())],
                }));
            }
        }
        if fragments.len() == before {
            fragments.push(Fragment {
                range: start..end,
                ids: vec![group.clone()],
                escape: None,
            });
        }
    }
    fragments
}

/// The fragments of the contributing content conditions; `None` when there is none.
fn file_matches(content: &Content, leaves: &[&Filter]) -> Option<FileMatches> {
    let mut fragments = Vec::new();
    for leaf in leaves {
        if let Filter::Content { id, order, pattern } = leaf {
            fragments.extend(
                pattern
                    .content_matches(content)
                    .into_iter()
                    .map(|range| Fragment {
                        escape: pattern.escape_level(content.text(), range.start),
                        range,
                        ids: vec![(*order, id.clone())],
                    }),
            );
        }
    }
    for leaf in leaves {
        match leaf {
            Filter::Line { id, order, inner } => {
                let lines: Vec<usize> = (0..content.line_count())
                    .filter(|&i| inner.eval_line(content.line_ref(i)))
                    .collect();
                fragments.extend(line_fragments(content, (*order, id), inner, &lines));
            }
            Filter::Near {
                id, order, a, b, ..
            } => {
                let pairs = leaf.near_pairs(content);
                let mut a_lines: Vec<usize> = pairs.iter().map(|p| p.0).collect();
                let mut b_lines: Vec<usize> = pairs.iter().map(|p| p.1).collect();
                a_lines.sort_unstable();
                a_lines.dedup();
                b_lines.sort_unstable();
                b_lines.dedup();
                fragments.extend(line_fragments(content, (*order, id), a, &a_lines));
                fragments.extend(line_fragments(content, (*order, id), b, &b_lines));
            }
            _ => {}
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
    /// Whether the search was stopped at the deadline: the files are those found so far.
    pub timed_out: bool,
}

/// A device and inode pair which identifies a file reachable by several hard links.
pub type FileId = (u64, u64);

/// Searches `root` and returns the matched entries in path order, every file once; without a `type` condition only
/// regular files.
pub fn search(root: &Path, query: &Query, options: &SearchOptions) -> SearchResult {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(!options.hidden)
        .parents(true)
        .ignore(false)
        .git_ignore(false)
        .git_global(options.git_global_excludes)
        .git_exclude(true)
        .require_git(false)
        .follow_links(options.follow_symlinks)
        .threads(options.threads)
        .max_depth(options.max_depth);
    let outside: std::sync::Arc<Mutex<Vec<(PathBuf, bool)>>> = std::sync::Arc::default();
    if options.follow_symlinks {
        let roots = std::sync::Arc::clone(&options.allowed_roots);
        let outside = std::sync::Arc::clone(&outside);
        let base = root.to_path_buf();
        builder.filter_entry(move |entry| {
            if !entry.path_is_symlink() {
                return true;
            }
            let target = std::fs::canonicalize(entry.path());
            if target
                .as_ref()
                .is_ok_and(|t| roots.iter().any(|r| t.starts_with(r)))
            {
                return true;
            }
            if let (Ok(target), Ok(rel)) = (&target, entry.path().strip_prefix(&base)) {
                outside
                    .lock()
                    .expect("no thread panics while holding the lock")
                    .push((rel.to_path_buf(), target.is_dir()));
            }
            false
        });
    }
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
        timed_out: AtomicBool::new(false),
    };
    builder.build_parallel().run(|| {
        let walk = &walk;
        Box::new(move |entry| {
            if walk.past_deadline() {
                return WalkState::Quit;
            }
            match entry {
                Ok(entry) => {
                    if let Some(item) = walk.visit(&entry) {
                        walk.found
                            .lock()
                            .expect("no thread panics while holding the lock")
                            .push(item);
                    }
                }
                Err(err) => walk.walk_error(&err),
            }
            WalkState::Continue
        })
    });
    let mut found = walk.found.into_inner().expect("the walk is finished");
    let mut warnings = walk.warnings.into_inner().expect("the walk is finished");
    for (rel, is_dir) in outside
        .lock()
        .expect("no thread panics while holding the lock")
        .drain(..)
    {
        warnings.add(WarningCode::PermissionDenied, rel, is_dir);
    }
    found.sort_by(|a, b| a.rel.cmp(&b.rel));
    let mut seen = std::collections::HashSet::new();
    let files = found
        .into_iter()
        .filter(|f| f.id.is_none_or(|id| seen.insert(id)))
        .collect();
    SearchResult {
        files,
        warnings,
        timed_out: walk.timed_out.into_inner(),
    }
}

/// The state of one walk, shared by its threads.
struct Walk<'a> {
    root: &'a Path,
    query: &'a Query,
    options: &'a SearchOptions,
    kinds: KindRules,
    git: GitIndex,
    found: Mutex<Vec<Found>>,
    warnings: Mutex<Warnings>,
    /// Set when the deadline passed.
    timed_out: AtomicBool,
}

/// The dates of an entry in seconds since the Unix epoch.
struct Dates {
    modified: Option<f64>,
    created: Option<f64>,
    accessed: Option<f64>,
}

impl Walk<'_> {
    /// The matched entry and its hard link identity, or `None` if the entry doesn't match or is skipped.
    fn visit(&self, entry: &ignore::DirEntry) -> Option<Found> {
        let query = self.query;
        if entry.depth() == 0 || entry.depth() < self.options.min_depth {
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
        let language = ((query.needs.lang || query.needs.comments) && entry_type != EntryType::Dir)
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
            is_link: entry.path_is_symlink(),
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
        let (conditions, matches, encoding) = match &query.filter {
            None => (Vec::new(), None, None),
            Some(filter) => self.evaluate(filter, file, language, entry.path(), rel)?,
        };
        let is_file = entry_type == EntryType::File;
        if !self.readable(entry.path(), rel, entry_type) {
            return None;
        }
        if !query.needs.kind && is_file && self.kinds.is_generated(&path, entry.path()) {
            return None;
        }
        Some(Found {
            rel: rel.to_path_buf(),
            is_dir: entry_type == EntryType::Dir,
            conditions,
            matches,
            encoding,
            id: entry.metadata().ok().and_then(|m| file_id(&m)),
        })
    }

    /// The conditions which made the file match and its content fragments, or `None` if it doesn't match. The file
    /// is read only when the conditions without the content don't decide; a binary file's content is unknown.
    fn evaluate(
        &self,
        filter: &Filter,
        file: FileValues<'_>,
        language: Option<&Language>,
        path: &Path,
        rel: &Path,
    ) -> Option<(Vec<CondId>, Option<FileMatches>, Option<Encoding>)> {
        if filter.eval(&file) == Truth::False {
            return None;
        }
        let content: Content;
        let mut encoding = None;
        // a copy with a shorter lifetime, which can borrow the content read here
        let mut file = file;
        if matches!(file.content, ContentState::NotRead) {
            self.slow_read();
            let bytes = std::fs::read(path);
            if self.past_deadline() {
                return None;
            }
            let bytes = match bytes {
                Ok(bytes) => bytes,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        self.warn(WarningCode::PermissionDenied, rel, false);
                    }
                    return None;
                }
            };
            match decode(&bytes) {
                Decoded::Binary => {
                    file.content = ContentState::Unknown;
                    self.warn(WarningCode::BinarySkipped, rel, false);
                }
                Decoded::Text {
                    content: decoded,
                    encoding: used,
                    errors,
                } => {
                    if errors {
                        self.warn(WarningCode::DecodeErrors, rel, false);
                    }
                    encoding = (used != Encoding::Utf8).then_some(used);
                    content = match language {
                        Some(language) if self.query.needs.comments => {
                            decoded.with_comments(language)
                        }
                        _ => decoded,
                    };
                    file.content = ContentState::Text(&content);
                }
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
        Some((conditions, matches, encoding))
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

    /// Whether a matched entry can be read before it is returned: a file opened, a directory listed. A file which
    /// can't be opened gets `permission_denied`; the walker reports a directory it can't list.
    fn readable(&self, path: &Path, rel: &Path, entry_type: EntryType) -> bool {
        match entry_type {
            EntryType::File => match std::fs::File::open(path) {
                Ok(_) => true,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        self.warn(WarningCode::PermissionDenied, rel, false);
                    }
                    false
                }
            },
            EntryType::Dir => std::fs::read_dir(path).is_ok(),
            EntryType::Link => true,
        }
    }

    /// An error of the walker: a directory or file it can't read gets `permission_denied`.
    fn walk_error(&self, err: &ignore::Error) {
        if err
            .io_error()
            .is_some_and(|e| e.kind() == std::io::ErrorKind::PermissionDenied)
            && let Some(path) = error_path(err)
            && let Ok(rel) = path.strip_prefix(self.root)
        {
            self.warn(WarningCode::PermissionDenied, rel, path.is_dir());
        }
    }

    /// Whether the deadline passed; it is remembered for the result.
    fn past_deadline(&self) -> bool {
        let past = self
            .options
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline);
        if past {
            self.timed_out.store(true, AtomicOrdering::Relaxed);
        }
        past
    }

    /// The test hook `read_delay`: sleeps before a read, at most until the deadline.
    fn slow_read(&self) {
        let Some(delay) = self.options.read_delay else {
            return;
        };
        let until = Instant::now() + delay;
        let until = self.options.deadline.map_or(until, |d| until.min(d));
        if let Some(rest) = until.checked_duration_since(Instant::now()) {
            std::thread::sleep(rest);
        }
    }

    fn warn(&self, code: WarningCode, rel: &Path, is_dir: bool) {
        self.warnings
            .lock()
            .expect("no thread panics while holding the lock")
            .add(code, rel.to_path_buf(), is_dir);
    }
}

/// The path of a walker error, through its wrappers.
fn error_path(err: &ignore::Error) -> Option<&Path> {
    match err {
        ignore::Error::WithPath { path, .. } => Some(path),
        ignore::Error::WithDepth { err, .. } | ignore::Error::WithLineNumber { err, .. } => {
            error_path(err)
        }
        ignore::Error::Partial(errs) if errs.len() == 1 => error_path(&errs[0]),
        _ => None,
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
#[allow(clippy::unnecessary_wraps)] // the signature of every platform: `None` where there is no identity
fn file_id(metadata: &std::fs::Metadata) -> Option<FileId> {
    use std::os::unix::fs::MetadataExt;
    Some((metadata.dev(), metadata.ino()))
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
    fn walker_error_paths() {
        let io = || std::io::Error::from(std::io::ErrorKind::PermissionDenied);
        let nested = ignore::Error::WithDepth {
            depth: 1,
            err: Box::new(ignore::Error::WithPath {
                path: PathBuf::from("/r/a"),
                err: Box::new(ignore::Error::Io(io())),
            }),
        };
        assert_eq!(error_path(&nested), Some(Path::new("/r/a")));
        assert_eq!(error_path(&ignore::Error::Io(io())), None);
    }

    #[test]
    fn slash_paths() {
        let rel: PathBuf = ["src", "parser", "lexer.rs"].iter().collect();
        assert_eq!(slash_path(&rel), "src/parser/lexer.rs");
        assert_eq!(slash_path(Path::new("a.rs")), "a.rs");
    }
}
