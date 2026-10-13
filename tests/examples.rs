//! Runs every example of every task (`tasks/*__EXAMPLES__*.md`) through the library entry point.

mod common;

use std::fmt::Write as _;

use common::{Example, Fixture, load_examples, setup_fixture};
use easy_peasy_mcp::{Engine, EngineOptions, Trash};

/// Whether an example was skipped because its fixture can't be set up on this machine.
enum Skipped {
    No,
    Yes,
    /// A startup example, run by `tests/startup.rs`.
    Startup,
    /// An example of another platform (`Platform:`).
    Elsewhere,
}

fn run_example(example: &Example, fixtures: &[Fixture]) -> Result<Skipped, String> {
    let name = example
        .fixture
        .as_deref()
        .ok_or_else(|| "the example has no fixture".to_string())?;
    let fixture = fixtures
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| format!("unknown fixture {name}"))?;
    if example.is_startup() {
        return Ok(Skipped::Startup);
    }
    if !example.runs_here() {
        return Ok(Skipped::Elsewhere);
    }
    let dir = setup_fixture(fixture);
    if !dir.permissions_removed {
        return Ok(Skipped::Yes);
    }
    let fixture_path = dir.root.to_string_lossy().into_owned();
    // `server.output_dir` of the examples: `out` next to the fixture, not created beforehand (AGENTS.md)
    let output_dir = common::canonical(dir.temp()).join("out");
    let output_path = output_dir.to_string_lossy().into_owned();
    let mut options = EngineOptions::new(dir.root.clone());
    options.settings.output_dir = Some(output_dir.clone());
    options.git_global_excludes = false;
    options.time_zone = jiff::tz::TimeZone::UTC;
    options.now = fixture
        .now
        .map(|secs| jiff::Timestamp::from_second(secs).expect("a valid now"));
    options.read_delay = example.slow_reads;
    // the examples are written with the Linux path case unless they are marked with a platform (AGENTS.md)
    if example.platform.is_none() {
        options.native_case_insensitive = false;
    }
    // the trash test hook (AGENTS.md): the examples never touch the trash of the machine
    let trash = dir.temp().join("trash");
    options.trash = Trash::Dir(trash.clone());
    let engine = Engine::new(options);

    let mut blocks = example.blocks.iter().peekable();
    let mut step = 0;
    while let Some(block) = blocks.next() {
        if let Some(checked) = common::check_file(example, block, &fixture_path, &output_dir) {
            checked.map_err(|e| format!("step {step}: {e}"))?;
            continue;
        }
        if let Some(checked) = common::check_trash(block, &trash) {
            checked.map_err(|e| format!("step {step}: {e}"))?;
            continue;
        }
        if common::run_example_setup(block, &dir.root).is_some() {
            continue;
        }
        let Some(tool) = block.info.strip_prefix("json request") else {
            continue;
        };
        step += 1;
        let tool = match tool.trim() {
            "" => "find",
            other => other,
        };
        let request = common::placeholders(&block.text, true, &fixture_path, &output_path);
        let arguments: serde_json::Value = serde_json::from_str(&request)
            .map_err(|e| format!("step {step}: invalid request JSON: {e}"))?;
        let expected = blocks
            .next()
            .ok_or_else(|| format!("step {step}: no response after the request"))?;
        let output = engine
            .call(tool, &arguments)
            .map_err(|e| format!("step {step}: {e}"))?;
        let (want_error, kind) = match expected.info.as_str() {
            "text response" | "json response" => (false, "response"),
            "text error" | "json error" => (true, "error"),
            other => return Err(format!("step {step}: unknown block `{other}`")),
        };
        let expected_text = common::placeholders(
            &expected.text,
            expected.info.starts_with("json "),
            &fixture_path,
            &output_path,
        );
        let same = common::same_response(example, &expected.info, &expected_text, &output.text);
        if output.is_error != want_error || !same {
            return Err(format!(
                "step {step}: expected {kind} (isError: {want_error}):\n{expected_text}\nactual (isError: {}):\n{}",
                output.is_error, output.text
            ));
        }
    }
    if step == 0 {
        return Err("the example has no request".to_string());
    }
    Ok(Skipped::No)
}

#[test]
fn all_examples() {
    let (examples, fixtures) = load_examples();
    assert!(!examples.is_empty(), "no examples found");
    let mut failures = String::new();
    let mut count = 0;
    let mut skipped = Vec::new();
    for example in &examples {
        match run_example(example, &fixtures) {
            Ok(Skipped::No) => count += 1,
            Ok(Skipped::Yes) => skipped.push(example.id.as_str()),
            Ok(Skipped::Startup | Skipped::Elsewhere) => {}
            Err(e) => {
                let _ = writeln!(failures, "=== {} ({}) ===\n{e}", example.id, example.file);
            }
        }
    }
    assert!(failures.is_empty(), "examples failed:\n{failures}");
    println!("{count} examples passed");
    if !skipped.is_empty() {
        println!(
            "skipped, permissions can't be removed here: {}",
            skipped.join(", ")
        );
    }
}
