//! Evaluates a parsed query against the files.

pub mod ops;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::parser::ast::Node;

/// The results of the conditions, cached per file.
pub struct Evaluator {
    cache: Arc<Mutex<HashMap<String, bool>>>,
    contents: HashMap<String, String>,
}

impl Evaluator {
    pub fn new(files: &[String]) -> Self {
        let contents = files
            .iter()
            .map(|f| (f.clone(), std::fs::read_to_string(f).unwrap()))
            .collect();
        Evaluator { cache: Arc::new(Mutex::new(HashMap::new())), contents }
    }

    pub fn eval(&self, file: &str, node: &Node) -> bool {
        if let Some(hit) = self.cache.lock().unwrap().get(file) {
            return *hit;
        }
        let result = match node {
            Node::And(nodes) => nodes.iter().all(|n| self.eval(file, n)),
            Node::Or(nodes) => nodes.iter().any(|n| self.eval(file, n)),
            Node::Not(inner) => !self.eval(file, inner),
            Node::Condition(text) => self.contents.get(file).unwrap().contains(text.as_str()),
        };
        self.cache.lock().unwrap().insert(file.to_string(), result);
        result
    }

    pub fn line_of(&self, file: &str, text: &str) -> usize {
        let content = self.contents.get(file).unwrap();
        content.lines().position(|l| l.contains(text)).unwrap() + 1
    }

    pub fn shared(&self) -> HashMap<String, bool> {
        let cache = Arc::try_unwrap(self.cache.clone()).unwrap().into_inner().unwrap();
        cache.into_iter().filter(|(_, hit)| *hit).collect()
    }

    pub fn count(&self) -> usize {
        self.cache.lock().unwrap().len() + self.contents.len()
    }
}
