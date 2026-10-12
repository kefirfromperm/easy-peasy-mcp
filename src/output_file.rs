//! Responses written to a file under `server.output_dir` (OUTPUT.md#output-to-a-file).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::{ErrorCode, Report};

/// What `output.file` asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileTarget {
    /// A file name, a relative path in the output directory or an absolute path inside it.
    Path(String),
    /// `true`: the server names the file.
    Auto,
}

/// A written file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    /// The absolute canonical path.
    pub path: PathBuf,
    /// The lines of the file, a last line without a line break too.
    pub lines: usize,
    /// The size of the file.
    pub bytes: usize,
}

impl Written {
    /// The line of the summary: `written: <path>  lines: N  bytes: N`.
    #[must_use]
    pub fn line(&self) -> String {
        format!(
            "written: {}  lines: {}  bytes: {}",
            self.path.display(),
            self.lines,
            self.bytes
        )
    }
}

/// What the summary of a response keeps (OUTPUT.md#the-summary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Head {
    /// The text format: the header line of the response and its warnings (an empty line and the warning lines).
    Text { first: String, warnings: String },
    /// JSON: the document with the fields of the summary.
    Json(String),
}

impl Head {
    /// The summary of the response written to a file: the head with the line or the fields of the file.
    #[must_use]
    pub fn summary(&self, written: &Written) -> String {
        match self {
            // a header whose parts are all left out by `output.omit` has no line
            Self::Text { first, warnings } if first.is_empty() => {
                format!("{}\n{warnings}", written.line())
            }
            Self::Text { first, warnings } => format!("{first}\n{}\n{warnings}", written.line()),
            Self::Json(fields) => with_file(fields, written),
        }
    }
}

/// A JSON document with the fields of a written file added at its end: `file`, `lines`, `bytes`.
#[must_use]
pub fn with_file(document: &str, written: &Written) -> String {
    let path = serde_json::to_string(&written.path.to_string_lossy()).expect("a string serializes");
    format!(
        "{},\"file\":{path},\"lines\":{},\"bytes\":{}}}",
        document.strip_suffix('}').unwrap_or(document),
        written.lines,
        written.bytes
    )
}

/// The path of `file` in the output directory `dir`: joined to it unless absolute, `.` and `..` resolved, the
/// existing part canonicalized.
///
/// # Errors
///
/// `output_file_denied`: the path is not inside `dir`.
pub fn resolve(dir: &Path, file: &str) -> Result<PathBuf, Report> {
    let path = crate::paths::canonical_prefix(&dir.join(file));
    if path.starts_with(dir) && path != dir {
        Ok(path)
    } else {
        Err(Report::new(
            ErrorCode::OutputFileDenied,
            format!("the output file '{file}' is outside of the output directory"),
        )
        .hint(format!("files are written to {}", dir.display())))
    }
}

/// Numbers the temporary files of this process.
static TEMPORARY: AtomicUsize = AtomicUsize::new(0);

/// Writes `text` to `path`, creating its directories: to a temporary file next to it which is renamed, so a reader
/// never sees half of it; an existing file is replaced.
///
/// # Errors
///
/// `output_file_failed` with the error of the system as the hint.
pub fn write(path: &Path, text: &str) -> Result<Written, Report> {
    let failed = |error: &std::io::Error| {
        Report::new(
            ErrorCode::OutputFileFailed,
            format!("the output file '{}' can't be written", path.display()),
        )
        .hint(error.to_string())
    };
    let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
        return Err(failed(&std::io::Error::from(
            std::io::ErrorKind::InvalidInput,
        )));
    };
    std::fs::create_dir_all(dir).map_err(|e| failed(&e))?;
    let temporary = dir.join(format!(
        ".{}.{}-{}.tmp",
        name.to_string_lossy(),
        std::process::id(),
        TEMPORARY.fetch_add(1, Ordering::Relaxed)
    ));
    if let Err(error) =
        std::fs::write(&temporary, text).and_then(|()| std::fs::rename(&temporary, path))
    {
        let _ = std::fs::remove_file(&temporary);
        return Err(failed(&error));
    }
    Ok(Written {
        path: path.to_path_buf(),
        lines: line_count(text),
        bytes: text.len(),
    })
}

/// The lines of a text, a last line without a line break too.
fn line_count(text: &str) -> usize {
    text.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output_dir() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let dir = crate::paths::canonicalize(tmp.path()).unwrap().join("out");
        (tmp, dir)
    }

    #[test]
    fn relative_and_absolute_paths_inside() {
        let (_tmp, dir) = output_dir();
        assert_eq!(resolve(&dir, "a.txt").unwrap(), dir.join("a.txt"));
        assert_eq!(
            resolve(&dir, "r/./s/../t.txt").unwrap(),
            dir.join("r/t.txt")
        );
        let absolute = dir.join("x").join("..").join("b.txt");
        assert_eq!(
            resolve(&dir, &absolute.to_string_lossy()).unwrap(),
            dir.join("b.txt")
        );
    }

    #[test]
    fn paths_outside_are_denied() {
        let (tmp, dir) = output_dir();
        for file in [
            "../a.txt".to_string(),
            "r/../../a.txt".to_string(),
            ".".to_string(),
            tmp.path().join("a.txt").to_string_lossy().into_owned(),
        ] {
            let error = resolve(&dir, &file).unwrap_err().to_text();
            assert_eq!(
                error,
                format!(
                    "error[output_file_denied]: the output file '{file}' is outside of the output directory\n  hint: files are written to {}\n",
                    dir.display()
                ),
                "{file}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_symlink_out_of_the_directory_is_denied() {
        let (tmp, dir) = output_dir();
        std::fs::create_dir_all(&dir).unwrap();
        std::os::unix::fs::symlink(tmp.path(), dir.join("link")).unwrap();
        assert!(resolve(&dir, "link/a.txt").is_err());
    }

    #[test]
    fn a_file_is_written_whole_and_replaced() {
        let (_tmp, dir) = output_dir();
        let path = dir.join("r/a.txt");
        let written = write(&path, "one\ntwo ⟦x⟧\n").unwrap();
        assert_eq!(written.lines, 2);
        assert_eq!(written.bytes, 16);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "one\ntwo ⟦x⟧\n");
        let written = write(&path, "{}").unwrap();
        assert_eq!((written.lines, written.bytes), (1, 2));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{}");
        let names: Vec<_> = std::fs::read_dir(dir.join("r"))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(names, ["a.txt"], "no temporary file is left");
        assert_eq!(
            written.line(),
            format!("written: {}  lines: 1  bytes: 2", path.display())
        );
    }

    #[test]
    fn a_file_which_cant_be_written() {
        let (_tmp, dir) = output_dir();
        write(&dir.join("a.txt"), "x").unwrap();
        let error = write(&dir.join("a.txt").join("b.txt"), "x")
            .unwrap_err()
            .to_text();
        assert!(
            error.starts_with(&format!(
                "error[output_file_failed]: the output file '{}' can't be written\n  hint: ",
                dir.join("a.txt").join("b.txt").display()
            )),
            "{error}"
        );
    }

    #[test]
    fn the_lines_of_a_text() {
        assert_eq!(line_count(""), 0);
        assert_eq!(line_count("a"), 1);
        assert_eq!(line_count("a\n"), 1);
        assert_eq!(line_count("a\nb"), 2);
    }
}
