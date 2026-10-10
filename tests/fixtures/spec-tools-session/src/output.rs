//! Writes the results.

use std::io::Write;

/// Prints one result line per file.
pub fn print(files: &[(String, usize)]) {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for (path, hits) in files {
        writeln!(out, "{path}: {hits}").unwrap();
    }
    out.flush().unwrap();
}
