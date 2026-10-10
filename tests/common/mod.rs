//! Test helpers: the example files of `tasks/`, fixtures in a temp dir, an MCP client over stdio.

#![allow(dead_code)] // every test binary uses a different part of the helpers

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

/// The server binary.
pub const BIN: &str = env!("CARGO_BIN_EXE_easy-peasy-mcp");

/// The command of the server binary in `dir`, with `HOME`, `XDG_CONFIG_HOME` and `APPDATA` at `home` and without
/// `EASY_PEASY_CONFIG`, so no config file of the machine is read.
pub fn server_command(dir: &Path, home: &Path) -> Command {
    let mut command = Command::new(BIN);
    command
        .current_dir(dir)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home)
        .env("APPDATA", home)
        .env_remove("EASY_PEASY_CONFIG");
    command
}

/// A running server and the pipes to it.
pub struct McpServer {
    pub child: Child,
    pub stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl McpServer {
    /// Starts the server of `command` with piped stdin and stdout.
    pub fn spawn(command: &mut Command) -> Self {
        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("the server starts");
        Self::from_child(child)
    }

    /// The server of a child started with piped stdin and stdout.
    pub fn from_child(mut child: Child) -> Self {
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            stdin,
            stdout,
        }
    }

    pub fn send(&mut self, message: &Value) {
        writeln!(self.stdin, "{message}").unwrap();
        self.stdin.flush().unwrap();
    }

    /// Reads messages until the response with `id`; `None` when the server closed stdout.
    pub fn response(&mut self, id: i64) -> Option<Value> {
        loop {
            let mut line = String::new();
            if self.stdout.read_line(&mut line).unwrap() == 0 {
                return None;
            }
            let message: Value =
                serde_json::from_str(&line).expect("stdout carries only JSON-RPC messages");
            if message["id"] == json!(id) {
                return Some(message);
            }
        }
    }

    pub fn request(&mut self, id: i64, method: &str, params: &Value) -> Value {
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        self.response(id)
            .unwrap_or_else(|| panic!("the server closed stdout before responding to {id}"))
    }

    /// Initializes the session; `None` when the server exited instead of answering.
    pub fn response_to_initialize(&mut self) -> Option<Value> {
        self.send(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0" }
            }
        }));
        let response = self.response(1)?;
        self.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        Some(response)
    }

    pub fn initialize(&mut self) -> Value {
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
    /// The only platform the example runs on, from a line `` Platform: `linux` ``.
    pub platform: Option<String>,
    pub blocks: Vec<Block>,
}

impl Example {
    /// Whether the example starts the binary (a `text command` block) instead of calling the library.
    pub fn is_startup(&self) -> bool {
        self.blocks.iter().any(|b| b.info == "text command")
    }

    /// Whether the example runs on this platform.
    pub fn runs_here(&self) -> bool {
        self.platform
            .as_deref()
            .is_none_or(|p| p == std::env::consts::OS)
    }
}

