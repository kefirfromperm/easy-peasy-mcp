//! easy-interp: a small interpreter for the `.ez` scripting language.

mod cache;
mod cli;
mod config;
mod error;
mod eval;
mod lexer;
mod parser;

use clap::Parser;

use crate::cli::Args;
use crate::error::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = config::load(&args.config).unwrap();
    run(cfg)
}

/// Runs the script of the configuration, or the REPL when it names none.
fn run(cfg: config::Config) -> Result<()> {
    match &cfg.script {
        Some(path) => run_file(path, &cfg),
        None => repl(&cfg),
    }
}

/// Parses and evaluates one script file.
fn run_file(path: &std::path::Path, cfg: &config::Config) -> Result<()> {
    let source = std::fs::read_to_string(path)?;
    let program = parser::parse(&source)?;
    let mut interp = eval::Interpreter::new(cfg);
    let value = interp.run(&program)?;
    if cfg.trace {
        eprintln!("result: {value}");
    }
    Ok(())
}

/// Reads one statement per line and prints its value.
fn repl(cfg: &config::Config) -> Result<()> {
    use std::io::{BufRead, Write};

    let mut interp = eval::Interpreter::new(cfg);
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    write!(out, "> ")?;
    out.flush()?;
    for line in stdin.lines() {
        let line = line?;
        match parser::parse(&line).and_then(|program| interp.run(&program)) {
            Ok(value) => writeln!(out, "{value}")?,
            Err(e) => print_error(&line, &e),
        }
        write!(out, "> ")?;
        out.flush()?;
    }
    Ok(())
}

/// Prints an error with the line it happened in.
// TODO: show the column of the error with a caret
fn print_error(line: &str, error: &error::Error) {
    eprintln!("error: {error}");
    eprintln!("  {line}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_binary_has_a_name() {
        assert_eq!(env!("CARGO_PKG_NAME"), "easy-interp");
    }
}
