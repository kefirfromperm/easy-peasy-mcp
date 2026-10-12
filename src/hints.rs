//! The hints of an empty result (OUTPUT.md#hints): variations of the condition where the funnel dropped to 0, the
//! skipped files which could match, similar names and directories. Every hint is a search of its own, within the
//! time budget of the hints.

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Instant;

use crate::output::{Hint, display_found, facets};
use crate::query::{
    self, FunnelKind, Kind, Lookup, ParseOptions, Query, ast::Target, edit_distance,
};
use crate::search::{self, Binary, Found, SearchOptions, slash_path};

/// The time budget of the hints is over.
struct OutOfTime;

/// A change of the options of a search.
type Relax = fn(&mut SearchOptions);

/// What the hints of a request need.
pub struct Context<'a> {
    /// The working directories and their prefixes.
    pub working_dirs: &'a [(PathBuf, PathBuf)],
    pub parse: &'a ParseOptions,
    /// The options of the search of the request.
    pub search: &'a SearchOptions,
    pub query: &'a Query,
    /// The numbers of files of the steps of the funnel.
    pub counts: &'a [usize],
    /// When the hints must stop.
    pub deadline: Instant,
}

/// The hints of an empty result, as many as fit in the time budget.
pub fn empty(cx: &Context<'_>) -> Vec<Hint> {
    let Some(funnel) = &cx.query.funnel else {
        return Vec::new();
    };
    // the steps whose conditions are varied, and the steps which are searched with them
    let varied: Vec<(usize, &[query::FunnelStep])> = match funnel.kind {
        FunnelKind::And => cx
            .counts
            .iter()
            .position(|n| *n == 0)
            .map(|zero| vec![(zero, &funnel.steps[..zero])])
            .unwrap_or_default(),
        FunnelKind::Or | FunnelKind::One => (0..funnel.steps.len()).map(|i| (i, &[][..])).collect(),
    };
    let mut hints = Vec::new();
    // the hints found before the time is over
    let _ = collect(cx, funnel, &varied, &mut hints);
    hints
}

/// Adds the hints to `hints` until the time is over.
fn collect(
    cx: &Context<'_>,
    funnel: &query::Funnel,
    varied: &[(usize, &[query::FunnelStep])],
    hints: &mut Vec<Hint>,
) -> Result<(), OutOfTime> {
    for &(step, before) in varied {
        for variation in &funnel.steps[step].variations {
            hints.extend(variation_hint(
                cx,
                before,
                variation,
                funnel.steps[step].target,
            )?);
        }
    }
    hints.extend(skipped(cx)?);
    for &(step, _) in varied {
        let step = &funnel.steps[step];
        if let Some(lookup) = &step.lookup {
            hints.extend(similar(cx, &step.condition, lookup)?);
        }
    }
    Ok(())
}

/// The options of a hint search: the options of the request with the deadline of the hints, without what only the
/// response needs.
fn options(cx: &Context<'_>) -> SearchOptions {
    let mut options = cx.search.clone();
    options.deadline = Some(cx.deadline);
    options.funnel = false;
    options.rank = false;
    options.meta = false;
    options.path_fragments = false;
    options.first_only = false;
    options
}

/// The files of a query.
fn search(
    cx: &Context<'_>,
    query: &Query,
    options: &SearchOptions,
) -> Result<Vec<Found>, OutOfTime> {
    if Instant::now() >= cx.deadline {
        return Err(OutOfTime);
    }
    let result = crate::engine::search_all(cx.working_dirs, query, options);
    if result.timed_out {
        Err(OutOfTime)
    } else {
        Ok(result.files)
    }
}

