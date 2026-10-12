//! The budget of a response: the detail lowered until the response fits (OUTPUT.md#budget).

use super::{BudgetUse, Detail, Response};

/// The length of the lines at the third level.
const CUT_LINES: usize = 120;

/// The files which have to fit at the level of the files, unless they are all.
const MIN_FILES: usize = 10;

/// The estimated tokens of a text: its characters divided by `chars_per_token`, rounded up.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn tokens(text: &str, chars_per_token: f64) -> usize {
    (text.chars().count() as f64 / chars_per_token).ceil() as usize
}

/// The response written in its format at the first level of the budget it fits in: as requested, without context
/// lines, lines cut to 120 characters, the files (as many as fit, at least 10 or all), the facets, the counts; never
/// above the requested detail; the response at that level.
pub fn fit(
    mut response: Response<'_>,
    budget: usize,
    chars_per_token: f64,
    json: bool,
) -> (String, Response<'_>) {
    let render = |r: &Response<'_>| if json { r.to_json() } else { r.to_text() };
    let fits = |text: &str| tokens(text, chars_per_token) <= budget;
    let text = render(&response);
    let requested = response.detail;
    if fits(&text) || requested <= Detail::Count {
        return (text, response);
    }
    let lower = |response: &mut Response<'_>, detail: Detail| {
        response.detail = detail;
        response.budget = Some(BudgetUse {
            tokens: budget,
            lowered: detail < requested,
        });
    };
    if requested == Detail::Lines {
        lower(&mut response, Detail::Lines);
        response.blocks.before = 0;
        response.blocks.after = 0;
        let text = render(&response);
        if fits(&text) {
            return (text, response);
        }
        response.limits.max_line_length = response.limits.max_line_length.min(CUT_LINES);
        let text = render(&response);
        if fits(&text) {
            return (text, response);
        }
    }
    if requested >= Detail::Files {
        lower(&mut response, Detail::Files);
        let all = response.listed().len();
        // the whole list first: a list cut by the budget has a next cursor, so it can be longer with fewer files
        response.limits.budget_files = all;
        let (mut low, mut high) = if fits(&render(&response)) {
            (all, all)
        } else {
            (0, all.saturating_sub(1))
        };
        // the most files which fit, searched in halves
        while low < high {
            let middle = (low + high).div_ceil(2);
            response.limits.budget_files = middle;
            if fits(&render(&response)) {
                low = middle;
            } else {
                high = middle - 1;
            }
        }
        if low == all || low >= MIN_FILES {
            response.limits.budget_files = low;
            return (render(&response), response);
        }
        response.limits.budget_files = usize::MAX;
    }
    lower(&mut response, Detail::Facets.min(requested));
    let text = render(&response);
    if requested >= Detail::Facets && fits(&text) {
        return (text, response);
    }
    lower(&mut response, Detail::Count);
    (render(&response), response)
}

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
            root: 0,
        }
    }

    fn response<'a>(
        files: &'a [Found],
        legend: &'a [(CondId, String)],
        warnings: &'a Warnings,
    ) -> Response<'a> {
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
            since: None,
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
        let fitted =
            |budget: usize| fit(response(&files, &legend, &warnings), budget, 4.0, false).0;
        // without the context lines of 200 characters the lines fit
        let text = fitted(600);
        assert!(
            text.contains("f00  [#1]  lines: 1  matches: 1\n2:[#1:x]\n"),
            "{text}"
        );
        assert!(
            text.starts_with("files: 12  lines: 12  matches: 12  truncated: yes (budget)\n"),
            "{text}"
        );
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
            fit(big, 100, 4.0, false).0,
            "files: 12  lines: 12  matches: 12  detail: count (budget 100)  truncated: yes (budget)\n"
        );
    }

    #[test]
    fn the_whole_list_without_a_cursor() {
        let files: Vec<Found> = (0..11).map(|i| file(&format!("f{i:02}"), "x\n")).collect();
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        let warnings = Warnings::default();
        let mut paged = response(&files, &legend, &warnings);
        paged.page.hash = Some("0123abcd".to_string());
        // all 11 files fit; 10 or 9 don't, with the next cursor of the cut list
        let text = fit(paged, 116, 4.0, false).0;
        assert!(text.starts_with("files: 11  lines: 11  matches: 11  detail: files (budget 116)  truncated: yes (budget)\n"), "{text}");
        assert!(text.contains("f10  [#1]"), "all the files: {text}");
    }
}
