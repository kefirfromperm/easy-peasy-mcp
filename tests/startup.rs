//! Runs the startup examples (AGENTS.md): the binary with a command line, environment and config files.

mod common;

use std::fmt::Write as _;
use std::fs;

use std::time::Duration;

use common::{Example, Exchange, Fixture, McpServer, load_examples, server_command, setup_fixture};
use serde_json::{Value, json};

fn run_example(example: &Example, fixtures: &[Fixture]) -> Result<(), String> {
    let name = example
        .fixture
        .as_deref()
        .ok_or_else(|| "the example has no fixture".to_string())?;
    let fixture = fixtures
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| format!("unknown fixture {name}"))?;
    let dir = setup_fixture(fixture);
    let fixture_path = dir.root.to_string_lossy().into_owned();
    let home = dir.temp().join("no-home");
    let mut command = None;
    let mut env = Vec::new();
    for block in &example.blocks {
        if let Some(path) = block.info.strip_prefix("toml ") {
            let path = dir.root.join(path.trim());
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, &block.text).unwrap();
        } else if block.info == "text env" {
            for line in block.text.lines().filter(|l| !l.trim().is_empty()) {
                let (key, value) = line.split_once('=').ok_or("env: NAME=value")?;
                env.push((key.to_string(), value.replace("<fixture>", &fixture_path)));
            }
        } else if block.info == "text command" {
            command = Some(block.text.trim().to_string());
        }
    }
    let command = command.ok_or("no command")?;
    let mut words = command.split_whitespace();
    if words.next() != Some("easy-peasy-mcp") {
        return Err(format!("the command doesn't start the server: {command}"));
    }
    let mut process = server_command(&dir.root, &home);
    process.args(words);
    for (key, value) in &env {
        process.env(key, value);
    }
    if let Some(slow) = example.slow_reads {
        process.env("EASY_PEASY_TEST_SLOW_READS", slow.as_millis().to_string());
    }

    if let Some(block) = example
        .blocks
        .iter()
        .find(|b| b.info.starts_with("text stderr exit "))
    {
        let code: i32 = block.info["text stderr exit ".len()..]
            .trim()
            .parse()
            .map_err(|_| "text stderr exit <code>")?;
        let expected = block.text.replace("<fixture>", &fixture_path);
        let output = process
            .stdin(std::process::Stdio::null())
            .output()
            .map_err(|e| e.to_string())?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if output.status.code() != Some(code) || stderr != expected || !output.stdout.is_empty() {
            return Err(format!(
                "expected exit {code} and stderr:\n{expected}\nactual exit {:?}, stdout {} bytes, stderr:\n{stderr}",
                output.status.code(),
                output.stdout.len()
            ));
        }
        return Ok(());
    }

    run_session(McpServer::spawn(&mut process), example, &fixture_path)
}

/// Sends the requests of a startup example to the started server and compares the responses; runs its JSON-RPC
/// exchange blocks.
fn run_session(server: McpServer, example: &Example, fixture_path: &str) -> Result<(), String> {
    let mut server = server;
    if server.response_to_initialize().is_none() {
        let status = server.child.wait().map_err(|e| e.to_string())?;
        return Err(format!("the server didn't start: {status}"));
    }
    let mut exchange = server.into_exchange();
    let result = run_blocks(&mut exchange, example, fixture_path);
    let _ = exchange.child.kill();
    let _ = exchange.child.wait();
    result
}

/// The blocks of a startup example after `initialize`.
fn run_blocks(exchange: &mut Exchange, example: &Example, fixture_path: &str) -> Result<(), String> {
    let mut blocks = example.blocks.iter();
    let mut id = 1;
    while let Some(block) = blocks.next() {
        let parse = |text: &str| -> Result<Value, String> {
            serde_json::from_str(text).map_err(|e| format!("{}: {e}", block.info))
        };
        match block.info.as_str() {
            "json send" => exchange.send(&parse(&block.text)?),
            "json receive" => {
                let expected = parse(&block.text)?;
                if exchange.receive(&expected, Duration::from_secs(10)).is_none() {
                    return Err(format!(
                        "no message like {expected}; received:\n{}",
                        log(exchange)
                    ));
                }
            }
            "json never" => {
                let pattern = parse(&block.text)?;
                if let Some(message) = exchange.never(&pattern, Duration::from_secs(1)) {
                    return Err(format!("an unexpected message: {message}"));
                }
            }
            info => {
                let Some(tool) = info.strip_prefix("json request") else {
                    continue;
                };
                let tool = match tool.trim() {
                    "" => "find",
                    other => other,
                };
                id += 1;
                let arguments = parse(&block.text)?;
                let expected = blocks
                    .next()
                    .ok_or_else(|| format!("request {id}: no response"))?;
                let response = exchange
                    .request(id, "tools/call", &json!({ "name": tool, "arguments": arguments }))
                    .ok_or_else(|| format!("request {id}: no response from the server"))?;
                let text = response["result"]["content"][0]["text"]
                    .as_str()
                    .unwrap_or_default();
                let is_error = response["result"]["isError"] == json!(true);
                let expected_text = expected
                    .text
                    .replace("<version>", env!("CARGO_PKG_VERSION"))
                    .replace("<fixture>", fixture_path);
                let same = if expected.info.starts_with("json ") {
                    common::same_json(&expected_text, text)
                } else {
                    text == expected_text
                };
                if is_error != expected.info.ends_with(" error") || !same {
                    return Err(format!(
                        "request {id}: expected:\n{expected_text}\nactual (isError: {is_error}):\n{text}"
                    ));
                }
            }
        }
    }
    Ok(())
}

/// The messages received in an exchange, one per line.
fn log(exchange: &Exchange) -> String {
    exchange
        .log
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn exchange_blocks_match_messages() {
    let message = json!({
        "jsonrpc": "2.0",
        "method": "notifications/progress",
        "params": { "progressToken": "p7", "progress": 12, "message": "scanned 12 files, 3 matched, 1.0s" }
    });
    assert!(common::matches(
        &json!({ "params": { "progress": "*", "message": "scanned * files, * matched, *s" } }),
        &message
    ));
    assert!(!common::matches(&json!({ "params": { "progressToken": "p8" } }), &message));
    assert!(!common::matches(&json!({ "params": { "message": "scanned * dirs*" } }), &message));
    assert!(common::matches(&json!({ "method": "notifications/progress" }), &message));
}

#[test]
fn startup_examples() {
    let (examples, fixtures) = load_examples();
    let mut failures = String::new();
    let mut count = 0;
    for example in examples.iter().filter(|e| e.is_startup() && e.runs_here()) {
        count += 1;
        if let Err(e) = run_example(example, &fixtures) {
            let _ = writeln!(failures, "=== {} ({}) ===\n{e}", example.id, example.file);
        }
    }
    assert!(failures.is_empty(), "startup examples failed:\n{failures}");
    println!("{count} startup examples passed");
}
