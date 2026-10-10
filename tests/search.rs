//! Search cases which can't live in a fixture in git: hard links, decomposed Unicode names.

use std::fs;

use easy_peasy_mcp::{Engine, EngineOptions};
use serde_json::json;

fn engine(root: &std::path::Path) -> Engine {
    let mut options = EngineOptions::new(root.to_path_buf());
    options.git_global_excludes = false;
    Engine::new(options)
}

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
        format!(
            "files: 1  lines: 0  matches: 0  result: r1\n#1 name = 'data.txt'\n\na{}\ndata.txt  [#1]\n",
            std::path::MAIN_SEPARATOR
        )
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
        format!(
            "files: 1  lines: 1  matches: 1  result: r2\n#1 text has 'retry'\n\n.{}\na.txt  [#1]  lines: 1  matches: 1\n1:⟦#1:retry⟧\n",
            std::path::MAIN_SEPARATOR
        )
    );
}

#[test]
fn since_sees_changed_and_deleted_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::write(root.join("a.txt"), "retry\n").unwrap();
    fs::write(root.join("b.txt"), "retry\n").unwrap();
    fs::write(root.join("c.txt"), "retry\n").unwrap();
    let engine = engine(root);
    let first = engine.find(&json!({ "query": "text has 'retry'" }));
    assert!(first.text.starts_with("files: 3  "), "{}", first.text);
    fs::write(root.join("a.txt"), "retry again\n").unwrap();
    fs::remove_file(root.join("b.txt")).unwrap();
    fs::write(root.join("d.txt"), "retry\n").unwrap();
    let since = engine.find(&json!({
        "since": "r1",
        "query": "text has 'retry'",
        "output": { "sort": "path asc", "detail": "files" }
    }));
    assert_eq!(
        since.text,
        "files: 3  lines: 3  matches: 3  result: r2  since: r1  unchanged: 1\n#1 text has 'retry'\n\n~ a.txt  [#1]  lines: 1  matches: 1\n+ d.txt  [#1]  lines: 1  matches: 1\n- b.txt\n"
    );
}

#[test]
fn a_cancelled_search_stops_soon() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    for i in 0..20 {
        fs::write(root.join(format!("{i:02}.txt")), "x\n").unwrap();
    }
    let mut options = EngineOptions::new(root.to_path_buf());
    options.git_global_excludes = false;
    options.read_delay = Some(std::time::Duration::from_secs(1));
    let engine = std::sync::Arc::new(Engine::new(options));
    // one search thread: twenty reads of a second take twenty seconds
    let update = engine
        .call("update_config", &json!({ "set": "server.threads = 1" }))
        .unwrap();
    assert!(!update.is_error, "{}", update.text);
    let progress = std::sync::Arc::new(easy_peasy_mcp::Progress::default());
    let cancel = std::sync::Arc::clone(&progress);
    let started = std::time::Instant::now();
    let canceller = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        cancel.cancel();
    });
    let _ = engine.call_with("find", &json!({ "query": "text has 'x'" }), Some(&progress));
    canceller.join().unwrap();
    assert!(
        started.elapsed() < std::time::Duration::from_secs(3),
        "twenty reads of a second take far longer: {:?}",
        started.elapsed()
    );
}

#[test]
fn native_path_case_follows_the_option() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::write(root.join("README.md"), "x\n").unwrap();
    let count = |insensitive: bool| {
        let mut options = EngineOptions::new(root.to_path_buf());
        options.git_global_excludes = false;
        options.native_case_insensitive = insensitive;
        let out = Engine::new(options).find(
            &json!({ "query": "name = 'readme.md'", "output": { "detail": "count", "funnel": "never" } }),
        );
        assert!(!out.is_error, "{}", out.text);
        out.text
    };
    assert!(count(true).starts_with("files: 1  "), "{}", count(true));
    assert!(count(false).starts_with("files: 0  "), "{}", count(false));
}

#[test]
fn native_path_case_is_the_platform_convention_by_default() {
    let options = EngineOptions::new(std::path::PathBuf::from("."));
    assert_eq!(
        options.native_case_insensitive,
        cfg!(any(target_os = "macos", target_os = "windows"))
    );
}

#[test]
fn a_working_directory_in_the_home_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir(root.join("projects")).unwrap();
    fs::write(root.join("projects/a.txt"), "x\n").unwrap();
    let mut options = EngineOptions::new(root.to_path_buf());
    options.git_global_excludes = false;
    options.home = Some(root.to_path_buf());
    let out =
        Engine::new(options).find(&json!({ "dirs": ["~/projects"], "query": "name = 'a.txt'" }));
    assert!(!out.is_error, "{}", out.text);
    assert!(out.text.starts_with("files: 1  "), "{}", out.text);
}