/// A fixture: `### <name>` under `## Fixtures` with a `text tree` block.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub name: String,
    /// Paths and their modification times in seconds since the Unix epoch, UTC; directories end with `/`.
    pub tree: Vec<(String, i64)>,
    /// The fixed `now` of the fixture in seconds since the Unix epoch, from a line `` Now: `2026-10-08 12:00` ``.
    pub now: Option<i64>,
    /// Actions of the `text setup` block: `mkdir <dir>`, `file <path> [<bytes>]`, `bytes <path> <hex>`,
    /// `symlink <path> -> <target>`, `atime <path> <date> <time>`, `git <dir> <args>`, `chmod <path> <mode>`.
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
                    platform: None,
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
        } else if let (Some(rest), Section::Fixture(name)) = (line.strip_prefix("Now: "), &section)
        {
            let text = rest.trim().trim_matches('`');
            let (date, time) = text.split_once(' ').expect("Now: `<date> <time>`");
            let fixture = fixtures
                .iter_mut()
                .rev()
                .find(|f| &f.name == name)
                .expect("the Now line comes after the tree block");
            fixture.now = Some(parse_datetime(date, time));
        } else if let Some(rest) = line.strip_prefix("Platform: ") {
            if section == Section::Example {
                let name = rest.trim().trim_matches('`').to_string();
                examples.last_mut().unwrap().platform = Some(name);
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
                    now: None,
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
/// A fixture set up in a temp dir; the permissions removed by `chmod` are restored when it is dropped, so the temp
/// dir can be removed.
pub struct FixtureDir {
    pub root: PathBuf,
    /// Whether every `chmod` took effect: false on Windows and when the entry stays readable (running as root).
    pub permissions_removed: bool,
    restricted: Vec<PathBuf>,
    tmp: tempfile::TempDir,
}

impl FixtureDir {
    /// The temp dir which holds the fixture directory.
    pub fn temp(&self) -> &Path {
        self.tmp.path()
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        for path in &self.restricted {
            restore_permissions(path);
        }
    }
}

/// Copies the fixture to a temp dir and runs its setup: actions, times, access times, permissions.
pub fn setup_fixture(fixture: &Fixture) -> FixtureDir {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join(&fixture.name);
    let source = repo_root().join("tests/fixtures").join(&fixture.name);
    if source.exists() {
        copy_dir(&source, &root);
    } else {
        fs::create_dir_all(&root).unwrap();
    }
    let late = |a: &&String| a.starts_with("atime ") || a.starts_with("chmod ");
    for action in fixture.setup.iter().filter(|a| !late(a)) {
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
    for action in fixture.setup.iter().filter(|a| a.starts_with("atime ")) {
        let parts: Vec<&str> = action.split_whitespace().collect();
        let p = root.join(parts[1]);
        let atime = filetime::FileTime::from_unix_time(parse_datetime(parts[2], parts[3]), 0);
        let mtime = filetime::FileTime::from_last_modification_time(&fs::metadata(&p).unwrap());
        filetime::set_file_times(&p, atime, mtime).unwrap();
    }
    let mut restricted = Vec::new();
    let mut permissions_removed = true;
    for action in fixture.setup.iter().filter(|a| a.starts_with("chmod ")) {
        let parts: Vec<&str> = action.split_whitespace().collect();
        let [_, path, mode] = parts[..] else {
            panic!("chmod <path> <mode>: {action}")
        };
        let p = root.join(path);
        let mode = u32::from_str_radix(mode, 8).expect("an octal mode");
        permissions_removed &= set_permissions(&p, mode);
        restricted.push(p);
    }
    FixtureDir {
        root,
        permissions_removed,
        restricted,
        tmp,
    }
}

/// Sets the Unix permissions of `path`; whether it is unreadable afterwards, as the mode asks.
#[cfg(unix)]
fn set_permissions(path: &Path, mode: u32) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
    let readable = if path.is_dir() {
        fs::read_dir(path).is_ok()
    } else {
        fs::File::open(path).is_ok()
    };
    readable == (mode & 0o400 != 0)
}

#[cfg(not(unix))]
fn set_permissions(_path: &Path, _mode: u32) -> bool {
    false
}

#[cfg(unix)]
fn restore_permissions(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
}

#[cfg(not(unix))]
fn restore_permissions(_path: &Path) {}

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
        "bytes" => {
            let (path, hex) = rest.trim().split_once(' ').expect("bytes <path> <hex>");
            let hex: String = hex.split_whitespace().collect();
            let bytes: Vec<u8> = (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex bytes"))
                .collect();
            let p = root.join(path);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, bytes).unwrap();
        }
        "git" | "git!" => {
            let (dir, args) = rest.trim().split_once(' ').expect("git <dir> <args>");
            let status = std::process::Command::new("git")
                .arg("-C")
                .arg(root.join(dir))
                .args(args.split_whitespace())
                .env("GIT_CONFIG_GLOBAL", null_device())
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_AUTHOR_NAME", "Test")
                .env("GIT_AUTHOR_EMAIL", "test@example.com")
                .env("GIT_COMMITTER_NAME", "Test")
                .env("GIT_COMMITTER_EMAIL", "test@example.com")
                .env("GIT_AUTHOR_DATE", "2026-10-01T10:00:00Z")
                .env("GIT_COMMITTER_DATE", "2026-10-01T10:00:00Z")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .expect("git is installed");
            assert!(verb == "git!" || status.success(), "setup failed: {action}");
        }
        other => panic!("unknown setup action {other}"),
    }
}

fn null_device() -> &'static str {
    if cfg!(windows) { "NUL" } else { "/dev/null" }
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
