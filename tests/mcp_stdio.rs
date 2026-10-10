//! End to end: spawn the server binary and talk JSON-RPC over its stdin and stdout.

mod common;

use std::process::Command;

use common::{BIN, McpServer, server_command};
use serde_json::json;

fn fixture() -> common::FixtureDir {
    let (_, fixtures) = common::load_examples();
    let mvp = fixtures
        .iter()
        .find(|f| f.name == "mvp")
        .expect("fixture mvp");
    common::setup_fixture(mvp)
}

#[test]
fn initialize_list_and_find() {
    let dir = fixture();
    let mut server = McpServer::spawn(&mut server_command(&dir.root, dir.temp()));

    let init = server.initialize();
    let result = &init["result"];
    assert_eq!(result["serverInfo"]["name"], "easy-peasy-mcp");
    assert_eq!(result["serverInfo"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(
        result["capabilities"]["tools"],
        json!({ "listChanged": false })
    );
    assert_eq!(
        result["instructions"],
        "File search: find + grep in one query. Call get_config first to learn allowed roots and defaults. Spec version 1."
    );

    let list = server.request(2, "tools/list", &json!({}));
    let tools = list["result"]["tools"].as_array().expect("tools");
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["find", "get_config", "update_config"]);
    let update = &tools[2];
    assert_eq!(update["annotations"]["readOnlyHint"], false);
    assert_eq!(update["annotations"]["destructiveHint"], false);
    assert_eq!(update["annotations"]["idempotentHint"], true);
    assert_eq!(update["annotations"]["openWorldHint"], false);
    assert_eq!(
        update["inputSchema"]["properties"]["reset"]["type"],
        "array"
    );
    let get_config = &tools[1];
    assert_eq!(get_config["annotations"]["readOnlyHint"], true);
    assert_eq!(get_config["annotations"]["openWorldHint"], false);
    assert_eq!(get_config["inputSchema"]["additionalProperties"], false);
    assert_eq!(
        get_config["description"],
        "Returns the effective configuration of the search server as TOML: allowed roots, default directories, traversal and\noutput defaults, limits, supported spec versions. Read-only. Call it once before the first find."
    );
    let find = &tools[0];
    assert_eq!(find["annotations"]["readOnlyHint"], true);
    assert_eq!(find["annotations"]["openWorldHint"], false);
    assert_eq!(find["inputSchema"]["type"], "object");
    assert_eq!(find["inputSchema"]["properties"]["query"]["type"], "string");
    let description = find["description"].as_str().unwrap();
    assert!(description.ends_with("Spec version 1."), "{description}");

    let call = server.request(
        3,
        "tools/call",
        &json!({ "name": "find", "arguments": { "query": "name = 'Cargo.toml'" } }),
    );
    assert_eq!(call["result"]["isError"], false);
    assert_eq!(
        call["result"]["content"],
        json!([{ "type": "text", "text": "files: 1  lines: 0  matches: 0  result: r1\n#1 name = 'Cargo.toml'\n\n./\nCargo.toml  [#1]\n" }])
    );

    let error = server.request(
        4,
        "tools/call",
        &json!({ "name": "find", "arguments": { "query": "name ~" } }),
    );
    assert_eq!(error["result"]["isError"], true);

    let second = server.request(
        5,
        "tools/call",
        &json!({ "name": "find", "arguments": { "query": "name = 'Cargo.toml'" } }),
    );
    let text = second["result"]["content"][0]["text"].as_str().unwrap();
    assert!(
        text.starts_with("files: 1  lines: 0  matches: 0  result: r2\n"),
        "{text}"
    );

    let unknown = server.request(6, "tools/call", &json!({ "name": "nope", "arguments": {} }));
    assert!(
        unknown.get("error").is_some(),
        "an unknown tool is a protocol error: {unknown}"
    );

    drop(server.stdin);
    let status = server.child.wait().unwrap();
    assert!(
        status.success(),
        "the server stops when stdin is closed: {status}"
    );
}

#[test]
fn version_flag() {
    let out = Command::new(BIN).arg("--version").output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        format!("easy-peasy-mcp {}, spec 1\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn unknown_argument() {
    let out = Command::new(BIN).arg("--bogus").output().unwrap();
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("usage: easy-peasy-mcp"), "{stderr}");
}

#[test]
fn options_and_keys_not_implemented_yet() {
    let out = Command::new(BIN).args(["--no-update"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "easy-peasy-mcp: the option '--no-update' is not implemented yet\n"
    );
    let dir = fixture();
    let config = dir.temp().join("config.toml");
    std::fs::write(&config, "[output]\nmax_files = 10\n").unwrap();
    let out = server_command(&dir.root, dir.temp())
        .arg("--config")
        .arg(&config)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        format!(
            "easy-peasy-mcp: {}:2: the key 'output.max_files' is not implemented yet\n",
            config.display()
        )
    );
}

/// Runs one `find` with the server's stderr captured; the stderr text.
fn stderr_of_session(args: &[&str]) -> String {
    let dir = fixture();
    let mut command = server_command(&dir.root, dir.temp());
    command.args(args).stderr(std::process::Stdio::piped());
    let mut child = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let reader = std::thread::spawn(move || {
        let mut text = String::new();
        std::io::Read::read_to_string(&mut stderr, &mut text).unwrap();
        text
    });
    let mut server = McpServer::from_child(child);
    server.initialize();
    let call = server.request(
        2,
        "tools/call",
        &json!({ "name": "find", "arguments": { "query": "name = 'Cargo.toml'" } }),
    );
    assert_eq!(call["result"]["isError"], false);
    drop(server.stdin);
    assert!(server.child.wait().unwrap().success());
    reader.join().unwrap()
}

#[test]
fn logs_go_to_stderr() {
    let debug = stderr_of_session(&["--log-level", "debug"]);
    assert!(
        debug.contains(&format!(
            "easy-peasy-mcp {} started",
            env!("CARGO_PKG_VERSION")
        )),
        "{debug}"
    );
    assert!(debug.contains("tools/call find"), "{debug}");
    assert_eq!(stderr_of_session(&[]), "");
}
