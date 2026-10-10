//! A small cache of computed values, cleared when the configuration changes.
//!
//! The values are cloned out of the cache, so a caller never holds a borrow of it.

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// A cached value and when it was stored.
struct Entry<V> {
    value: V,
    stored: Instant,
}

/// Values by key, each kept for `ttl`.
pub struct Cache<K, V> {
    entries: HashMap<K, Entry<V>>,
    ttl: Duration,
}

impl<K: Hash + Eq + Clone, V: Clone> Cache<K, V> {
    /// An empty cache whose values are kept for `ttl`.
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
        }
    }

    /// Stores `value` under `key`, replacing an older one.
    pub fn insert(&mut self, key: K, value: V) {
        let entry = Entry {
            value,
            stored: Instant::now(),
        };
        self.entries.insert(key, entry);
    }

    /// The value of `key` if it is not older than the ttl.
    pub fn get(&mut self, key: &K) -> Option<V> {
        let fresh = self
            .entries
            .get(key)
            .filter(|e| e.stored.elapsed() < self.ttl)
            .map(|e| e.value.clone());
        fresh
    }

    /// The values of `keys`, computing the missing ones with `compute`.
    pub fn get_all(&mut self, keys: &[K], compute: impl Fn(&K) -> V) -> HashMap<K, V> {
        let mut map = HashMap::new();
        for k in keys {
            let value = self.get(k).unwrap_or_else(|| compute(k));
            map.insert(k.clone(), value);
        }
        map
    }

    /// The value of the first key, computed if it is missing.
    pub fn first(&mut self, keys: &[K], compute: impl Fn(&K) -> V) -> V {
        let map = self.get_all(keys, compute);
        let k = &keys[0];
        let v = map.get(k).unwrap();
        v.clone()
    }
}