/// A variation searched with the steps before it: the hint when it matches.
fn variation_hint(
    cx: &Context<'_>,
    before: &[query::FunnelStep],
    variation: &str,
    target: Option<Target>,
) -> Result<Option<Hint>, OutOfTime> {
    let text: Vec<&str> = before
        .iter()
        .map(|s| s.condition.as_str())
        .chain([variation])
        .collect();
    let Ok(query) = query::parse(&text.join(" and "), cx.parse) else {
        return Ok(None);
    };
    let files = search(cx, &query, &options(cx))?;
    if files.is_empty() {
        return Ok(None);
    }
    let id = query.legend.last().map(|(id, _)| id.clone());
    let mut examples: Vec<String> = Vec::new();
    for file in &files {
        let found: Vec<String> = match target {
            Some(Target::Text) => file
                .matches
                .iter()
                .flat_map(|m| {
                    m.fragments
                        .iter()
                        .filter(|f| f.ids.iter().any(|(_, i)| Some(i) == id.as_ref()))
                        .map(|f| m.content.text()[f.range.clone()].to_string())
                })
                .collect(),
            Some(Target::Name) => vec![name_of(file)],
            _ => vec![display_found(file)],
        };
        for example in found {
            if examples.len() < 3 && !examples.contains(&example) {
                examples.push(example);
            }
        }
    }
    Ok(Some(Hint::Variation {
        condition: variation.to_string(),
        files: files.len(),
        examples,
    }))
}

/// The name of a found entry: the last part of its path or of its member path.
fn name_of(file: &Found) -> String {
    match &file.member {
        Some(member) => member.rsplit('/').next().unwrap_or(member).to_string(),
        None => file
            .rel
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
    }
}

/// `skipped:` the files which match when one setting is relaxed.
fn skipped(cx: &Context<'_>) -> Result<Option<Hint>, OutOfTime> {
    let relaxed: [(&str, Relax); 4] = [
        ("ignored (traversal.ignore_files)", |o| {
            o.ignore_files.clear();
            o.ignore_paths.clear();
            o.git_excludes = false;
            o.git_global_excludes = false;
        }),
        ("generated (traversal.skip_generated)", |o| {
            o.skip_generated = false;
        }),
        ("too big (traversal.max_file_size)", |o| {
            o.max_file_size = f64::INFINITY;
        }),
        ("binary (traversal.binary)", |o| {
            o.binary = Binary::Search;
            o.decode.binary_as_text = true;
        }),
    ];
    let mut parts = Vec::new();
    for (what, relax) in relaxed {
        let mut options = options(cx);
        relax(&mut options);
        let files = search(cx, cx.query, &options)?;
        if !files.is_empty() {
            parts.push(format!("{} {what}", files.len()));
        }
    }
    Ok((!parts.is_empty()).then(|| Hint::Line(format!("skipped: {}", parts.join(", ")))))
}

/// Similar names of `name = '...'`, similar directories of a missing directory of `path`.
fn similar(cx: &Context<'_>, condition: &str, lookup: &Lookup) -> Result<Option<Hint>, OutOfTime> {
    let (text, wanted) = match lookup {
        Lookup::Name(name) => ("", name.as_str()),
        Lookup::Dir(dir) => ("type = dir", dir.as_str()),
    };
    let Ok(query) = query::parse(text, cx.parse) else {
        return Ok(None);
    };
    let mut candidates: Vec<String> = Vec::new();
    let mut seen = HashSet::new();
    for (path, _) in cx.working_dirs {
        if Instant::now() >= cx.deadline {
            return Err(OutOfTime);
        }
        let result = search::search(path, &query, &options(cx));
        if result.timed_out {
            return Err(OutOfTime);
        }
        for file in result.files {
            let candidate = match lookup {
                Lookup::Name(_) => name_of(&file),
                Lookup::Dir(_) => slash_path(&file.rel),
            };
            if seen.insert(candidate.clone()) {
                candidates.push(candidate);
            }
        }
    }
    if matches!(lookup, Lookup::Dir(_)) && candidates.iter().any(|c| c == wanted) {
        return Ok(None);
    }
    let mut close: Vec<(usize, String)> = candidates
        .into_iter()
        .filter_map(|c| {
            let distance = edit_distance(wanted, &c);
            (distance > 0 && distance <= c.chars().count() / 3).then_some((distance, c))
        })
        .collect();
    close.sort_by_key(|(distance, _)| *distance);
    let shown: Vec<String> = close
        .into_iter()
        .take(3)
        .map(|(_, c)| match lookup {
            Lookup::Name(_) => c,
            Lookup::Dir(_) => format!("{c}/"),
        })
        .collect();
    Ok(match lookup {
        Lookup::Name(_) if shown.is_empty() => None,
        Lookup::Name(_) => Some(Hint::Line(format!(
            "{condition}: similar: {}",
            shown.join(", ")
        ))),
        Lookup::Dir(_) if shown.is_empty() => {
            Some(Hint::Line(format!("{condition}: no such directory")))
        }
        Lookup::Dir(_) => Some(Hint::Line(format!(
            "{condition}: no such directory, similar: {}",
            shown.join(", ")
        ))),
    })
}

