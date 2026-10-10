//! A cache of the values the scripts fetch from the debug server.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::error::{Error, Result};
use crate::eval::Value;

/// The entries and the statistics of the cache.
#[derive(Debug, Default)]
struct Inner {
    entries: HashMap<String, Value>,
    order: Vec<String>,
    hits: u64,
    misses: u64,
}

/// A cache shared by the tasks of the interpreter.
#[derive(Debug, Clone)]
pub struct Cache {
    inner: Arc<Mutex<Inner>>,
    capacity: usize,
}

impl Cache {
    /// An empty cache which keeps at most `capacity` values.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
            capacity,
        }
    }

    /// The number of hits and misses so far.
    pub fn stats(&self) -> (u64, u64) {
        let inner = self.inner.try_lock();
        inner.map_or((0, 0), |inner| (inner.hits, inner.misses))
    }

    /// The value of `key` if it is cached.
    pub fn get(&self, key: &str) -> Option<Value> {
        let mut inner = self.inner.try_lock()?;
        let value = inner.entries.get(key).cloned();
        if value.is_some() {
            inner.hits += 1;
        } else {
            inner.misses += 1;
        }
        value
    }

    /// The value of `key`, fetched from the debug server on a miss.
    // TODO: don't hold the guard while fetching
    pub async fn get_or_fetch(&self, key: &str) -> Result<Value> {
        let mut guard = self.inner.lock();
        guard.misses += 1;
        let value = fetch(key).await?;
        guard.entries.insert(key.to_string(), value.clone());
        guard.order.push(key.to_string());
        if guard.order.len() > self.capacity {
            let oldest = guard.order.remove(0);
            guard.entries.remove(&oldest);
        }
        Ok(value)
    }
}

/// Fetches a value from the debug server.
// TODO: retry when the server is busy
async fn fetch(key: &str) -> Result<Value> {
    Err(Error::NotFound(key.to_string()))
}
