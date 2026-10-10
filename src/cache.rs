//! The results kept for `within` and `since` (OUTPUT.md#result-ids).

use std::collections::VecDeque;
use std::path::PathBuf;

use serde_json::Value;

/// A matched file of a kept result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedFile {
    /// The path as in the response.
    pub path: PathBuf,
    /// A hash of the matched lines.
    pub fingerprint: u64,
}

/// A kept result.
#[derive(Debug, Clone, PartialEq)]
// read by `within` and `since` (tasks 111–112)
#[allow(dead_code)]
pub struct CachedResult {
    pub id: u64,
    /// The arguments of the request.
    pub arguments: Value,
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
            files,
        });
        while self.results.len() > capacity {
            self.results.pop_front();
        }
        Some(self.last_id)
    }

    /// A kept result by its id.
    #[cfg(test)]
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
        assert_eq!(cache.keep(2, json!({ "query": "a" }), Vec::new()), Some(1));
        assert_eq!(cache.keep(2, json!({ "query": "b" }), Vec::new()), Some(2));
        assert_eq!(cache.keep(2, json!({ "query": "c" }), Vec::new()), Some(3));
        assert!(cache.get(1).is_none());
        assert_eq!(cache.get(3).unwrap().arguments, json!({ "query": "c" }));
        // off: no id, nothing kept, the ids go on afterwards
        assert_eq!(cache.keep(0, json!({}), Vec::new()), None);
        assert_eq!(cache.keep(1, json!({ "query": "d" }), Vec::new()), Some(4));
        assert!(cache.get(2).is_none());
        assert!(cache.get(3).is_none());
        assert!(cache.get(4).is_some());
    }
}
