//! The results kept for `within` and `since` (OUTPUT.md#result-ids).

use std::collections::VecDeque;
use std::path::PathBuf;

use serde_json::Value;

/// A matched file of a kept result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedFile {
    /// The index of its working directory.
    pub root: usize,
    /// The path in its working directory.
    pub rel: PathBuf,
    /// The path of a member of an archive.
    pub member: Option<String>,
    /// A hash of the matched lines.
    pub fingerprint: u64,
}

/// A kept result.
#[derive(Debug, Clone, PartialEq)]
pub struct CachedResult {
    pub id: u64,
    /// The arguments of the request.
    #[allow(dead_code)] // read by `since` (task 112)
    pub arguments: Value,
    /// The working directories: the canonical directory and the prefix of the paths.
    pub working_dirs: Vec<(PathBuf, PathBuf)>,
    /// All matched files, not only the shown ones.
    pub files: Vec<CachedFile>,
}

/// The last results of a session.
#[derive(Debug, Default)]
pub struct ResultCache {
    results: VecDeque<CachedResult>,
    last_id: u64,
}

impl ResultCache {
    /// Keeps a result among the last `capacity` ones and returns its id; `None` when the cache is off.
    pub fn keep(
        &mut self,
        capacity: usize,
        arguments: Value,
        working_dirs: Vec<(PathBuf, PathBuf)>,
        files: Vec<CachedFile>,
    ) -> Option<u64> {
        if capacity == 0 {
            self.results.clear();
            return None;
        }
        self.last_id += 1;
        self.results.push_back(CachedResult {
            id: self.last_id,
            arguments,
            working_dirs,
            files,
        });
        while self.results.len() > capacity {
            self.results.pop_front();
        }
        Some(self.last_id)
    }

    /// A kept result by its id.
    pub fn get(&self, id: u64) -> Option<&CachedResult> {
        self.results.iter().find(|r| r.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ids_and_eviction() {
        let mut cache = ResultCache::default();
        assert_eq!(
            cache.keep(2, json!({ "query": "a" }), Vec::new(), Vec::new()),
            Some(1)
        );
        assert_eq!(
            cache.keep(2, json!({ "query": "b" }), Vec::new(), Vec::new()),
            Some(2)
        );
        assert_eq!(
            cache.keep(2, json!({ "query": "c" }), Vec::new(), Vec::new()),
            Some(3)
        );
        assert!(cache.get(1).is_none());
        assert_eq!(cache.get(3).unwrap().arguments, json!({ "query": "c" }));
        // off: no id, nothing kept, the ids go on afterwards
        assert_eq!(cache.keep(0, json!({}), Vec::new(), Vec::new()), None);
        assert_eq!(
            cache.keep(1, json!({ "query": "d" }), Vec::new(), Vec::new()),
            Some(4)
        );
        assert!(cache.get(2).is_none());
        assert!(cache.get(3).is_none());
        assert!(cache.get(4).is_some());
    }
}
