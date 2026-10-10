//! Test helpers: the example files of `tasks/`, fixtures in a temp dir, an MCP client over stdio.

#![allow(dead_code)] // every test binary uses a different part of the helpers

use std::fs;
use std::path::{Path, PathBuf};

/// The repository root.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A fenced code block of an example file: its info string and its text.
#[derive(Debug, Clone)]
pub struct Block {
    pub info: String,
    pub text: String,
}

/// One example: `## E<task>.<n> <title>` with its fixture and blocks.
#[derive(Debug, Clone)]
pub struct Example {
    pub id: String,
    pub file: String,
    pub fixture: Option<String>,
    pub blocks: Vec<Block>,
}

/// A fixture: `### <name>` under `## Fixtures` with a `text tree` block.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub name: String,
    /// Paths and their modification times in seconds since the Unix epoch, UTC; directories end with `/`.
    pub tree: Vec<(String, i64)>,
    /// Actions of the `text setup` block: `mkdir <dir>`, `file <path> [<bytes>]`, `symlink <path> -> <target>`.
    pub setup: Vec<String>,
}

/// All examples and fixtures of all `tasks/*__EXAMPLES__*.md` files, in task order.
pub fn load_examples() -> (Vec<Example>, Vec<Fixture>) {
    let dir = repo_root().join("tasks");
    let mut files: Vec<(u32, PathBuf)> = fs::read_dir(&dir)
        .expect("tasks/ is readable")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
                n.contains("__EXAMPLES__") && p.extension().is_some_and(|e| e == "md")
            })
        })
        .map(|p| {
            let n = p.file_name().unwrap().to_str().unwrap();
            let task: u32 = n.split("__").next().unwrap().parse().unwrap();
            (task, p)
        })
        .collect();
    files.sort();
    let mut examples = Vec::new();
    let mut fixtures = Vec::new();
    for (_, path) in files {
        let text = fs::read_to_string(&path).unwrap();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        parse_file(&name, &text, &mut examples, &mut fixtures);
    }
    (examples, fixtures)
}

fn parse_file(file: &str, text: &str, examples: &mut Vec<Example>, fixtures: &mut Vec<Fixture>) {
    #[derive(PartialEq)]
    enum Section {
        None,
        Fixture(String),
        Example,
    }
    let mut section = Section::None;
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        if let Some(title) = line.strip_prefix("## ") {
            if title.starts_with('E') && title.contains('.') {
                let id = title.split_whitespace().next().unwrap().to_string();
                examples.push(Example {
                    id,
                    file: file.to_string(),
                    fixture: None,
                    blocks: Vec::new(),
                });
                section = Section::Example;
            } else {
                section = Section::None;
            }
        } else if let Some(name) = line.strip_prefix("### ") {
            if section != Section::Example {
                section = Section::Fixture(name.trim().to_string());
            }
        } else if let Some(rest) = line.strip_prefix("Fixture: ") {
            if section == Section::Example {
                let name = rest.trim().trim_matches('`').to_string();
                examples.last_mut().unwrap().fixture = Some(name);
            }
        } else if let Some(info) = line.strip_prefix("```") {
            let fence = if line.starts_with("````") {
                "````"
            } else {
                "```"
            };
            let info = info.trim_start_matches('`').trim().to_string();
            let mut body = String::new();
            for l in lines.by_ref() {
                if l.trim_end() == fence {
                    break;
                }
                body.push_str(l);
                body.push('\n');
            }
            match &section {
                Section::Example => examples
                    .last_mut()
                    .unwrap()
                    .blocks
                    .push(Block { info, text: body }),
                Section::Fixture(name) if info == "text tree" => fixtures.push(Fixture {
                    name: name.clone(),
                    tree: parse_tree(&body),
                    setup: Vec::new(),
                }),
                Section::Fixture(name) if info == "text setup" => {
                    let fixture = fixtures
                        .iter_mut()
                        .rev()
                        .find(|f| &f.name == name)
                        .expect("the setup block comes after the tree block");
                    fixture.setup = body
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .map(str::to_string)
                        .collect();
                }
                _ => {}
            }
        }
    }
}

fn parse_tree(body: &str) -> Vec<(String, i64)> {
    body.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let n = parts.len();
            assert!(n >= 3, "tree line needs a path, a date and a time: {l}");
            let path = parts[..n - 2].join(" ");
            (path, parse_datetime(parts[n - 2], parts[n - 1]))
        })
        .collect()
}

/// `2026-10-01` `10:00` (UTC) → seconds since the Unix epoch.
pub fn parse_datetime(date: &str, time: &str) -> i64 {
    let d: Vec<i64> = date.split('-').map(|x| x.parse().unwrap()).collect();
    let t: Vec<i64> = time.split(':').map(|x| x.parse().unwrap()).collect();
    let days = days_from_civil(d[0], d[1], d[2]);
    let secs = t.get(2).copied().unwrap_or(0);
    days * 86_400 + t[0] * 3600 + t[1] * 60 + secs
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Copies `tests/fixtures/<name>` into a new temp dir, runs the setup actions and sets the modification times of
/// the tree. Returns the temp dir (kept alive by the caller) and the fixture root inside it.
pub fn setup_fixture(fixture: &Fixture) -> (tempfile::TempDir, PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join(&fixture.name);
    let source = repo_root().join("tests/fixtures").join(&fixture.name);
    if source.exists() {
        copy_dir(&source, &root);
    } else {
        fs::create_dir_all(&root).unwrap();
    }
    for action in &fixture.setup {
        run_setup(&root, action);
    }
    for (path, mtime) in &fixture.tree {
        let p = root.join(path.trim_end_matches('/'));
        let time = filetime::FileTime::from_unix_time(*mtime, 0);
        let meta = fs::symlink_metadata(&p).unwrap_or_else(|_| {
            panic!(
                "fixture {}: {path} is listed in the tree, but missing",
                fixture.name
            )
        });
        if meta.file_type().is_symlink() {
            filetime::set_symlink_file_times(&p, time, time).unwrap();
        } else {
            filetime::set_file_mtime(&p, time).unwrap();
        }
    }
    (tmp, root)
}

fn run_setup(root: &Path, action: &str) {
    let (verb, rest) = action
        .trim()
        .split_once(' ')
        .expect("an action has an argument");
    match verb {
        "mkdir" => fs::create_dir_all(root.join(rest.trim())).unwrap(),
        "file" => {
            let mut parts = rest.split_whitespace();
            let p = root.join(parts.next().expect("file <path> [<bytes>]"));
            let bytes: usize = parts
                .next()
                .map_or(0, |n| n.parse().expect("a number of bytes"));
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, "x".repeat(bytes)).unwrap();
        }
        "symlink" => {
            let (link, target) = rest.split_once(" -> ").expect("symlink <path> -> <target>");
            let link = root.join(link.trim());
            symlink(target.trim(), &link);
        }
        other => panic!("unknown setup action {other}"),
    }
}

#[cfg(unix)]
fn symlink(target: &str, link: &Path) {
    std::os::unix::fs::symlink(target, link).unwrap();
}

#[cfg(windows)]
fn symlink(target: &str, link: &Path) {
    let resolved = link.parent().unwrap().join(target);
    if resolved.is_dir() {
        std::os::windows::fs::symlink_dir(target, link).unwrap();
    } else {
        std::os::windows::fs::symlink_file(target, link).unwrap();
    }
}

/// Copies a directory tree.
pub fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}
