//! The git status of files (QUERY.md#git), read by the `git` program once per repository.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// A value of the `git` target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitValue {
    Modified,
    Staged,
    Untracked,
    Conflicted,
    Changed,
    Branch,
    Clean,
}

impl GitValue {
    /// All values in the order of the spec.
    pub const ALL: [GitValue; 7] = [
        Self::Modified,
        Self::Staged,
        Self::Untracked,
        Self::Conflicted,
        Self::Changed,
        Self::Branch,
        Self::Clean,
    ];

    /// The value as written in a query.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::Staged => "staged",
            Self::Untracked => "untracked",
            Self::Conflicted => "conflicted",
            Self::Changed => "changed",
            Self::Branch => "branch",
            Self::Clean => "clean",
        }
    }

    /// The value written as `word`, in any case.
    pub fn from_keyword(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|v| v.keyword().eq_ignore_ascii_case(word))
    }
}

/// The statuses of one file in its repository; none of them — the file is clean.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // independent statuses, a file can have several
pub struct GitStatus {
    pub modified: bool,
    pub staged: bool,
    pub untracked: bool,
    pub conflicted: bool,
}

impl GitStatus {
    /// Whether the file has the status `value`.
    pub fn has(self, value: GitValue) -> bool {
        let changed = self.modified || self.staged || self.untracked || self.conflicted;
        match value {
            GitValue::Modified => self.modified,
            GitValue::Staged => self.staged,
            GitValue::Untracked => self.untracked,
            GitValue::Conflicted => self.conflicted,
            GitValue::Changed | GitValue::Branch => changed,
            GitValue::Clean => !changed,
        }
    }
}

/// A repository: its statuses by `/`-separated path relative to the root, and its tracked files.
#[derive(Debug, Default)]
struct Repo {
    root: PathBuf,
    statuses: HashMap<String, GitStatus>,
    tracked: HashSet<String>,
}

/// The repositories found during one search, cached per directory.
#[derive(Debug, Default)]
pub struct GitIndex {
    /// Without the global and system git configuration (tests).
    isolated: bool,
    by_dir: Mutex<HashMap<PathBuf, Option<Arc<Repo>>>>,
}

impl GitIndex {
    /// An empty index; `isolated` runs git without the global and system configuration.
    pub fn new(isolated: bool) -> Self {
        Self {
            isolated,
            by_dir: Mutex::default(),
        }
    }

    /// The status of the file at the absolute `path`, `None` outside of a repository. A file which is neither
    /// tracked nor listed by `git status` (e.g. ignored) has no status either.
    pub fn status(&self, path: &Path) -> Option<GitStatus> {
        let dir = path.parent()?;
        let repo = {
            let mut cache = self
                .by_dir
                .lock()
                .expect("no thread panics while holding the lock");
            if let Some(repo) = cache.get(dir) {
                repo.clone()
            } else {
                let repo = find_root(path).and_then(|root| {
                    let known = cache.values().flatten().find(|r| r.root == root).cloned();
                    known.or_else(|| self.read(&root).map(Arc::new))
                });
                cache.insert(dir.to_path_buf(), repo.clone());
                repo
            }
        }?;
        let rel = path.strip_prefix(&repo.root).ok()?;
        let key = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        match repo.statuses.get(&key) {
            Some(status) => Some(*status),
            None if repo.tracked.contains(&key) => Some(GitStatus::default()),
            None => None,
        }
    }

    /// Whether the file at `path` is in a repository which git could read.
    pub fn in_repo(&self, path: &Path) -> bool {
        let Some(dir) = path.parent() else {
            return false;
        };
        let _ = self.status(path);
        self.by_dir
            .lock()
            .expect("no thread panics while holding the lock")
            .get(dir)
            .is_some_and(Option::is_some)
    }

