//! Every query of the code blocks of QUERY.md is valid: the server answers it without an error.

use std::fmt::Write as _;
use std::path::Path;

use easy_peasy_mcp::{Engine, EngineOptions};
use serde_json::json;

/// The code blocks of QUERY.md which are not queries: the section and the start of the first line of the block.
const NOT_QUERIES: &[(&str, &str)] = &[
    ("Dates", "2026-10-01"),   // absolute dates, the values of a condition
    ("Dates", "-30m"),         // relative dates
    ("Comments", "// let x"),  // the code which `/c` searches
    ("Escaping", "{\"type\""), // the text which `/e` matches
    ("Proximity", "near(<A>"), // the syntax with placeholders
    ("Grammar", "query "),     // the grammar itself
    ("Errors", "error["),      // an error message
];

/// A fenced code block: the heading of its section, its info string and its lines, trimmed.
struct Block {
    section: String,
    info: String,
    lines: Vec<String>,
}

/// The fenced code blocks of a Markdown text.
fn code_blocks(markdown: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut section = String::new();
    let mut lines = markdown.lines();
    while let Some(line) = lines.next() {
        if let Some(info) = line.trim_start().strip_prefix("```") {
            let lines = lines
                .by_ref()
                .take_while(|l| !l.trim_start().starts_with("```"))
                .map(|l| l.trim().to_string())
                .collect();
            blocks.push(Block {
                section: section.clone(),
                info: info.trim().to_string(),
                lines,
            });
        } else if line.starts_with('#') {
            section = line.trim_start_matches('#').trim().to_string();
        }
    }
    blocks
}

/// The queries of a block: the `query` of a JSON request, otherwise one per line, where a line starting with `and`,
/// `or` or `xor` continues the query above it. Comments are removed; a line of `^` markers starts the explanation of
/// the query above it, which goes to the end of the block.
fn queries(block: &Block) -> Vec<String> {
    if block.info == "json" {
        let request: serde_json::Value =
            serde_json::from_str(&block.lines.join("\n")).expect("a JSON request");
        return vec![request["query"].as_str().expect("a query").to_string()];
    }
    assert_eq!(block.info, "", "a block of QUERY.md#{}", block.section);
    let mut queries: Vec<String> = Vec::new();
    for line in &block.lines {
        let line = without_comment(line).trim_end();
        if line.starts_with('^') {
            break;
        }
        if line.is_empty() {
            continue;
        }
        let continues = ["and ", "or ", "xor "]
            .iter()
            .any(|op| line.starts_with(op));
        match queries.last_mut() {
            Some(query) if continues => {
                query.push('\n');
                query.push_str(line);
            }
            _ => queries.push(line.to_string()),
        }
    }
    queries
}

/// The line without its comment: `#` outside of a string starts a comment until the end of the line. Strings are
/// `'...'` and `"..."` with `\` escapes, and `` `...` `` without escapes.
fn without_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (i, c) in line.char_indices() {
        if let Some(q) = quote {
            if escaped {
                escaped = false;
            } else if c == '\\' && q != '`' {
                escaped = true;
            } else if c == q {
                quote = None;
            }
        } else if c == '#' {
            return &line[..i];
        } else if matches!(c, '\'' | '"' | '`') {
            quote = Some(c);
        }
    }
    line
}

#[test]
fn every_query_of_query_md_is_valid() {
    let spec = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("QUERY.md"))
        .expect("QUERY.md is readable");
    let tmp = tempfile::tempdir().unwrap();
    let mut options = EngineOptions::new(tmp.path().to_path_buf());
    options.git_global_excludes = false;
    let engine = Engine::new(options);

    let mut found = vec![false; NOT_QUERIES.len()];
    let mut count = 0;
    let mut failures = String::new();
    for block in code_blocks(&spec) {
        let first = block.lines.first().map_or("", String::as_str);
        let not_a_query = NOT_QUERIES
            .iter()
            .position(|(section, start)| block.section == *section && first.starts_with(start));
        if let Some(i) = not_a_query {
            found[i] = true;
            continue;
        }
        for query in queries(&block) {
            count += 1;
            // without the funnel: its hints for the empty result only take time
            let out = engine.find(
                &json!({ "query": query, "output": { "detail": "count", "funnel": "never" } }),
            );
            if out.is_error {
                let _ = writeln!(
                    failures,
                    "=== QUERY.md#{}\n{query}\n{}",
                    block.section, out.text
                );
            }
        }
    }
    assert!(
        failures.is_empty(),
        "queries of QUERY.md with an error:\n{failures}"
    );
    for ((section, start), found) in NOT_QUERIES.iter().zip(found) {
        assert!(
            found,
            "no block of QUERY.md#{section} starts with {start:?}"
        );
    }
    println!("{count} queries of QUERY.md are valid");
}
