//! The order of the files (OUTPUT.md#options, OUTPUT.md#limits-and-truncation).

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
