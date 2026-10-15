//! The CI of the repository: the workflow runs the definition of done on every platform, and the tracked files can
//! be checked out on every platform.

mod common;

use std::collections::HashMap;

use unicode_normalization::UnicodeNormalization;

/// The commands of the definition of done of AGENTS.md.
fn definition_of_done() -> Vec<String> {
    let agents = std::fs::read_to_string(common::repo_root().join("AGENTS.md")).unwrap();
    let section = agents
        .split("## Definition of done")
        .nth(1)
        .expect("AGENTS.md has a definition of done");
    let block = section
        .split("```sh\n")
        .nth(1)
        .and_then(|b| b.split("```").next())
        .expect("the definition of done has a sh block");
    block.lines().map(str::to_string).collect()
}

#[test]
fn ci_runs_the_definition_of_done() {
    let workflow =
        std::fs::read_to_string(common::repo_root().join(".github/workflows/ci.yml")).unwrap();
    for os in ["ubuntu-latest", "macos-latest", "windows-latest"] {
        assert!(workflow.contains(os), "CI runs on {os}");
    }
    let commands = definition_of_done();
    assert_eq!(commands.len(), 4, "{commands:?}");
    for command in commands {
        assert!(
            workflow
                .lines()
                .any(|l| l.trim() == format!("run: {command}")),
            "CI runs {command}"
        );
    }
}

/// Whether a file name can't be created on Windows: a reserved character, a trailing dot or space, a device name.
fn reserved_on_windows(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name).to_ascii_uppercase();
    let device = ["CON", "PRN", "AUX", "NUL"].contains(&stem.as_str())
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.ends_with(|c: char| c.is_ascii_digit()));
    name.contains(['<', '>', ':', '"', '|', '?', '*', '\\'])
        || name.chars().any(char::is_control)
        || name.ends_with(['.', ' '])
        || device
}

#[test]
fn tracked_names_are_portable() {
    let output = std::process::Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(common::repo_root())
        .output()
        .expect("git is installed");
    if !output.status.success() {
        return; // not a git checkout, e.g. a source package
    }
    let listed = String::from_utf8(output.stdout).expect("UTF-8 paths");
    let paths: Vec<&str> = listed.split('\0').filter(|p| !p.is_empty()).collect();
    assert_ne!(paths.len(), 0);
    let mut folded: HashMap<String, &str> = HashMap::new();
    for path in paths {
        for name in path.split('/') {
            assert!(!reserved_on_windows(name), "{path}: not valid on Windows");
        }
        let key: String = path.nfc().collect::<String>().to_lowercase();
        if let Some(other) = folded.insert(key, path) {
            panic!(
                "{path} and {other} differ only in case: one of them is lost on macOS and Windows"
            );
        }
    }
}

#[test]
fn windows_names() {
    for name in ["a:b", "x?", "CON", "nul.txt", "com1", "a.", "b "] {
        assert!(reserved_on_windows(name), "{name}");
    }
    for name in ["a.rs", "CONFIG.md", "com", "console.log", ".gitignore"] {
        assert!(!reserved_on_windows(name), "{name}");
    }
}
