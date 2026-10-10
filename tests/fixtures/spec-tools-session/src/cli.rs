//! The command line.

use std::env;

/// The parsed command line.
pub struct Args {
    pub check: bool,
    pub port: Option<u16>,
    pub root: Option<String>,
}

/// Parses `--check`, `--port <n>` and `--root <dir>`.
pub fn parse() -> Args {
    let mut args = Args { check: false, port: None, root: None };
    let mut words = env::args().skip(1);
    while let Some(word) = words.next() {
        match word.as_str() {
            "--check" => args.check = true,
            "--port" => args.port = Some(words.next().unwrap_or_default().parse().unwrap()),
            "--root" => args.root = Some(words.next().unwrap()),
            _ => {
                eprintln!("unknown argument {word}");
                std::process::exit(2);
            }
        }
    }
    args
}

/// The current directory, the default root.
pub fn current_dir() -> String {
    env::current_dir().unwrap().display().to_string()
}

/// The name the program was started with.
pub fn program() -> String {
    env::args().next().unwrap()
}
