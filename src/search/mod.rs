//! Walks the working directory and collects the files which match the query (CONFIG.md#traversal).

mod kinds;

use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};

use ignore::{WalkBuilder, WalkState};

pub use kinds::{
    DEFAULT_CONFIG, DEFAULT_DOC, DEFAULT_GENERATED, DEFAULT_MARKER_LINES, DEFAULT_MARKERS,
    DEFAULT_TEST, KindOptions, KindRules,
};

use crate::archive::{ArchiveKind, Member, for_each_member};
use crate::compressed::{Compression, Decompressed, decompress};
use crate::content::{Content, DecodeOptions, Decoded, Encoding, decode, is_binary_content};
use crate::git::{GitIndex, GitStatus};
use crate::languages::{Language, LanguageTable};
use crate::output::{Fragment, matched_lines, merge_identical};
use crate::query::UNKNOWN_LANG;
use crate::query::{
    ContentState, EntryType, FileValues, Filter, FunnelKind, Kind, Query, Truth, ast::CondId, nfc,
};
use crate::relevance::{self, Signals};
use crate::warning::{WarningCode, Warnings};

/// How far the running searches of a call are (TOOLS.md#progress): the entries evaluated and the ones matched.
#[derive(Debug, Default)]
pub struct Progress {
    scanned: AtomicUsize,
    matched: AtomicUsize,
}

impl Progress {
    /// The entries whose conditions were evaluated so far, matched or not.
    #[must_use]
    pub fn scanned(&self) -> usize {
        self.scanned.load(AtomicOrdering::Relaxed)
    }

    /// The entries matched so far.
    #[must_use]
    pub fn matched(&self) -> usize {
        self.matched.load(AtomicOrdering::Relaxed)
    }
}

/// Settings of one search.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // independent switches of the walk, from the configuration
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
    /// The ignore files looked up in every directory; a later name wins at the same level.
    pub ignore_files: Vec<String>,
    /// Respect `.git/info/exclude` and the global git excludes file.
    pub git_excludes: bool,
    /// Ignore files applied as if they were in the working directory, with the lowest priority; a later one wins.
    pub ignore_paths: Vec<PathBuf>,
    /// Wildcards of the paths which are never walked, compiled like `path ~`.
    pub exclude: Vec<regex::Regex>,
    /// What happens to binary files.
    pub binary: Binary,
    /// How the content of files is decoded; `binary_as_text` follows `binary`.
    pub decode: DecodeOptions,
    /// Which compressed files and archives are searched inside.
    pub archives: Archives,
    /// The local time zone: the dates of zip members without one.
    pub time_zone: jiff::tz::TimeZone,
    /// Bigger files are not searched by content, in bytes.
    pub max_file_size: f64,
    /// The total uncompressed size of the members of one archive, in bytes.
    pub max_archive_size: f64,
    /// Skip generated files unless the query has a `kind` condition.
    pub skip_generated: bool,
    /// The rules of the kinds.
    pub kinds: KindOptions,
    /// The base of `git = branch`; `None` — `auto`.
    pub git_base: Option<String>,
    /// Stop at the first matched entry (`detail = exists`).
    pub first_only: bool,
    /// Compute the fragments of the `name` and `path` conditions (`output.path_fragments`).
    pub path_fragments: bool,
    /// Give every matched entry its metadata (`output.meta`).
    pub meta: bool,
    /// Give every matched entry the signals of its relevance (`sort = relevance`).
    pub rank: bool,
    /// Count the files of the steps of the funnel (OUTPUT.md#funnel).
    pub funnel: bool,
    /// Read the git status of every matched file, for the `git` facet.
    pub git_status: bool,
    /// Counts the scanned and matched entries, for the progress notifications.
    pub progress: Option<std::sync::Arc<Progress>>,
    /// The canonical allowed roots: a followed symlink must point inside them.
    pub allowed_roots: std::sync::Arc<Vec<PathBuf>>,
    /// A test hook: every read of a file's content takes this long, cut short at the deadline.
    pub read_delay: Option<Duration>,
}

/// What happens to binary files (`traversal.binary`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    /// Not searched by content: content conditions are unknown for them.
    Skip,
    /// Searched as text.
    Search,
    /// Never returned.
    Exclude,
}

/// Which compressed files and archives are searched inside (`traversal.archives`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Archives {
    /// Nothing: they are binary files.
    None,
    /// Single compressed files, as their decompressed content.
    Compressed,
    /// Compressed files and the members of archives.
    All,
}

/// A matched file.
#[derive(Debug, Clone, PartialEq)]
pub struct Found {
    /// The path relative to the working directory; of a member of an archive the path of the archive.
    pub rel: PathBuf,
    /// The path of a member inside its archive, as stored; `None` for an entry of the file system.
    pub member: Option<String>,
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
    /// The fragments of the `name` and `path` conditions, with `path_fragments`.
    pub path_fragments: Option<PathFragments>,
    /// The metadata, with `meta`.
    pub meta: Option<Meta>,
    /// The signals of the relevance, with `sort = relevance`.
    pub signals: Option<Signals>,
    /// The relevance score, once the files are ranked.
    pub score: Option<f64>,
    /// The git status, with `git_status`; `None` outside of a repository.
    pub git: Option<GitStatus>,
    /// The index of its working directory in the request.
    pub root: usize,
}

