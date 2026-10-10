//! The order of the files (OUTPUT.md#options, OUTPUT.md#limits-and-truncation).

use std::cmp::Ordering;

use crate::search::Found;

/// The order of the files: `desc` from the highest value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    /// The relevance score (task 93); the path order until then.
    Relevance,
    Path {
        desc: bool,
    },
    Modified {
        desc: bool,
    },
    Size {
        desc: bool,
    },
    Matches {
        desc: bool,
    },
}

impl Sort {
    /// The order of a valid value of `output.sort`.
    #[must_use]
    pub fn parse(value: &str) -> Self {
        let (key, direction) = value.split_once(' ').unwrap_or((value, "asc"));
        let desc = direction == "desc";
        match key {
            "path" => Self::Path { desc },
            "modified" => Self::Modified { desc },
            "size" => Self::Size { desc },
            "matches" => Self::Matches { desc },
            _ => Self::Relevance,
        }
    }

    /// Whether the order needs the metadata of the files: the date, the size, or the kind and the date of the
    /// relevance.
    #[must_use]
    pub fn needs_meta(self) -> bool {
        matches!(
            self,
            Self::Relevance | Self::Modified { .. } | Self::Size { .. }
        )
    }
}

/// Sorts the files, which are in path order, by the value of the order: equal values stay in path order, missing
/// values (no metadata, the size of a directory) come last.
pub fn sort_files(files: &mut [Found], sort: Sort) {
    let (value, desc): (fn(&Found) -> Option<i128>, bool) = match sort {
        Sort::Relevance | Sort::Path { desc: false } => return,
        Sort::Path { desc: true } => {
            files.reverse();
            return;
        }
        Sort::Modified { desc } => (|f| f.meta.as_ref()?.modified.map(i128::from), desc),
        Sort::Size { desc } => (|f| f.meta.as_ref()?.size.map(i128::from), desc),
        Sort::Matches { desc } => (
            |f| {
                let matches = f.matches.as_ref().map_or(0, |m| m.fragments.len());
                Some(i128::try_from(matches).unwrap_or(i128::MAX))
            },
            desc,
        ),
    };
    files.sort_by(|a, b| match (value(a), value(b)) {
        (Some(x), Some(y)) if desc => y.cmp(&x),
        (Some(x), Some(y)) => x.cmp(&y),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::EntryType;
    use crate::search::Meta;

    fn file(path: &str, size: Option<u64>, modified: Option<i64>) -> Found {
        Found {
            rel: path.into(),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: None,
            encoding: None,
            id: None,
            path_fragments: None,
            signals: None,
            score: None,
            meta: Some(Meta {
                entry_type: EntryType::File,
                size,
                lang: None,
                kind: None,
                modified,
            }),
        }
    }

    fn order(files: &[Found], sort: &str) -> Vec<String> {
        let mut files = files.to_vec();
        sort_files(&mut files, Sort::parse(sort));
        files
            .iter()
            .map(|f| f.rel.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn by_the_value_then_by_path() {
        let files = [
            file("a", Some(5), Some(30)),
            file("b", Some(15), Some(20)),
            file("c", Some(10), Some(20)),
            file("d", None, None),
        ];
        assert_eq!(order(&files, "path desc"), ["d", "c", "b", "a"]);
        assert_eq!(order(&files, "size asc"), ["a", "c", "b", "d"]);
        assert_eq!(order(&files, "size desc"), ["b", "c", "a", "d"]);
        assert_eq!(order(&files, "modified desc"), ["a", "b", "c", "d"]);
        assert_eq!(order(&files, "modified asc"), ["b", "c", "a", "d"]);
        assert_eq!(order(&files, "relevance"), ["a", "b", "c", "d"]);
        assert!(Sort::parse("size desc").needs_meta());
        assert!(!Sort::parse("matches desc").needs_meta());
    }
}
