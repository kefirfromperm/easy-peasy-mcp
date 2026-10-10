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
    assert!(
        second.text.contains("result: r2\n"),
        "errors take no result id: {}",
        second.text
    );
}

#[test]
fn results_dont_depend_on_the_threads() {
    let tmp = tempfile::tempdir().unwrap();
    for d in 0..8 {
        let dir = tmp.path().join(format!("d{d}"));
        fs::create_dir(&dir).unwrap();
        for f in 0..25 {
            fs::write(
                dir.join(format!("f{f}.txt")),
                format!("line {f}\nneedle {d}\n"),
            )
            .unwrap();
        }
    }
    let engine = engine(tmp.path());
    let query = json!({ "query": "text has 'needle' or name ~ 'f1*'" });
    let one = |threads: u32| {
        let set = format!("server.threads = {threads}");
        let update = engine
            .call("update_config", &json!({ "set": set }))
            .unwrap();
        assert!(!update.is_error, "{}", update.text);
        let out = engine.find(&query);
        assert!(!out.is_error, "{}", out.text);
        // the result id is the only difference
        out.text.split_once('\n').unwrap().1.to_string()
    };
    let single = one(1);
    assert_eq!(one(4), single);
    assert_eq!(one(0), single);
}

#[test]
fn within_skips_files_deleted_since_the_result() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::write(root.join("a.txt"), "retry\n").unwrap();
    fs::write(root.join("b.txt"), "retry\n").unwrap();
    let engine = engine(root);
    let first = engine.find(&json!({ "query": "name ~ '*.txt'" }));
    assert!(first.text.starts_with("files: 2  "), "{}", first.text);
    fs::remove_file(root.join("b.txt")).unwrap();
    fs::write(root.join("c.txt"), "retry\n").unwrap();
    let within = engine.find(&json!({ "within": "r1", "query": "text has 'retry'" }));
    assert_eq!(
        within.text,
        "files: 1  lines: 1  matches: 1  result: r2\n#1 text has 'retry'\n\n./\na.txt  [#1]  lines: 1  matches: 1\n1:⟦#1:retry⟧\n"
    );
}
