//! The values of the facets of the matched files (OUTPUT.md#facets).

use std::collections::HashMap;
use std::path::Path;

use super::{SEP, path_parts};

/// The `dir` value of a file: its directory `depth` levels below the working directory, its own directory when it is
/// higher up, `./` in the working directory; with the separator at the end.
#[must_use]
pub fn dir_value(rel: &Path, depth: usize) -> String {
    let parts = path_parts(rel);
    let dirs = &parts[..parts.len().saturating_sub(1)];
    let dirs = &dirs[..dirs.len().min(depth)];
    if dirs.is_empty() {
        format!(".{SEP}")
    } else {
        format!("{}{SEP}", dirs.join(&SEP.to_string()))
    }
}

/// The number of files per value, more files first, then in the order of the values.
pub fn count(values: impl IntoIterator<Item = String>) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for value in values {
        *counts.entry(value).or_default() += 1;
    }
    let mut counts: Vec<(String, usize)> = counts.into_iter().collect();
    counts.sort_by(|(a, n), (b, m)| m.cmp(n).then_with(|| a.cmp(b)));
    counts
}

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
        assert_eq!(
            dir_value(Path::new("tests/integration/x.rs"), 1),
            format!("tests{s}")
        );
    }

    #[test]
    fn more_files_first() {
        let values = ["b", "a", "b", "c", "a", "b"].map(String::from);
        assert_eq!(
            count(values),
            [
                ("b".to_string(), 3),
                ("a".to_string(), 2),
                ("c".to_string(), 1)
            ]
        );
    }

    #[test]
    fn values_over_the_limit() {
        let facet = Facet {
            name: "dir".to_string(),
            values: (1..=10).rev().map(|n| (format!("d{n}/"), n)).collect(),
        };
        assert_eq!(
            facet.line(8),
            "dir:        d10/ 10 · d9/ 9 · d8/ 8 · d7/ 7 · d6/ 6 · d5/ 5 · d4/ 4 · d3/ 3 · … 2 more (3)"
        );
        let short = Facet {
            name: "condition".to_string(),
            values: vec![("#1".to_string(), 3)],
        };
        assert_eq!(short.line(8), "condition:  #1 3");
    }
}