/// The metadata of a matched entry (OUTPUT.md#meta).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    /// The type; a symlink is `Link`, followed or not.
    pub entry_type: EntryType,
    /// Of a file: its size in bytes.
    pub size: Option<u64>,
    /// Of a file: its language, `unknown` without one.
    pub lang: Option<String>,
    /// Of a file: its kind.
    pub kind: Option<Kind>,
    /// The modification date in whole seconds since the Unix epoch.
    pub modified: Option<i64>,
}

impl Found {
    /// An entry known only by its path (and member): a removed file of `since`.
    #[must_use]
    pub fn path_only(rel: PathBuf, member: Option<String>) -> Self {
        Self {
            rel,
            member,
            is_dir: false,
            conditions: Vec::new(),
            matches: None,
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
            git: None,
            root: 0,
        }
    }
}

/// The fragments of the `name` and `path` conditions which made an entry match (OUTPUT.md#path-fragments).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathFragments {
    /// The path as shown, without the trailing separator of a directory.
    pub path: String,
    /// Byte ranges in `path`, sorted, identical ones merged.
    pub fragments: Vec<Fragment>,
}

impl PathFragments {
    /// The fragments of the contributing conditions in the NFC `path` (relative to the working directory, `/` as
    /// the separator) of an entry named `name`; `None` without any.
    fn of(leaves: &[&Filter], name: &str, path: &str) -> Option<Self> {
        let fragments: Vec<Fragment> = leaves
            .iter()
            .flat_map(|leaf| {
                let ids: Vec<(usize, CondId)> = leaf
                    .id()
                    .map(|(order, id)| (order, id.clone()))
                    .into_iter()
                    .collect();
                leaf.path_ranges(name, path)
                    .into_iter()
                    .map(move |range| Fragment {
                        range,
                        ids: ids.clone(),
                        escape: None,
                    })
            })
            .collect();
        (!fragments.is_empty()).then(|| Self {
            path: path.to_string(),
            fragments: merge_identical(fragments),
        })
    }

    /// The fragments of an entry found in the working directory `prefix`: the path as shown, with the directory before
    /// it and native separators (the path of a member of an archive keeps `/` after `!/`).
    #[must_use]
    pub fn shown(mut self, prefix: &Path, member: bool) -> Self {
        let archive_end = if member {
            self.path.find("!/").unwrap_or(self.path.len())
        } else {
            self.path.len()
        };
        let native: String = self.path[..archive_end]
            .chars()
            .map(|c| {
                if c == '/' {
                    std::path::MAIN_SEPARATOR
                } else {
                    c
                }
            })
            .collect();
        self.path.replace_range(..archive_end, &native);
        let dir = crate::output::display_path(prefix, false);
        if !dir.is_empty() {
            let shift = dir.len() + std::path::MAIN_SEPARATOR.len_utf8();
            self.path = format!("{dir}{}{}", std::path::MAIN_SEPARATOR, self.path);
            for f in &mut self.fragments {
                f.range = f.range.start + shift..f.range.end + shift;
            }
        }
        self
    }
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
    /// The number of files of every step of the funnel, with `funnel`.
    pub funnel: Vec<usize>,
}

/// A device and inode pair which identifies a file reachable by several hard links.
pub type FileId = (u64, u64);

/// Symlinks which point outside of the allowed roots, found by the walk: the path relative to the working directory
/// and whether the target is a directory.
type Outside = std::sync::Arc<Mutex<Vec<(PathBuf, bool)>>>;

