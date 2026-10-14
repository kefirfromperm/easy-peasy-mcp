//! The budget of a response: the detail lowered until the response fits (OUTPUT.md#budget).

use super::{BudgetUse, CountBy, Detail, Response};

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
/// lines, lines cut to 120 characters, the files (as many as fit, at least 10 or all), the facets, the counts (the
/// totals); never above the requested detail; the response at that level. A requested `count` detail with
/// `count_by` other than `total` has the levels of `fit_counts`.
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
    if requested == Detail::Count && response.count_by != CountBy::Total && !fits(&text) {
        return fit_counts(response, budget, &render, &fits);
    }
    if fits(&text) || requested <= Detail::Count {
        return (text, response);
    }
    let lower = |response: &mut Response<'_>, detail: Detail| {
        response.detail = detail;
        response.budget = Some(BudgetUse {
            tokens: budget,
            lowered: detail < requested,
            count_by: None,
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
        if let Some(text) = cut_files(&mut response, &render, &fits) {
            return (text, response);
        }
    }
    lower(&mut response, Detail::Facets.min(requested));
    let text = render(&response);
    if requested >= Detail::Facets && fits(&text) {
        return (text, response);
    }
    lower(&mut response, Detail::Count);
    response.count_by = CountBy::Total;
    (render(&response), response)
}

/// The levels of a requested `count` detail with `count_by` other than `total` which doesn't fit as requested:
/// `count_by = file`, the list of the files cut to the budget, the totals.
fn fit_counts<'a>(
    mut response: Response<'a>,
    budget: usize,
    render: &dyn Fn(&Response<'_>) -> String,
    fits: &dyn Fn(&str) -> bool,
) -> (String, Response<'a>) {
    let requested = response.count_by;
    let lower = |response: &mut Response<'_>, count_by: CountBy| {
        response.count_by = count_by;
        response.budget = Some(BudgetUse {
            tokens: budget,
            lowered: false,
            count_by: (count_by < requested).then_some(count_by),
        });
    };
    lower(&mut response, CountBy::File);
    if requested == CountBy::Line {
        let text = render(&response);
        if fits(&text) {
            return (text, response);
        }
    }
    if let Some(text) = cut_files(&mut response, render, fits) {
        return (text, response);
    }
    lower(&mut response, CountBy::Total);
    (render(&response), response)
}

/// The list of the files cut to the most files which fit, if they are at least 10 or all of them; `None` otherwise,
/// with the list not cut.
fn cut_files(
    response: &mut Response<'_>,
    render: &dyn Fn(&Response<'_>) -> String,
    fits: &dyn Fn(&str) -> bool,
) -> Option<String> {
    let all = response.listed().len();
    // the whole list first: a list cut by the budget has a next cursor, so it can be longer with fewer files
    response.limits.budget_files = all;
    let (mut low, mut high) = if fits(&render(response)) {
        (all, all)
    } else {
        (0, all.saturating_sub(1))
    };
    // the most files which fit, searched in halves
    while low < high {
        let middle = (low + high).div_ceil(2);
        response.limits.budget_files = middle;
        if fits(&render(response)) {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    if low == all || low >= MIN_FILES {
        response.limits.budget_files = low;
        return Some(render(response));
    }
    response.limits.budget_files = usize::MAX;
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{BlockOptions, CountBy, Detail, Highlight, Limits, Page, Response};
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
                groups: Vec::new(),
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
            count_by: CountBy::Total,
            blocks: BlockOptions {
                before: 1,
                after: 1,
                ..BlockOptions::default()
            },
            group_dirs: false,
            group_repeated: 0,
            repeated_locations: 3,
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
            omit: crate::output::Omit::default(),
            in_dirs: None,
            spec: 1,
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
        // the count level of a lowered detail is the totals, whatever count_by is
        big.count_by = CountBy::Line;
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

    #[test]
    fn the_levels_of_the_counts() {
        let files: Vec<Found> = (0..30).map(|i| file(&format!("f{i:02}"), "x\n")).collect();
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        let warnings = Warnings::default();
        let fitted = |budget: usize| {
            let mut counts = response(&files, &legend, &warnings);
            counts.detail = Detail::Count;
            counts.count_by = CountBy::Line;
            counts.page.hash = Some("0123abcd".to_string());
            fit(counts, budget, 4.0, false).0
        };
        // 30 × "f00  lines: 1  matches: 1\n  1: 1\n" and the header: 257 tokens
        let text = fitted(260);
        assert!(
            text.starts_with("files: 30  lines: 30  matches: 30\n"),
            "{text}"
        );
        assert!(
            text.contains("f29  lines: 1  matches: 1\n  1: 1\n"),
            "{text}"
        );
        // the counts of the files: 218 tokens
        let text = fitted(230);
        assert!(text.starts_with("files: 30  lines: 30  matches: 30  count by: file (budget 230)  truncated: yes (budget)\n"), "{text}");
        assert!(
            text.contains("f29  lines: 1  matches: 1\n") && !text.contains("1: 1"),
            "{text}"
        );
        // 16 of the files with a cursor
        let text = fitted(150);
        assert!(
            text.contains("count by: file (budget 150)  truncated: yes (budget)  next cursor: "),
            "{text}"
        );
        assert!(text.contains("f15  ") && !text.contains("f16  "), "{text}");
        // fewer than ten files fit: the totals
        assert_eq!(
            fitted(100),
            "files: 30  lines: 30  matches: 30  count by: total (budget 100)  truncated: yes (budget)\n"
        );
    }
}
