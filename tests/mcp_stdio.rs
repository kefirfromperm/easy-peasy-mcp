//! End to end: spawn the server binary and talk JSON-RPC over its stdin and stdout.

mod common;

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

const BIN: &str = env!("CARGO_BIN_EXE_easy-peasy-mcp");

/// A running server and the pipes to it.
struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Server {
    fn start(dir: &Path, home: &Path) -> Self {
        let mut child = Command::new(BIN)
            .current_dir(dir)
            .env("HOME", home)
            .env("XDG_CONFIG_HOME", home)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("the server starts");
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            stdin,
            stdout,
        }
    }

    fn send(&mut self, message: &Value) {
        writeln!(self.stdin, "{message}").unwrap();
        self.stdin.flush().unwrap();
    }

    /// Reads messages until the response with `id`.
    fn response(&mut self, id: i64) -> Value {
        loop {
            let mut line = String::new();
            let n = self.stdout.read_line(&mut line).unwrap();
            assert!(n > 0, "the server closed stdout before responding to {id}");
            let message: Value =
                serde_json::from_str(&line).expect("stdout carries only JSON-RPC messages");
            if message["id"] == json!(id) {
                return message;
            }
        }
    }

    fn request(&mut self, id: i64, method: &str, params: &Value) -> Value {
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        self.response(id)
    }

    fn initialize(&mut self) -> Value {
        let response = self.request(
            1,
            "initialize",
            &json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0" }
            }),
        );
        self.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        response
    }
}

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
    let mut server = Server::start(&dir.root, dir.temp());

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
        "File search: find + grep in one query. Spec version 1."
    );

    let list = server.request(2, "tools/list", &json!({}));
    let tools = list["result"]["tools"].as_array().expect("tools");
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["find"]);
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