    fn git(&self, root: &Path, args: &[&str]) -> Option<Vec<u8>> {
        let mut command = Command::new("git");
        command
            .arg("-C")
            .arg(root)
            .args(["-c", "core.fsmonitor=false", "-c", "core.quotepath=off"])
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null());
        if self.isolated {
            command
                .env(
                    "GIT_CONFIG_GLOBAL",
                    if cfg!(windows) { "NUL" } else { "/dev/null" },
                )
                .env("GIT_CONFIG_NOSYSTEM", "1");
        }
        let output = command.output().ok()?;
        output.status.success().then_some(output.stdout)
    }

    fn read(&self, root: &Path) -> Option<Repo> {
        let status = self.git(
            root,
            &[
                "status",
                "--porcelain=v2",
                "-z",
                "--untracked-files=all",
                "--no-renames",
            ],
        )?;
        let tracked = self.git(root, &["ls-files", "-z"])?;
        Some(Repo {
            root: root.to_path_buf(),
            statuses: parse_status(&status),
            tracked: tracked
                .split(|b| *b == 0)
                .filter(|p| !p.is_empty())
                .map(|p| String::from_utf8_lossy(p).into_owned())
                .collect(),
        })
    }
}

/// The root of the repository of the file at `path`: the nearest directory upwards with a `.git`.
fn find_root(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .skip(1)
        .find(|dir| dir.join(".git").exists())
        .map(Path::to_path_buf)
}

/// The statuses of `git status --porcelain=v2 -z` by path.
fn parse_status(output: &[u8]) -> HashMap<String, GitStatus> {
    let mut statuses = HashMap::new();
    for record in output.split(|b| *b == 0).filter(|r| !r.is_empty()) {
        let record = String::from_utf8_lossy(record);
        let (kind, rest) = record.split_at(1);
        let rest = rest.trim_start_matches(' ');
        let (path, status) = match kind {
            "1" | "2" => {
                let fields: Vec<&str> = rest.splitn(8, ' ').collect();
                let xy = fields[0].as_bytes();
                let status = GitStatus {
                    staged: xy.first() != Some(&b'.'),
                    modified: xy.get(1) != Some(&b'.'),
                    ..GitStatus::default()
                };
                (fields.get(7).copied(), status)
            }
            "u" => (
                rest.splitn(10, ' ').nth(9),
                GitStatus {
                    conflicted: true,
                    ..GitStatus::default()
                },
            ),
            "?" => (
                Some(rest),
                GitStatus {
                    untracked: true,
                    ..GitStatus::default()
                },
            ),
            _ => (None, GitStatus::default()),
        };
        if let Some(path) = path {
            statuses.insert(path.to_string(), status);
        }
    }
    statuses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn porcelain_v2_records() {
        let out = "1 .M N... 100644 100644 100644 a a modified.txt\0\
                   1 M. N... 100644 100644 100644 a b staged file.txt\0\
                   1 MM N... 100644 100644 100644 a b sub/both.txt\0\
                   u UU N... 100644 100644 100644 100644 a b c conflict.txt\0\
                   ? untracked.txt\0\
                   ! ignored.txt\0";
        let statuses = parse_status(out.as_bytes());
        assert_eq!(
            statuses["modified.txt"],
            GitStatus {
                modified: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["staged file.txt"],
            GitStatus {
                staged: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["sub/both.txt"],
            GitStatus {
                modified: true,
                staged: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["conflict.txt"],
            GitStatus {
                conflicted: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["untracked.txt"],
            GitStatus {
                untracked: true,
                ..GitStatus::default()
            }
        );
        assert!(!statuses.contains_key("ignored.txt"));
    }

    #[test]
    fn values() {
        let both = GitStatus {
            modified: true,
            staged: true,
            ..GitStatus::default()
        };
        assert!(
            both.has(GitValue::Modified)
                && both.has(GitValue::Staged)
                && both.has(GitValue::Changed)
        );
        assert!(!both.has(GitValue::Clean) && !both.has(GitValue::Untracked));
        let clean = GitStatus::default();
        assert!(clean.has(GitValue::Clean) && !clean.has(GitValue::Changed));
        assert_eq!(GitValue::from_keyword("CHANGED"), Some(GitValue::Changed));
        assert_eq!(GitValue::from_keyword("dirty"), None);
    }

    #[test]
    fn repositories_are_found_per_file() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("outer/.git")).unwrap();
        std::fs::create_dir_all(root.join("outer/inner/src")).unwrap();
        std::fs::write(root.join("outer/inner/.git"), "gitdir: x\n").unwrap();
        assert_eq!(find_root(&root.join("outer/a")), Some(root.join("outer")));
        assert_eq!(
            find_root(&root.join("outer/inner/src/b")),
            Some(root.join("outer/inner"))
        );
        assert_eq!(
            find_root(&root.join("c")).filter(|r| r.starts_with(root)),
            None
        );
    }
}
