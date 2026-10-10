//! The budget of a response: the detail lowered until the response fits (OUTPUT.md#budget).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{BlockOptions, Detail, Highlight, Limits, Page, Response};
    use crate::query::ast::CondId;
    use crate::search::{FileMatches, Found};
    use crate::warning::Warnings;

    fn file(name: &str, text: &str) -> Found {
        let content = crate::content::Content::from_bytes(text.as_bytes());
        let fragments: Vec<crate::output::Fragment> = content
            .text()
            .match_indices('x')
            .map(|(i, _)| crate::output::Fragment {
                range: i..i + 1,
                ids: vec![(1, CondId::Number(1))],
                escape: None,
            })
            .collect();
        let lines = crate::output::matched_lines(&content, &fragments);
        Found {
            rel: name.into(),
            member: None,
            is_dir: false,
            conditions: vec![CondId::Number(1)],
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
            git: None,
        }
    }

    fn response<'a>(files: &'a [Found], legend: &'a [(CondId, String)], warnings: &'a Warnings) -> Response<'a> {
        Response {
            detail: Detail::Lines,
            blocks: BlockOptions {
                before: 1,
                after: 1,
                ..BlockOptions::default()
            },
            group_dirs: false,
            group_repeated: 0,
            highlight: Highlight::Markers("[", "]"),
            time_zone: jiff::tz::TimeZone::UTC,
            limits: Limits::default(),
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            result_id: None,
            legend,
            files,
            warnings,
            timed_out: false,
        }
    }

    #[test]
    fn the_first_level_which_fits() {
        let long = format!("{}\nx\n{}\n", "a".repeat(200), "b".repeat(200));
        let files: Vec<Found> = (0..12).map(|i| file(&format!("f{i:02}"), &long)).collect();
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        let warnings = Warnings::default();
        let fitted = |budget: usize| fit(response(&files, &legend, &warnings), budget, 4.0, false);
        // without the context lines of 200 characters the lines fit
        let text = fitted(600);
        assert!(text.contains("f00  [#1]  lines: 1  matches: 1\n2:[#1:x]\n"), "{text}");
        assert!(text.starts_with("files: 12  lines: 12  matches: 12  truncated: yes (budget)\n"), "{text}");
        // the list of the files: 143 tokens without the context, 122 without the lines
        let text = fitted(130);
        assert!(text.starts_with("files: 12  lines: 12  matches: 12  detail: files (budget 130)  truncated: yes (budget)\n"), "{text}");
        assert!(!text.contains("2:[#1:x]"), "{text}");
        assert!(text.contains("f11  [#1]"), "all the files");
        // fewer than 10 files don't count: the facets
        let text = fitted(100);
        assert!(text.starts_with("files: 12  lines: 12  matches: 12  detail: facets (budget 100)  truncated: yes (budget)\n"), "{text}");
        // facets which don't fit: only the counts
        let mut big = response(&files, &legend, &warnings);
        big.facets = vec![crate::output::facets::Facet {
            name: "dir".to_string(),
            values: (0..100).map(|i| (format!("directory{i:03}/"), 1)).collect(),
        }];
        big.facet_limit = 100;
        assert_eq!(
            fit(big, 100, 4.0, false),
            "files: 12  lines: 12  matches: 12  detail: count (budget 100)  truncated: yes (budget)\n"
        );
    }
}
