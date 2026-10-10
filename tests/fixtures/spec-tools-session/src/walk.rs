//! Lists the files of a directory tree.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// All files below `root`, in path order.
pub fn files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.file_type().unwrap().is_dir() {
                dirs.push(entry.path());
            } else {
                found.push(entry.path());
            }
        }
    }
    found.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(found)
}
