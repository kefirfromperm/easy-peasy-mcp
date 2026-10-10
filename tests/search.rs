//! Search cases which can't live in a fixture in git: hard links, decomposed Unicode names.

use std::fs;

use easy_peasy_mcp::{Engine, EngineOptions};
use serde_json::json;

fn engine(root: &std::path::Path) -> Engine {
    let mut options = EngineOptions::new(root.to_path_buf());
    options.git_global_excludes = false;
    Engine::new(options)
}

#[cfg(unix)]
#[test]
fn hard_links_are_returned_once_under_the_first_path() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir(root.join("a")).unwrap();
    fs::create_dir(root.join("b")).unwrap();
    fs::write(root.join("b/data.txt"), "x\n").unwrap();
    fs::hard_link(root.join("b/data.txt"), root.join("a/data.txt")).unwrap();
    let out = engine(root).find(&json!({ "query": "name = 'data.txt'" }));
    assert_eq!(
        out.text,
        "files: 1  lines: 0  matches: 0  result: r1\n#1 name = 'data.txt'\n\na/\ndata.txt  [#1]\n"
    );
}

#[test]
fn decomposed_names_match_composed_queries() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let decomposed = "cafe\u{301}.txt";
    fs::write(root.join(decomposed), "x\n").unwrap();
    let out = engine(root).find(&json!({ "query": "name = 'caf\u{e9}.txt'" }));
    assert!(!out.is_error, "{}", out.text);
    assert!(out.text.starts_with("files: 1  "), "{}", out.text);
}

#[test]
fn result_ids_grow_per_session() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("a.txt"), "x\n").unwrap();
    let engine = engine(tmp.path());
    let first = engine.find(&json!({ "query": "name = 'a.txt'" }));
    let error = engine.find(&json!({ "query": "name =" }));
    let second = engine.find(&json!({ "query": "name = 'a.txt'" }));
    assert!(first.text.contains("result: r1\n"));
    assert!(error.is_error);
    assert!(second.text.contains("result: r2\n"), "errors take no result id: {}", second.text);
}