/// The values of a facet in the hints of a huge result.
const HINT_VALUES: usize = 4;

/// The hints of a huge result: the `dir` and `lang` values with the most files and how to narrow the query.
pub fn huge(files: &[Found], query: &Query, facet_depth: usize) -> Vec<Hint> {
    let top = |name: &str, counts: Vec<(String, usize)>| {
        (!counts.is_empty()).then(|| {
            let values: Vec<String> = counts
                .into_iter()
                .take(HINT_VALUES)
                .map(|(value, n)| format!("{value} {n}"))
                .collect();
            Hint::Line(format!("{name}: {}", values.join(" · ")))
        })
    };
    let mut hints = Vec::new();
    hints.extend(top(
        "dir",
        facets::count(files.iter().map(|f| facets::dir_value(&f.rel, facet_depth))),
    ));
    hints.extend(top(
        "lang",
        facets::count(files.iter().filter_map(|f| f.meta.as_ref()?.lang.clone())),
    ));
    let narrow = narrow(files, query);
    if !narrow.is_empty() {
        hints.push(Hint::Line(format!("narrow with: {}", narrow.join(", "))));
    }
    hints
}

/// The suggestions which narrow a huge result: leaving out the directory with a third of the files or more, `kind =
/// source`, `/w`.
fn narrow(files: &[Found], query: &Query) -> Vec<String> {
    let total = files.len();
    let mut out = Vec::new();
    let here = facets::dir_value(std::path::Path::new("x"), 1);
    let first_dirs = facets::count(
        files
            .iter()
            .map(|f| facets::dir_value(&f.rel, 1))
            .filter(|dir| *dir != here)
            .map(|dir| dir[..dir.len() - 1].replace(std::path::MAIN_SEPARATOR, "/")),
    );
    if let Some((dir, n)) = first_dirs.first()
        && n * 3 >= total
        && *n < total
    {
        out.push(format!("not path ~ '{dir}/**'"));
    }
    let other_kinds = files
        .iter()
        .filter(|f| {
            f.meta
                .as_ref()
                .is_some_and(|m| m.kind != Some(Kind::Source))
        })
        .count();
    if !query.narrowing.kind && other_kinds * 10 >= total && other_kinds > 0 {
        out.push("kind = source".to_string());
    }
    if query.narrowing.whole_word {
        out.push("/w".to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(scores: &[(&str, f64)]) -> Vec<(String, f64)> {
        scores.iter().map(|(f, s)| ((*f).to_string(), *s)).collect()
    }

    #[test]
    fn below_the_threshold_best_first() {
        let hint = below_hint(
            "name ~~ 'e'",
            0.5,
            files(&[
                ("README.md", 0.0822),
                ("parse.rs", 0.0825),
                ("parser.rs", 0.0822),
                ("lexer.rs", 0.0825),
            ]),
        );
        assert_eq!(
            hint,
            Some(Hint::Line(
                "name ~~ 'e': below query_options.fuzzy_min_score 0.5: parse.rs 0.08, lexer.rs 0.08, README.md 0.08"
                    .to_string()
            ))
        );
    }

    #[test]
    fn scores_rounded_down() {
        let hint = below_hint("path ~~ 'x'", 1.0, files(&[("a", 0.999), ("b", 0.5488)]));
        assert_eq!(
            hint,
            Some(Hint::Line(
                "path ~~ 'x': below query_options.fuzzy_min_score 1: a 0.99, b 0.54".to_string()
            ))
        );
    }

    #[test]
    fn nothing_below() {
        assert_eq!(below_hint("name ~~ 'x'", 0.5, Vec::new()), None);
    }
}