/// The walker of `root` with the walk options: ignore files, hidden entries, depth, symlinks and exclusions.
fn walk_builder(root: &Path, options: &SearchOptions) -> (WalkBuilder, Outside) {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(!options.hidden)
        .parents(true)
        .ignore(false)
        .git_ignore(false)
        .git_global(options.git_excludes && options.git_global_excludes)
        .git_exclude(options.git_excludes)
        .require_git(false)
        .follow_links(options.follow_symlinks)
        .threads(options.threads)
        .max_depth(options.max_depth);
    let outside: Outside = std::sync::Arc::default();
    if options.follow_symlinks || !options.exclude.is_empty() {
        let roots = std::sync::Arc::clone(&options.allowed_roots);
        let outside = std::sync::Arc::clone(&outside);
        let base = root.to_path_buf();
        let follow = options.follow_symlinks;
        let exclude = options.exclude.clone();
        builder.filter_entry(move |entry| {
            if is_excluded(&exclude, &base, entry) {
                return false;
            }
            if !follow || !entry.path_is_symlink() {
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
    for name in &options.ignore_files {
        builder.add_custom_ignore_filename(name);
    }
    builder.current_dir(root);
    for path in options.ignore_paths.iter().filter(|p| p.is_file()) {
        let _ = builder.add_ignore(path);
    }
    (builder, outside)
}

/// Searches `root` and returns the matched entries in path order, every file once; without a `type` condition only
/// regular files.
pub fn search(root: &Path, query: &Query, options: &SearchOptions) -> SearchResult {
    let (builder, outside) = walk_builder(root, options);
    let walk = Walk::new(root, query, options);
    builder.build_parallel().run(|| {
        let walk = &walk;
        Box::new(move |entry| {
            if walk.past_deadline() || walk.done.load(AtomicOrdering::Relaxed) {
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
                    if walk.options.archives == Archives::All {
                        walk.visit_members(&entry, None);
                    }
                    if walk.options.first_only
                        && !walk
                            .found
                            .lock()
                            .expect("no thread panics while holding the lock")
                            .is_empty()
                    {
                        walk.done.store(true, AtomicOrdering::Relaxed);
                        return WalkState::Quit;
                    }
                }
                Err(err) => walk.walk_error(&err),
            }
            WalkState::Continue
        })
    });
    let outside: Vec<(PathBuf, bool)> = outside
        .lock()
        .expect("no thread panics while holding the lock")
        .drain(..)
        .collect();
    walk.finish(outside)
}

/// Searches the files `files` of the working directory `root` (paths in it and the members of archives) without a
/// walk (`within`): a file which doesn't exist any more is skipped; the matched entries in path order.
pub fn search_files(
    root: &Path,
    files: &[(PathBuf, Option<String>)],
    query: &Query,
    options: &SearchOptions,
) -> SearchResult {
    let walk = Walk::new(root, query, options);
    let mut members: Vec<(&PathBuf, Vec<String>)> = Vec::new();
    for (rel, member) in files {
        if let Some(member) = member {
            match members.iter_mut().find(|(r, _)| *r == rel) {
                Some((_, list)) => list.push(member.clone()),
                None => members.push((rel, vec![member.clone()])),
            }
            continue;
        }
        if walk.past_deadline() {
            break;
        }
        let Some(entry) = FileEntry::new(root, rel, options.follow_symlinks) else {
            continue;
        };
        if let Some(item) = walk.visit(&entry) {
            walk.found
                .lock()
                .expect("no thread panics while holding the lock")
                .push(item);
        }
    }
    for (rel, list) in members {
        if let Some(entry) = FileEntry::new(root, rel, options.follow_symlinks)
            && !walk.past_deadline()
        {
            walk.visit_members(&entry, Some(&list));
        }
    }
    walk.finish(Vec::new())
}

impl<'a> Walk<'a> {
    /// The state of a search of `root`.
    fn new(root: &'a Path, query: &'a Query, options: &'a SearchOptions) -> Self {
        Walk {
            root,
            query,
            options,
            kinds: KindRules::new(&options.kinds, options.case_insensitive_paths),
            git: GitIndex::new(!options.git_global_excludes, options.git_base.clone()),
            found: Mutex::default(),
            warnings: Mutex::default(),
            timed_out: AtomicBool::new(false),
            done: AtomicBool::new(false),
            funnel: (0..query.funnel.as_ref().map_or(0, |f| f.steps.len()))
                .map(|_| AtomicUsize::new(0))
                .collect(),
        }
    }

    /// The result: the found entries in path order, every file once, the warnings with `permission_denied` for the
    /// entries `outside` of the allowed roots.
    fn finish(self, outside: Vec<(PathBuf, bool)>) -> SearchResult {
        let mut found = self.found.into_inner().expect("the walk is finished");
        let mut warnings = self.warnings.into_inner().expect("the walk is finished");
        for (rel, is_dir) in outside {
            warnings.add(WarningCode::PermissionDenied, rel, is_dir);
        }
        found.sort_by(|a, b| (&a.rel, &a.member).cmp(&(&b.rel, &b.member)));
        let mut seen = std::collections::HashSet::new();
        let files = found
            .into_iter()
            .filter(|f| f.id.is_none_or(|id| seen.insert((id, f.member.clone()))))
            .collect();
        SearchResult {
            files,
            warnings,
            timed_out: self.timed_out.into_inner(),
            funnel: self
                .funnel
                .into_iter()
                .map(AtomicUsize::into_inner)
                .collect(),
        }
    }
}

/// What the visit of an entry needs of it: an entry of the walk, or a file of a kept result (`within`).
trait Entry {
    fn path(&self) -> &Path;
    /// The number of the path components below the working directory: 0 for the working directory.
    fn depth(&self) -> usize;
    /// The type: of the target of a followed symlink.
    fn file_type(&self) -> Option<std::fs::FileType>;
    fn file_name(&self) -> &std::ffi::OsStr;
    /// The metadata: of the target of a followed symlink.
    fn metadata(&self) -> Option<std::fs::Metadata>;
    fn path_is_symlink(&self) -> bool;
}

impl Entry for ignore::DirEntry {
    fn path(&self) -> &Path {
        ignore::DirEntry::path(self)
    }

    fn depth(&self) -> usize {
        ignore::DirEntry::depth(self)
    }

    fn file_type(&self) -> Option<std::fs::FileType> {
        ignore::DirEntry::file_type(self)
    }

    fn file_name(&self) -> &std::ffi::OsStr {
        ignore::DirEntry::file_name(self)
    }

    fn metadata(&self) -> Option<std::fs::Metadata> {
        ignore::DirEntry::metadata(self).ok()
    }

    fn path_is_symlink(&self) -> bool {
        ignore::DirEntry::path_is_symlink(self)
    }
}

/// A file of a kept result, visited without a walk.
struct FileEntry {
    path: PathBuf,
    depth: usize,
    /// Of the target of a followed symlink.
    metadata: std::fs::Metadata,
    symlink: bool,
}

impl FileEntry {
    /// The file `rel` of the working directory `root`; `None` when it doesn't exist any more.
    fn new(root: &Path, rel: &Path, follow_symlinks: bool) -> Option<Self> {
        let path = root.join(rel);
        let link = std::fs::symlink_metadata(&path).ok()?;
        let symlink = link.file_type().is_symlink();
        let metadata = if symlink && follow_symlinks {
            std::fs::metadata(&path).ok()?
        } else {
            link
        };
        Some(Self {
            depth: rel.components().count(),
            path,
            metadata,
            symlink,
        })
    }
}

impl Entry for FileEntry {
    fn path(&self) -> &Path {
        &self.path
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn file_type(&self) -> Option<std::fs::FileType> {
        Some(self.metadata.file_type())
    }

    fn file_name(&self) -> &std::ffi::OsStr {
        self.path.file_name().unwrap_or_default()
    }

    fn metadata(&self) -> Option<std::fs::Metadata> {
        Some(self.metadata.clone())
    }

    fn path_is_symlink(&self) -> bool {
        self.symlink
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
    /// The files of every step of the funnel.
    funnel: Vec<AtomicUsize>,
    /// Set when the deadline passed.
    timed_out: AtomicBool,
    /// Set when the walk can stop: `first_only` and an entry was found.
    done: AtomicBool,
}

/// The content of a file read for its content conditions.
enum ReadContent {
    Bytes(Vec<u8>),
    /// Not searched: too big or a broken compressed file; the warning is given.
    Unknown,
    /// Not readable, or the deadline passed: the file is skipped.
    Unreadable,
}

/// What the evaluation of the query found out about a matched entry.
#[derive(Debug, Default)]
struct Evaluated {
    /// The ids of the conditions which made the entry match.
    conditions: Vec<CondId>,
    /// The content fragments, if any content condition made the entry match.
    matches: Option<FileMatches>,
    /// The encoding of the content, if it was read and is not UTF-8.
    encoding: Option<Encoding>,
    /// Whether the content was read and decoded as text.
    text: bool,
    /// The fragments of the `name` and `path` conditions, with `path_fragments`.
    path_fragments: Option<PathFragments>,
    /// The signals of the relevance, with `rank`.
    signals: Option<Signals>,
    /// Its point in the funnel, see `Walk::funnel_point`.
    funnel: usize,
}

/// What the visit of an entry already knows for its metadata.
struct Known<'a> {
    metadata: Option<&'a std::fs::Metadata>,
    /// The language, if it was detected (`None` also when it is unknown).
    language: Option<&'a Language>,
    /// The kind, if it was classified.
    kind: Option<Kind>,
}

/// The dates of an entry in seconds since the Unix epoch.
struct Dates {
    modified: Option<f64>,
    created: Option<f64>,
    accessed: Option<f64>,
}

impl Walk<'_> {
    /// The matched entry and its hard link identity, or `None` if the entry doesn't match or is skipped.
    fn visit(&self, entry: &dyn Entry) -> Option<Found> {
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
            .then(|| entry.metadata())
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
        let evaluated = match &query.filter {
            None => Evaluated::default(),
            Some(filter) => self.evaluate(
                filter,
                file,
                language,
                &mut || self.read_content(entry.path(), rel, &name),
                &|code| self.warn(code, rel, false),
            )?,
        };
        if self.left_out(entry, rel, entry_type, &name, &path, evaluated.text) {
            return None;
        }
        self.count_funnel(evaluated.funnel);
        self.count_match();
        let meta = self.options.meta.then(|| {
            let known = Known {
                metadata: metadata.as_ref(),
                language: language.filter(|_| query.needs.lang || query.needs.comments),
                kind: query.needs.kind.then_some(kind),
            };
            self.meta(entry, entry_type, &name, &path, &known)
        });
        Some(Found {
            rel: rel.to_path_buf(),
            member: None,
            is_dir: entry_type == EntryType::Dir,
            conditions: evaluated.conditions,
            matches: evaluated.matches,
            encoding: evaluated.encoding,
            id: entry.metadata().and_then(|m| file_id(&m)),
            path_fragments: evaluated.path_fragments,
            meta,
            signals: self.signals(evaluated.signals, &name, &nfc_path),
            score: None,
            git: if self.options.git_status && entry_type == EntryType::File {
                self.git.status(entry.path())
            } else {
                None
            },
            root: 0,
        })
    }

    /// The metadata of a matched entry of the file system, from what the visit knows and what is missing.
    fn meta(
        &self,
        entry: &dyn Entry,
        entry_type: EntryType,
        name: &str,
        path: &str,
        known: &Known<'_>,
    ) -> Meta {
        let read = known.metadata.is_none().then(|| entry.metadata()).flatten();
        let metadata = known.metadata.or(read.as_ref());
        let is_file = entry_type == EntryType::File;
        let language = if is_file {
            known.language.or_else(|| {
                self.options
                    .languages
                    .detect(name, || first_line(entry.path()))
            })
        } else {
            None
        };
        Meta {
            entry_type: if entry.path_is_symlink() {
                EntryType::Link
            } else {
                entry_type
            },
            size: metadata.filter(|_| is_file).map(std::fs::Metadata::len),
            lang: is_file.then(|| {
                language
                    .map_or(UNKNOWN_LANG, |l| l.name.as_str())
                    .to_string()
            }),
            kind: is_file.then(|| {
                known.kind.unwrap_or_else(|| {
                    self.kinds.classify(
                        path,
                        entry.path(),
                        true,
                        language.is_some_and(|l| l.source),
                    )
                })
            }),
            modified: metadata
                .and_then(|m| m.modified().ok())
                .map(|t| whole_seconds(system_seconds(t))),
        }
    }

    /// Whether a matched entry is left out after all: unreadable, generated (with `skip_generated`), or binary with
    /// `binary = exclude` (`text`: its content was decoded as text).
    fn left_out(
        &self,
        entry: &dyn Entry,
        rel: &Path,
        entry_type: EntryType,
        name: &str,
        path: &str,
        text: bool,
    ) -> bool {
        let is_file = entry_type == EntryType::File;
        !self.readable(entry.path(), rel, entry_type)
            || (self.skips_generated() && is_file && self.kinds.is_generated(path, entry.path()))
            || (self.options.binary == Binary::Exclude
                && is_file
                && !text
                && self.starts_binary(entry.path(), name))
    }

    /// The signals of the relevance of a matched entry: those of its evaluation, or for the empty query those of its
    /// path.
    fn signals(&self, evaluated: Option<Signals>, name: &str, path: &str) -> Option<Signals> {
        if !self.options.rank {
            return None;
        }
        evaluated.or_else(|| {
            Some(relevance::signals(
                &self.query.ranking,
                &[],
                name,
                path,
                None,
            ))
        })
    }

    /// The metadata of a matched member of an archive: a file with the size and the date stored in the archive.
    fn member_meta(
        &self,
        member: &Member,
        path: &str,
        language: Option<&Language>,
        kind: Option<Kind>,
    ) -> Meta {
        let language = language.or_else(|| {
            let name = path.rsplit('/').next().unwrap_or(path);
            self.options.languages.detect(name, || None)
        });
        Meta {
            entry_type: EntryType::File,
            size: Some(member.size),
            lang: Some(
                language
                    .map_or(UNKNOWN_LANG, |l| l.name.as_str())
                    .to_string(),
            ),
            kind: Some(kind.unwrap_or_else(|| {
                self.kinds.classify(
                    path,
                    Path::new(""),
                    false,
                    language.is_some_and(|l| l.source),
                )
            })),
            modified: member.modified.map(whole_seconds),
        }
    }

    /// The conditions which made the file match and its content fragments, or `None` if it doesn't match. The file
    /// is read only when the conditions without the content don't decide; a binary file's content is unknown.
    /// `read` reads the content when it is needed, `warn` gives a warning about the entry.
    fn evaluate(
        &self,
        filter: &Filter,
        file: FileValues<'_>,
        language: Option<&Language>,
        read: &mut dyn FnMut() -> ReadContent,
        warn: &dyn Fn(WarningCode),
    ) -> Option<Evaluated> {
        if let Some(progress) = &self.options.progress {
            progress.scanned.fetch_add(1, AtomicOrdering::Relaxed);
        }
        if filter.eval(&file) == Truth::False {
            self.count_funnel(self.funnel_point(filter, &file, false));
            return None;
        }
        let content: Content;
        let mut encoding = None;
        let mut text = false;
        // a copy with a shorter lifetime, which can borrow the content read here
        let mut file = file;
        if matches!(file.content, ContentState::NotRead) {
            let bytes = match read() {
                ReadContent::Bytes(bytes) => Some(bytes),
                ReadContent::Unknown => None,
                ReadContent::Unreadable => return None,
            };
            if let Some(bytes) = bytes {
                match decode(&bytes, &self.options.decode) {
                    Decoded::Binary if self.options.binary == Binary::Exclude => return None,
                    Decoded::Binary => {
                        file.content = ContentState::Unknown;
                        warn(WarningCode::BinarySkipped);
                    }
                    Decoded::Text {
                        content: decoded,
                        encoding: used,
                        errors,
                    } => {
                        if errors {
                            warn(WarningCode::DecodeErrors);
                        }
                        encoding = (used != Encoding::Utf8).then_some(used);
                        text = true;
                        content = match language {
                            Some(language) if self.query.needs.comments => {
                                decoded.with_comments(language)
                            }
                            _ => decoded,
                        };
                        file.content = ContentState::Text(&content);
                    }
                }
            } else {
                file.content = ContentState::Unknown;
            }
        }
        if !filter.matches(&file) {
            self.count_funnel(self.funnel_point(filter, &file, false));
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
        let path_fragments = if self.options.path_fragments {
            PathFragments::of(&leaves, file.name, file.path)
        } else {
            None
        };
        let signals = self.options.rank.then(|| {
            let content = match file.content {
                ContentState::Text(content) => Some((
                    content.line_count(),
                    matches.as_ref().map_or(0, |m| m.fragments.len()),
                )),
                _ => None,
            };
            relevance::signals(&self.query.ranking, &leaves, file.name, file.path, content)
        });
        Some(Evaluated {
            conditions,
            matches,
            encoding,
            text,
            path_fragments,
            signals,
            funnel: self.funnel_point(filter, &file, true),
        })
    }

    /// The point of an entry in the funnel: for `and` the number of the leading steps it passes (a matched entry
    /// passes all; the last step isn't evaluated for one which doesn't match, it can't pass it), for `or` the number of
    /// its first true branch from 1, for one step 1 when it matches; 0 without a funnel. Unknown doesn't pass.
    fn funnel_point(&self, filter: &Filter, file: &FileValues<'_>, matched: bool) -> usize {
        let Some(funnel) = self.query.funnel.as_ref().filter(|_| self.options.funnel) else {
            return 0;
        };
        let ((FunnelKind::And, Filter::And(items)) | (FunnelKind::Or, Filter::Or(items))) =
            (funnel.kind, filter)
        else {
            return usize::from(matched);
        };
        let passes = |item: usize| items[item].eval(file) == Truth::True;
        if funnel.kind == FunnelKind::Or {
            return funnel
                .steps
                .iter()
                .position(|step| passes(step.item))
                .map_or(0, |n| n + 1);
        }
        if matched {
            return funnel.steps.len();
        }
        let last = funnel.steps.len() - 1;
        funnel.steps[..last]
            .iter()
            .take_while(|step| passes(step.item))
            .count()
    }

    /// Counts a matched entry for the progress.
    fn count_match(&self) {
        if let Some(progress) = &self.options.progress {
            progress.matched.fetch_add(1, AtomicOrdering::Relaxed);
        }
    }

    /// Adds an entry at its point to the numbers of the steps of the funnel.
    fn count_funnel(&self, point: usize) {
        let Some(funnel) = &self.query.funnel else {
            return;
        };
        let steps = if funnel.kind == FunnelKind::Or {
            point.checked_sub(1).map_or(0..0, |n| n..n + 1)
        } else {
            0..point
        };
        for step in &self.funnel[steps] {
            step.fetch_add(1, AtomicOrdering::Relaxed);
        }
    }

    /// Visits the members of an archive with `archives = all` (`only` those, when given): the matched members go to
    /// the found entries; an archive which can't be read gets `archive_error` (`permission_denied` when it can't be
    /// opened).
    fn visit_members(&self, entry: &dyn Entry, only: Option<&[String]>) {
        if entry.depth() == 0 || entry_type(entry) != Some(EntryType::File) {
            return;
        }
        let Some(kind) = ArchiveKind::of(&entry.file_name().to_string_lossy()) else {
            return;
        };
        let Ok(rel) = entry.path().strip_prefix(self.root) else {
            return;
        };
        let archive = slash_path(rel);
        let id = entry.metadata().and_then(|m| file_id(&m));
        let mut found = Vec::new();
        let mut total = 0_u64;
        let mut too_big = false;
        let result = for_each_member(
            entry.path(),
            kind,
            &self.options.time_zone,
            &mut |member, reader| {
                if self.past_deadline() {
                    return false;
                }
                total = total.saturating_add(member.size);
                #[allow(clippy::cast_precision_loss)]
                // sizes far below 2^53 are exact, bigger ones surely too big
                if total as f64 > self.options.max_archive_size {
                    too_big = true;
                    return false;
                }
                if only.is_some_and(|only| !only.contains(&member.path)) {
                    return true;
                }
                if let Some(mut item) = self.visit_member(rel, &archive, member, reader) {
                    item.id = id;
                    found.push(item);
                }
                true
            },
        );
        if too_big {
            self.warn(WarningCode::ArchiveTooBig, rel, false);
        }
        if let Err(e) = result {
            let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                WarningCode::PermissionDenied
            } else {
                WarningCode::ArchiveError
            };
            self.warn(code, rel, false);
        }
        self.found
            .lock()
            .expect("no thread panics while holding the lock")
            .extend(found);
    }

    /// The matched member of an archive, or `None`. A member is a regular file with the path
    /// `<archive>!/<member>`, its name, size and modification date from the archive, no creation or access date, no
    /// git status; nested archives are not opened.
    fn visit_member(
        &self,
        rel: &Path,
        archive: &str,
        member: &Member,
        reader: &mut dyn std::io::Read,
    ) -> Option<Found> {
        use std::io::Read;
        let query = self.query;
        let path = format!("{archive}!/{}", member.path);
        let nfc_path = nfc(&path);
        let name = nfc(member.path.rsplit('/').next().unwrap_or(&member.path));
        let warn = |code| self.warn_member(code, rel, &member.path);
        if query.needs.created || query.needs.accessed {
            warn(WarningCode::DateUnsupported);
        }
        let language = (query.needs.lang || query.needs.comments)
            .then(|| self.options.languages.detect(&name, || None))
            .flatten();
        let kind = if query.needs.kind {
            self.kinds.classify(
                &path,
                Path::new(""),
                false,
                language.is_some_and(|l| l.source),
            )
        } else {
            Kind::Other
        };
        let file = FileValues {
            name: &name,
            path: &nfc_path,
            entry_type: EntryType::File,
            is_link: false,
            empty: member.size == 0,
            size: member.size,
            modified: member.modified,
            created: None,
            accessed: None,
            lang: language.map_or(UNKNOWN_LANG, |l| l.name.as_str()),
            kind,
            git: None,
            content: if query.needs.content {
                ContentState::NotRead
            } else {
                ContentState::Absent
            },
        };
        let limit = self.size_limit();
        let mut read = || {
            if member.size > limit {
                warn(WarningCode::TooBig);
                return ReadContent::Unknown;
            }
            let mut bytes = Vec::new();
            match reader.take(limit.saturating_add(1)).read_to_end(&mut bytes) {
                Ok(_) if bytes.len() as u64 > limit => {
                    warn(WarningCode::TooBig);
                    ReadContent::Unknown
                }
                Ok(_) => ReadContent::Bytes(bytes),
                Err(_) => {
                    self.warn(WarningCode::ArchiveError, rel, false);
                    ReadContent::Unknown
                }
            }
        };
        let evaluated = match &query.filter {
            None => Evaluated::default(),
            Some(filter) => self.evaluate(filter, file, language, &mut read, &warn)?,
        };
        if self.skips_generated() && self.kinds.is_generated_path(&path) {
            return None;
        }
        if self.options.binary == Binary::Exclude && !evaluated.text {
            let mut head = Vec::new();
            if reader.take(8 * 1024).read_to_end(&mut head).is_ok()
                && is_binary_content(&head, &self.options.decode)
            {
                return None;
            }
        }
        self.count_funnel(evaluated.funnel);
        self.count_match();
        Some(Found {
            rel: rel.to_path_buf(),
            member: Some(member.path.clone()),
            is_dir: false,
            conditions: evaluated.conditions,
            matches: evaluated.matches,
            encoding: evaluated.encoding,
            id: None,
            path_fragments: evaluated.path_fragments,
            meta: self.options.meta.then(|| {
                self.member_meta(member, &path, language, query.needs.kind.then_some(kind))
            }),
            signals: self.signals(evaluated.signals, &name, &nfc_path),
            score: None,
            git: None,
            root: 0,
        })
    }

    /// The content of a file whose content conditions must be decided: the bytes, decompressed for a compressed file
    /// with `archives`; `Unknown` (with its warning) for a file too big or a broken compressed one; `Unreadable` when
    /// it can't be read or the deadline passed.
    fn read_content(&self, path: &Path, rel: &Path, name: &str) -> ReadContent {
        let compression = if self.options.archives == Archives::None {
            None
        } else {
            Compression::of(name)
        };
        if compression.is_none() && self.too_big(path) {
            self.warn(WarningCode::TooBig, rel, false);
            return ReadContent::Unknown;
        }
        self.slow_read();
        let bytes = std::fs::read(path);
        if self.past_deadline() {
            return ReadContent::Unreadable;
        }
        let bytes = match bytes {
            Ok(bytes) => bytes,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    self.warn(WarningCode::PermissionDenied, rel, false);
                }
                return ReadContent::Unreadable;
            }
        };
        let Some(compression) = compression else {
            return ReadContent::Bytes(bytes);
        };
        match decompress(compression, bytes.as_slice(), self.size_limit()) {
            Ok(Decompressed::Bytes(bytes)) => ReadContent::Bytes(bytes),
            Ok(Decompressed::TooBig) => {
                self.warn(WarningCode::TooBig, rel, false);
                ReadContent::Unknown
            }
            Err(_) => {
                self.warn(WarningCode::ArchiveError, rel, false);
                ReadContent::Unknown
            }
        }
    }

    /// Whether a file is binary by its first 8K, decompressed for a compressed file with `archives`; a file which
    /// can't be read or decompressed is not.
    fn starts_binary(&self, path: &Path, name: &str) -> bool {
        use std::io::Read;
        let Ok(file) = std::fs::File::open(path) else {
            return false;
        };
        let compression = if self.options.archives == Archives::None {
            None
        } else {
            Compression::of(name)
        };
        let head = if let Some(compression) = compression {
            crate::compressed::head(compression, file, 8 * 1024)
        } else {
            let mut head = Vec::new();
            file.take(8 * 1024).read_to_end(&mut head).map(|_| head)
        };
        head.is_ok_and(|head| is_binary_content(&head, &self.options.decode))
    }

    /// `max_file_size` in whole bytes: a content longer than this is too big.
    fn size_limit(&self) -> u64 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        // a non-negative size, saturated at the limits of u64
        let limit = self.options.max_file_size.floor() as u64;
        limit
    }

    /// Whether a file is bigger than `max_file_size`; a file without metadata is not.
    fn too_big(&self, path: &Path) -> bool {
        #[allow(clippy::cast_precision_loss)]
        // sizes far below 2^53 are exact, bigger ones are surely too big
        std::fs::metadata(path).is_ok_and(|m| m.len() as f64 > self.options.max_file_size)
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
        if self.git.base_missing(path) {
            self.warn(WarningCode::GitBaseNotFound, rel, false);
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

    /// Whether generated files are skipped: `skip_generated` and no `kind` condition in the query.
    fn skips_generated(&self) -> bool {
        self.options.skip_generated && !self.query.needs.kind
    }

    fn warn_member(&self, code: WarningCode, rel: &Path, member: &str) {
        self.warnings
            .lock()
            .expect("no thread panics while holding the lock")
            .add_member(code, rel.to_path_buf(), member.to_string());
    }

    fn warn(&self, code: WarningCode, rel: &Path, is_dir: bool) {
        self.warnings
            .lock()
            .expect("no thread panics while holding the lock")
            .add(code, rel.to_path_buf(), is_dir);
    }
}

/// Whether an entry under `base` is excluded: a wildcard matches its relative `/` path, or for a directory the path
/// followed by `/`.
fn is_excluded(exclude: &[regex::Regex], base: &Path, entry: &ignore::DirEntry) -> bool {
    if exclude.is_empty() {
        return false;
    }
    let Ok(rel) = entry.path().strip_prefix(base) else {
        return false;
    };
    if rel.as_os_str().is_empty() {
        return false;
    }
    let path = nfc(&slash_path(rel));
    let is_dir = entry.file_type().is_some_and(|t| t.is_dir());
    excluded(exclude, &path, is_dir)
}

/// Whether a relative `/` path is excluded; a directory also by its path followed by `/`.
fn excluded(exclude: &[regex::Regex], path: &str, is_dir: bool) -> bool {
    exclude
        .iter()
        .any(|r| r.is_match(path) || (is_dir && r.is_match(&format!("{path}/"))))
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
fn entry_type(entry: &dyn Entry) -> Option<EntryType> {
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

/// Seconds since the Unix epoch without the fraction, rounded down.
#[allow(clippy::cast_possible_truncation)] // dates are far from the limits of i64
fn whole_seconds(seconds: f64) -> i64 {
    seconds.floor() as i64
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
    fn path_fragments_after_the_working_directory() {
        let fragments = PathFragments {
            path: "src/a.zip!/x/a.rs".into(),
            fragments: vec![Fragment {
                range: 4..5,
                ids: vec![],
                escape: None,
            }],
        };
        let sep = std::path::MAIN_SEPARATOR;
        let shown = fragments.clone().shown(Path::new("../api"), true);
        assert_eq!(shown.path, format!("..{sep}api{sep}src{sep}a.zip!/x/a.rs"));
        assert_eq!(shown.fragments[0].range, 11..12);
        let here = fragments.shown(Path::new("."), true);
        assert_eq!(here.path, format!("src{sep}a.zip!/x/a.rs"));
        assert_eq!(here.fragments[0].range, 4..5);
    }

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
    fn excluded_paths_and_directories() {
        let compile = |w: &str| crate::query::wildcard::compile(w, false).unwrap();
        let every = [compile("**/target/**")];
        assert!(excluded(&every, "target", true));
        assert!(excluded(&every, "a/target", true));
        assert!(excluded(&every, "a/target/x.rs", false));
        assert!(!excluded(&every, "target", false));
        assert!(!excluded(&every, "targets", true));
        let top = [compile("target/**")];
        assert!(excluded(&top, "target", true));
        assert!(!excluded(&top, "a/target", true));
        let file = [compile("*.lock")];
        assert!(excluded(&file, "Cargo.lock", false));
        assert!(!excluded(&file, "a/Cargo.lock", false));
    }

    #[test]
    fn slash_paths() {
        let rel: PathBuf = ["src", "parser", "lexer.rs"].iter().collect();
        assert_eq!(slash_path(&rel), "src/parser/lexer.rs");
        assert_eq!(slash_path(Path::new("a.rs")), "a.rs");
    }
}
