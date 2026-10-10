//! The values of the facets of the matched files (OUTPUT.md#facets).

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn directories_at_a_depth() {
        let s = std::path::MAIN_SEPARATOR;
        assert_eq!(
            dir_value(Path::new("tests/integration/foo/x.rs"), 2),
            format!("tests{s}integration{s}")
        );
        assert_eq!(dir_value(Path::new("src/main.rs"), 2), format!("src{s}"));
        assert_eq!(dir_value(Path::new("README.md"), 2), format!(".{s}"));
        assert_eq!(dir_value(Path::new("tests/integration/x.rs"), 1), format!("tests{s}"));
    }

    #[test]
    fn more_files_first() {
        let values = ["b", "a", "b", "c", "a", "b"].map(String::from);
        assert_eq!(
            count(values),
            [("b".to_string(), 3), ("a".to_string(), 2), ("c".to_string(), 1)]
        );
    }
}
