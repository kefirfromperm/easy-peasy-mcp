//! The trash of the system, where `delete` moves entries (TOOLS.md#delete).

use std::path::{Path, PathBuf};

/// Where `delete` moves entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Trash {
    /// The trash of the system: the freedesktop trash, the macOS Trash, the Windows Recycle Bin.
    #[default]
    System,
    /// A test hook: the entries go into this directory, at their paths in their allowed root.
    #[doc(hidden)]
    Dir(PathBuf),
    /// A test hook: a file system without a trash.
    #[doc(hidden)]
    None,
}

/// Why an entry wasn't moved to the trash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrashError {
    /// There is no trash for the entry.
    Unavailable,
    /// The entry can't be moved: the kind of the error.
    Failed(String),
}

impl Trash {
    /// Moves the entry at `path` (its directory canonical) to the trash; `roots` are the allowed roots.
    pub(crate) fn put(&self, path: &Path, roots: &[PathBuf]) -> Result<(), TrashError> {
        match self {
            Self::System => ::trash::delete(path).map_err(|e| system_error(&e, path)),
            Self::Dir(dir) => {
                let rel = roots
                    .iter()
                    .find_map(|root| path.strip_prefix(root).ok())
                    .unwrap_or(path);
                let mut target = dir.join(rel);
                let mut n = 1;
                while target.symlink_metadata().is_ok() {
                    n += 1;
                    target = dir.join(format!("{}.{n}", rel.to_string_lossy()));
                }
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| failed(&e))?;
                }
                std::fs::rename(path, &target).map_err(|e| failed(&e))
            }
            Self::None => Err(TrashError::Unavailable),
        }
    }
}

/// An error of an operation on an entry: the kind of the error of the operating system.
pub(crate) fn failed(error: &std::io::Error) -> TrashError {
    TrashError::Failed(error.kind().to_string())
}

/// The error of the trash of the system: an error of the entry itself fails, the others (no trash on its file system,
/// no home directory) mean the trash is unavailable.
fn system_error(error: &::trash::Error, path: &Path) -> TrashError {
    match error {
        #[cfg(all(
            unix,
            not(target_os = "macos"),
            not(target_os = "ios"),
            not(target_os = "android")
        ))]
        ::trash::Error::FileSystem {
            path: failed_path,
            source,
        } if failed_path == path => failed(source),
        ::trash::Error::Os { description, .. } => TrashError::Failed(description.clone()),
        _ => {
            let _ = path;
            TrashError::Unavailable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_test_hook_keeps_the_paths_in_the_root() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("root");
        let trash = tmp.path().join("trash");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/a.rs"), "a").unwrap();
        let hook = Trash::Dir(trash.clone());
        hook.put(&root.join("src/a.rs"), std::slice::from_ref(&root))
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(trash.join("src/a.rs")).unwrap(),
            "a"
        );
        // the same path again: a numbered name
        std::fs::write(root.join("src/a.rs"), "b").unwrap();
        hook.put(&root.join("src/a.rs"), std::slice::from_ref(&root))
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(trash.join("src/a.rs.2")).unwrap(),
            "b"
        );
        assert_eq!(
            hook.put(&root.join("src/missing.rs"), std::slice::from_ref(&root)),
            Err(TrashError::Failed(std::io::ErrorKind::NotFound.to_string()))
        );
        assert_eq!(
            Trash::None.put(&root.join("src"), &[root]),
            Err(TrashError::Unavailable)
        );
    }
}
