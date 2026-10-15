//! End to end: spawn the server binary and talk JSON-RPC over its stdin and stdout.

mod common;

use std::process::Command;

use common::{BIN, McpServer, server_command};
use serde_json::{Value, json};

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
        "File search: find + grep in one query. Call get_config first to learn allowed roots and defaults. Spec version 2."
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
    assert!(description.contains("Spec version 2."), "{description}");

    let call = server.request(
        3,
        "tools/call",
        &json!({ "name": "find", "arguments": { "query": "name = 'Cargo.toml'" } }),
    );
    assert_eq!(call["result"]["isError"], false);
    assert_eq!(
        call["result"]["content"],
        json!([{ "type": "text", "text": format!(
            "files: 1  lines: 0  matches: 0  result: r1\n\n.{}\nCargo.toml\n",
            std::path::MAIN_SEPARATOR
        ) }])
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
        format!("easy-peasy-mcp {}, spec 2\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn unknown_argument() {
    let out = Command::new(BIN).arg("--bogus").output().unwrap();
    assert!(!out.status.success());
    assert_eq!(out.stdout.len(), 0);
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("usage: easy-peasy-mcp"), "{stderr}");
}

#[test]
fn an_invalid_value_at_start() {
    let dir = fixture();
    let config = dir.temp().join("config.toml");
    std::fs::write(&config, "[output]\nchars_per_token = 0\n").unwrap();
    let out = server_command(&dir.root, dir.temp())
        .arg("--config")
        .arg(&config)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        format!(
            "easy-peasy-mcp: {}:2: invalid value for 'output.chars_per_token': a positive number is expected\n",
            config.display()
        )
    );
}

#[test]
fn no_update_hides_update_config() {
    let dir = fixture();
    let mut server = McpServer::spawn(server_command(&dir.root, dir.temp()).arg("--no-update"));
    server.initialize();
    let list = server.request(2, "tools/list", &json!({}));
    let names: Vec<&str> = list["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["find", "get_config"]);
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

/// The fenced blocks of a markdown text: the headings above each block, its info string and its text.
fn fenced_blocks(markdown: &str) -> Vec<(Vec<String>, String, String)> {
    let mut blocks = Vec::new();
    let mut headings: Vec<(usize, String)> = Vec::new();
    let mut lines = markdown.lines();
    while let Some(line) = lines.next() {
        let level = line.chars().take_while(|&c| c == '#').count();
        if level > 0 && line[level..].starts_with(' ') {
            headings.retain(|(l, _)| *l < level);
            headings.push((level, line[level + 1..].to_string()));
        } else if let Some(info) = line.strip_prefix("```") {
            let text: Vec<&str> = lines
                .by_ref()
                .take_while(|l| !l.starts_with("```"))
                .collect();
            let path = headings.iter().map(|(_, h)| h.clone()).collect();
            blocks.push((path, info.to_string(), text.join("\n")));
        }
    }
    blocks
}

/// The block of TOOLS.md with `info` under the headings ending with `path`.
fn spec_block<'a>(
    blocks: &'a [(Vec<String>, String, String)],
    path: &[&str],
    info: &str,
) -> &'a str {
    blocks
        .iter()
        .find(|(headings, i, _)| {
            i == info
                && headings.ends_with(&path.iter().map(ToString::to_string).collect::<Vec<_>>())
        })
        .map_or_else(
            || panic!("TOOLS.md has a {info} block under {path:?}"),
            |(_, _, text)| text.as_str(),
        )
}

/// The annotations of `tool` in the table of TOOLS.md#tools: `readOnlyHint: true`, ...
fn spec_annotations(spec: &str, tool: &str) -> Value {
    let row = spec
        .lines()
        .find(|l| l.starts_with(&format!("| `{tool}` ")))
        .unwrap_or_else(|| panic!("TOOLS.md lists {tool}"));
    let cell = row.split('|').nth(2).unwrap();
    let mut annotations = serde_json::Map::new();
    for hint in cell.split(',') {
        let (key, value) = hint.trim().trim_matches('`').split_once(": ").unwrap();
        annotations.insert(key.to_string(), json!(value == "true"));
    }
    Value::Object(annotations)
}

#[test]
fn tools_are_the_spec_text() {
    let spec = std::fs::read_to_string(common::repo_root().join("TOOLS.md")).unwrap();
    let blocks = fenced_blocks(&spec);
    let dir = fixture();
    // the write tools are listed only with --allow-write
    for (args, count) in [(&[][..], 3), (&["--allow-write"][..], 7)] {
        let mut server =
            McpServer::spawn(server_command(&dir.root, dir.temp()).args(args.iter().copied()));

        let init = server.initialize();
        let mut expected: Value =
            serde_json::from_str(spec_block(&blocks, &["Transport"], "json")).unwrap();
        expected["serverInfo"]["version"] = json!(env!("CARGO_PKG_VERSION"));
        for key in ["capabilities", "serverInfo", "instructions"] {
            assert_eq!(init["result"][key], expected[key], "{key}");
        }

        let list = server.request(2, "tools/list", &json!({}));
        let tools = list["result"]["tools"].as_array().expect("tools");
        assert_eq!(tools.len(), count, "{args:?}");
        for tool in tools {
            let name = tool["name"].as_str().unwrap();
            let expected = json!({
                "name": name,
                "description": spec_block(&blocks, &["Tools", name, "Description"], ""),
                "inputSchema": serde_json::from_str::<Value>(spec_block(&blocks, &["Tools", name, "Input schema"], "json")).unwrap(),
                "annotations": spec_annotations(&spec, name),
            });
            assert_eq!(tool, &expected, "{name}");
        }
    }
}
