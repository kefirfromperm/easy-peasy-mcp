//! The HTTP server.

pub mod handlers;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

/// The shared state of the handlers.
pub struct State {
    pub files: HashMap<u64, crate::walk::File>,
    pub index: crate::eval::Evaluator,
    pub counter: Mutex<u64>,
}

impl State {
    pub fn new(root: &Path) -> std::io::Result<State> {
        let paths = crate::walk::files(root)?;
        let names: Vec<String> = paths.iter().map(|p| p.to_str().unwrap().to_string()).collect();
        let files = paths.iter().enumerate().map(|(i, p)| (i as u64, crate::walk::File::read(p).unwrap())).collect();
        Ok(State { files, index: crate::eval::Evaluator::new(&names), counter: Mutex::new(0) })
    }

    /// Counts a request.
    pub fn hit(&self) {
        *self.counter.lock().unwrap() += 1;
    }
}
