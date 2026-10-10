//! Canonical paths in the form the agent reads: on Windows without the `\\?\` prefix where the path means the same
//! without it (OUTPUT.md#paths).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prefix_is_dropped_where_the_path_means_the_same() {
        assert_eq!(
            simplified(r"\\?\C:\Users\me\p").as_deref(),
            Some(r"C:\Users\me\p")
        );
        assert_eq!(simplified(r"\\?\c:\").as_deref(), Some(r"c:\"));
        assert_eq!(
            simplified(r"\\?\UNC\server\share\p").as_deref(),
            Some(r"\\server\share\p")
        );
    }

    #[test]
    fn the_prefix_stays_where_it_is_needed() {
        for verbatim in [
            r"C:\Users",
            r"\\?\C:\a\CON",
            r"\\?\C:\a\nul.txt",
            r"\\?\C:\a\b.",
            r"\\?\C:\a\b ",
            r"\\?\C:\a\..\b",
            r"\\?\C:\a\.\b",
            r"\\?\C:\a/b",
            r"\\?\C:",
            r"\\?\Volume{1234}\a",
            r"\\?\UNC\server",
        ] {
            assert_eq!(simplified(verbatim), None, "{verbatim}");
        }
        let long = format!(r"\\?\C:\{}", "a".repeat(300));
        assert_eq!(simplified(&long), None);
    }

    #[test]
    #[cfg(not(windows))]
    fn elsewhere_the_canonical_path_of_the_standard_library() {
        let path = std::path::Path::new(".");
        assert_eq!(
            canonicalize(path).unwrap(),
            std::fs::canonicalize(path).unwrap()
        );
    }
}
