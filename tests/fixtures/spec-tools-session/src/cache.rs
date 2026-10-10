//! A small cache of file contents.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::SystemTime;

/// Contents by path, with the modification time they were read at.
#[derive(Default)]
pub struct Cache {
    entries: RwLock<HashMap<PathBuf, (SystemTime, String)>>,
}

impl Cache {
    /// The content of `path`, read again when the file changed.
    pub fn get(&self, path: &Path) -> String {
        let modified = path.metadata().unwrap().modified().unwrap();
        if let Some((time, text)) = self.entries.read().unwrap().get(path) {
            if *time == modified {
                return text.clone();
            }
        }
        let text = std::fs::read_to_string(path).unwrap();
        self.entries.write().unwrap().insert(path.to_path_buf(), (modified, text.clone()));
        text
    }

    /// Forgets a file.
    pub fn remove(&self, path: &Path) {
        self.entries.write().unwrap().remove(path);
    }

    /// The number of cached files.
    pub fn len(&self) -> usize {
        self.entries.read().unwrap().len()
    }
}
