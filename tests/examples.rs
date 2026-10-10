//! Runs every example of every task (`tasks/*__EXAMPLES__*.md`) through the library entry point.

mod common;

use std::fmt::Write as _;

use common::{Example, Fixture, load_examples, setup_fixture};
use easy_peasy_mcp::{Engine, EngineOptions};

fn run_example(example: &Example, fixtures: &[Fixture]) -> Result<(), String> {
    let name = example
        .fixture
        .as_deref()
        .ok_or_else(|| "the example has no fixture".to_string())?;
    let fixture = fixtures
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| format!("unknown fixture {name}"))?;
    let (_tmp, root) = setup_fixture(fixture);
    let mut options = EngineOptions::new(root);
    options.git_global_excludes = false;
    options.time_zone = jiff::tz::TimeZone::UTC;
    options.now = fixture
        .now
        .map(|secs| jiff::Timestamp::from_second(secs).expect("a valid now"));
    let engine = Engine::new(options);

    let mut blocks = example.blocks.iter().peekable();
    let mut step = 0;
    while let Some(block) = blocks.next() {
        let Some(tool) = block.info.strip_prefix("json request") else {
            continue;
        };
        step += 1;
        let tool = match tool.trim() {
            "" => "find",
            other => other,
        };
        let arguments: serde_json::Value = serde_json::from_str(&block.text)
            .map_err(|e| format!("step {step}: invalid request JSON: {e}"))?;
        let expected = blocks
            .next()
            .ok_or_else(|| format!("step {step}: no response after the request"))?;
        let output = engine
            .call(tool, &arguments)
            .map_err(|e| format!("step {step}: {e}"))?;
        let (want_error, kind) = match expected.info.as_str() {
            "text response" | "json response" => (false, "response"),
            "text error" => (true, "error"),
            other => return Err(format!("step {step}: unknown block `{other}`")),
        };
        if output.is_error != want_error || output.text != expected.text {
            return Err(format!(
                "step {step}: expected {kind} (isError: {want_error}):\n{}\nactual (isError: {}):\n{}",
                expected.text, output.is_error, output.text
            ));
        }
    }
    if step == 0 {
        return Err("the example has no request".to_string());
    }
    Ok(())
}

#[test]
fn all_examples() {
    let (examples, fixtures) = load_examples();
    assert!(!examples.is_empty(), "no examples found");
    let mut failures = String::new();
    let mut count = 0;
    for example in &examples {
        count += 1;
        if let Err(e) = run_example(example, &fixtures) {
            let _ = writeln!(failures, "=== {} ({}) ===\n{e}", example.id, example.file);
        }
    }
    assert!(failures.is_empty(), "examples failed:\n{failures}");
    println!("{count} examples passed");
}
