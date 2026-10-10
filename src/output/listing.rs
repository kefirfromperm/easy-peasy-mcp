//! What is listed of the matched files after the repeated lines and the limits (OUTPUT.md#limits-and-truncation).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::query::ast::CondId;
    use crate::search::FileMatches;

    fn file(text: &str) -> Found {
        let content = Content::from_bytes(text.as_bytes());
        let fragments: Vec<Fragment> = content
            .text()
            .match_indices("x\ny")
            .chain(content.text().match_indices('a'))
            .map(|(i, m)| Fragment {
                range: i..i + m.len(),
                ids: vec![(1, CondId::Number(1))],
                escape: None,
            })
            .collect();
        let fragments = crate::output::merge_identical(fragments);
        let lines = crate::output::matched_lines(&content, &fragments);
        Found {
            rel: "f".into(),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: Some(FileMatches {
                content,
                fragments,
                lines,
            }),
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
        }
    }

    fn listed_lines(listing: &Listing, files: &[Found]) -> Vec<Vec<usize>> {
        listing
            .files
            .iter()
            .zip(files)
            .map(|(listed, file)| match listed {
                Listed::Lines(fragments) => {
                    let m = file.matches.as_ref().unwrap();
                    crate::output::lines::lines_of(&m.content, fragments)
                }
                _ => Vec::new(),
            })
            .collect()
    }

    #[test]
    fn the_first_matched_lines() {
        // lines 0, 1 and 3 (the multiline fragment x/y touches 1 and 2)
        let files = [file("a\na\nb\na\n"), file("a\nx\ny\na\n")];
        let limits = Limits {
            max_lines: 4,
            ..Limits::default()
        };
        let listing = Listing::new(&files, &Repeated::default(), &limits);
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1, 3], vec![0]]);
        assert_eq!(listing.truncated_by, ["max_lines"]);
        let limits = Limits {
            max_lines: 5,
            ..Limits::default()
        };
        // the multiline fragment needs lines 1 and 2: it is left out with line 3
        let listing = Listing::new(&files, &Repeated::default(), &limits);
        assert_eq!(listed_lines(&listing, &files), [vec![0, 1, 3], vec![0]]);
        let all = Listing::new(&files, &Repeated::default(), &Limits::default());
        assert_eq!(listed_lines(&all, &files), [vec![0, 1, 3], vec![0, 1, 2, 3]]);
        assert!(all.truncated_by.is_empty());
        let one = Listing::new(
            &files,
            &Repeated::default(),
            &Limits {
                max_lines: 1,
                ..Limits::default()
            },
        );
        assert_eq!(listed_lines(&one, &files), [vec![0], vec![]]);
        assert!(matches!(one.files[1], Listed::Lines(ref f) if f.is_empty()));
    }
}
