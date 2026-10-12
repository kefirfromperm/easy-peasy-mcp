//! Responses written to a file under `server.output_dir` (OUTPUT.md#output-to-a-file).

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
        assert_eq!(resolve(&dir, "r/./s/../t.txt").unwrap(), dir.join("r/t.txt"));
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
        assert_eq!(written.bytes, 14);
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
