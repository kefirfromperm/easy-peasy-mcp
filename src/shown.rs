//! The matches of some groups of the query (OUTPUT.md#shown-groups): the fragments of the found files replaced by
//! the matches of the groups of `output.groups`.

use crate::error::Report;
use crate::output::{Fragment, matched_lines, merge_identical};
use crate::query::Query;
use crate::query::ast::CondId;
use crate::search::Found;

/// The checked references of `output.groups`: the whole groups and the sub-groups whose matches are shown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Shown {
    /// The ids of the whole groups.
    whole: Vec<CondId>,
    /// The sub-groups as their matches list them: `@def.1`, `@def.fname`.
    subs: Vec<String>,
}

impl Shown {
    /// The references of `list` checked against the query, in their order; `None` for an empty list, which shows
    /// every match.
    ///
    /// # Errors
    ///
    /// `unknown_group` for the first reference the query doesn't have, with its position in the reference.
    pub(crate) fn of(query: &Query, list: &[String]) -> Result<Option<Self>, Report> {
        if list.is_empty() {
            return Ok(None);
        }
        let mut shown = Self {
            whole: Vec::new(),
            subs: Vec::new(),
        };
        for reference in list {
            let group = query
                .group(reference)
                .map_err(|e| e.into_field_report("output.groups", reference))?;
            if group.sub.is_some() {
                shown.subs.push(group.to_string());
            } else {
                shown.whole.push(group.id);
            }
        }
        Ok(Some(shown))
    }

    /// The number of distinct references: the ids of the matches are written with more than one
    /// (OUTPUT.md#defaults-of-spec-2).
    pub(crate) fn references(&self) -> usize {
        let mut whole: Vec<&CondId> = self.whole.iter().collect();
        whole.sort_by_key(ToString::to_string);
        whole.dedup();
        let mut subs: Vec<&String> = self.subs.iter().collect();
        subs.sort();
        subs.dedup();
        whole.len() + subs.len()
    }

    /// Replaces the fragments and the path fragments of the files by the shown matches; a file without a shown
    /// match keeps no matches, so it is listed without counts and lines.
    pub(crate) fn apply(&self, files: &mut [Found]) {
        for file in files {
            if let Some(m) = file.matches.as_mut() {
                let fragments = merge_identical(
                    m.fragments
                        .iter()
                        .flat_map(|f| self.matches_of(f))
                        .collect(),
                );
                m.lines = matched_lines(&m.content, &fragments);
                m.fragments = fragments;
            }
            if file
                .matches
                .as_ref()
                .is_some_and(|m| m.fragments.is_empty())
            {
                file.matches = None;
            }
            if let Some(path) = file.path_fragments.as_mut() {
                path.fragments = path
                    .fragments
                    .iter()
                    .filter_map(|f| self.whole_match(f))
                    .collect();
                if path.fragments.is_empty() {
                    file.path_fragments = None;
                }
            }
        }
    }

    /// The shown matches of a fragment: itself with the ids of the whole groups of the list, and the matches of its
    /// sub-groups in the list.
    fn matches_of(&self, fragment: &Fragment) -> Vec<Fragment> {
        let mut out: Vec<Fragment> = self.whole_match(fragment).into_iter().collect();
        for group in &fragment.groups {
            let ids: Vec<(usize, CondId)> = group
                .ids
                .iter()
                .filter(|reference| self.subs.contains(reference))
                .filter_map(|reference| {
                    // the order of the condition of the sub-group, so it follows its condition
                    let (order, _) = fragment.ids.iter().find(|(_, id)| {
                        reference
                            .strip_prefix(&id.to_string())
                            .is_some_and(|rest| rest.starts_with('.'))
                    })?;
                    Some((*order, CondId::Sub(reference.clone())))
                })
                .collect();
            if !ids.is_empty() {
                out.push(Fragment {
                    range: group.range.clone(),
                    ids,
                    escape: fragment.escape,
                    groups: Vec::new(),
                });
            }
        }
        out
    }

    /// The fragment with only the ids of the whole groups of the list, if it has any.
    fn whole_match(&self, fragment: &Fragment) -> Option<Fragment> {
        let ids: Vec<(usize, CondId)> = fragment
            .ids
            .iter()
            .filter(|(_, id)| self.whole.contains(id))
            .cloned()
            .collect();
        (!ids.is_empty()).then(|| Fragment {
            ids,
            ..fragment.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::output::{Fragment, SubMatch};
    use crate::query::ast::CondId;
    use crate::query::{ParseOptions, parse};
    use crate::search::{FileMatches, Found, PathFragments};

    fn options() -> ParseOptions {
        ParseOptions {
            case_insensitive_paths: false,
            now: jiff::Timestamp::UNIX_EPOCH,
            time_zone: jiff::tz::TimeZone::UTC,
            languages: std::sync::Arc::new(crate::languages::LanguageTable::builtin()),
            backslash_separator: false,
            max_escape_level: 3,
            fuzzy_min_score: 0.5,
        }
    }

    fn shown(query: &str, list: &[&str]) -> Result<Option<Shown>, String> {
        let query = parse(query, &options()).unwrap();
        let list: Vec<String> = list.iter().map(ToString::to_string).collect();
        Shown::of(&query, &list).map_err(|r| r.to_text())
    }

    fn label(l: &str) -> CondId {
        CondId::Label(l.into())
    }

    fn found(text: &str, fragments: Vec<Fragment>) -> Found {
        let content = Content::from_bytes(text.as_bytes());
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
            git: None,
            root: 0,
        }
    }

    fn fragment(range: std::ops::Range<usize>, ids: &[(usize, CondId)]) -> Fragment {
        Fragment {
            range,
            ids: ids.to_vec(),
            escape: None,
            groups: Vec::new(),
        }
    }

    fn shown_fragments(file: &Found) -> Vec<(std::ops::Range<usize>, Vec<String>)> {
        file.matches
            .iter()
            .flat_map(|m| &m.fragments)
            .map(|f| {
                (
                    f.range.clone(),
                    f.ids.iter().map(|(_, id)| id.to_string()).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn an_empty_list_shows_everything() {
        assert!(shown("text has 'a' @a", &[]).unwrap().is_none());
    }

    #[test]
    fn the_references_are_checked_in_order() {
        assert_eq!(
            shown("text has 'a' @a", &["@a", "@x", "@y"]).err().unwrap(),
            "error[unknown_group]: unknown group '@x'\n  @x\n  ^^\n  hint: the groups of the query: @a\n"
        );
        let json = {
            let query = parse("text =~ `(b)` @r", &options()).unwrap();
            Shown::of(&query, &["@r.2".to_string()])
                .err()
                .unwrap()
                .to_json()
        };
        assert!(json.contains(r#""output.groups":"@r.2""#), "{json}");
        assert_eq!(
            shown("text =~ `(b)` @r", &["@r.1", "@r"])
                .unwrap()
                .unwrap()
                .references(),
            2
        );
    }

    #[test]
    fn whole_groups_keep_only_their_ids() {
        let shown = shown(
            "(text has 'ab' or text has 'abc') @g and text has 'c' @c",
            &["@g"],
        )
        .unwrap()
        .unwrap();
        let mut files = vec![found(
            "abc c",
            vec![
                fragment(0..2, &[(1, label("g")), (2, CondId::Number(1))]),
                fragment(0..3, &[(1, label("g")), (3, CondId::Number(2))]),
                fragment(2..3, &[(4, label("c"))]),
                fragment(4..5, &[(4, label("c"))]),
            ],
        )];
        shown.apply(&mut files);
        assert_eq!(
            shown_fragments(&files[0]),
            vec![(0..2, vec!["@g".into()]), (0..3, vec!["@g".into()])]
        );
        assert_eq!(files[0].matches.as_ref().unwrap().lines, 1);
    }

    #[test]
    fn a_sub_group_is_a_match_of_its_own() {
        let shown = shown("text =~ `fn (?<name>\\w+)` @def", &["@def", "@def.name"])
            .unwrap()
            .unwrap();
        let mut whole = fragment(0..6, &[(1, label("def"))]);
        whole.groups = vec![SubMatch {
            range: 3..6,
            ids: vec!["@def.1".into(), "@def.name".into()],
        }];
        let mut files = vec![found("fn abc", vec![whole])];
        shown.apply(&mut files);
        assert_eq!(
            shown_fragments(&files[0]),
            vec![
                (0..6, vec!["@def".into()]),
                (3..6, vec!["@def.name".into()])
            ]
        );
        // the whole group keeps its sub-groups, the match of a sub-group has none
        let m = files[0].matches.as_ref().unwrap();
        assert_eq!(m.fragments[0].groups.len(), 1);
        assert!(m.fragments[1].groups.is_empty());
    }

    #[test]
    fn a_numbered_sub_group_is_written_as_its_number() {
        let shown = shown("text =~ `fn (\\w+)`", &["#1.01"]).unwrap().unwrap();
        let mut whole = fragment(0..6, &[(1, CondId::Number(1))]);
        whole.groups = vec![SubMatch {
            range: 3..6,
            ids: vec!["#1.1".into()],
        }];
        let mut files = vec![found("fn abc", vec![whole])];
        shown.apply(&mut files);
        assert_eq!(
            shown_fragments(&files[0]),
            vec![(3..6, vec!["#1.1".into()])]
        );
    }

    #[test]
    fn a_file_without_a_shown_match_has_no_matches() {
        let shown = shown("text has 'a' @a or text has 'b' @b", &["@a"])
            .unwrap()
            .unwrap();
        let mut files = vec![found("b", vec![fragment(0..1, &[(2, label("b"))])])];
        shown.apply(&mut files);
        assert!(files[0].matches.is_none());
    }

    #[test]
    fn path_fragments_of_the_groups() {
        let shown = shown("name has 'a' @a and name has 'b' @b", &["@b"])
            .unwrap()
            .unwrap();
        let mut file = found("", Vec::new());
        file.matches = None;
        file.path_fragments = Some(PathFragments {
            path: "ab".into(),
            fragments: vec![
                fragment(0..1, &[(1, label("a"))]),
                fragment(1..2, &[(2, label("b"))]),
            ],
        });
        let mut files = vec![file];
        shown.apply(&mut files);
        let path = files[0].path_fragments.as_ref().unwrap();
        assert_eq!(path.fragments, vec![fragment(1..2, &[(2, label("b"))])]);
        let shown_a = super::tests::shown("name has 'a' @a and name has 'b' @b", &["@a"])
            .unwrap()
            .unwrap();
        files[0].path_fragments.as_mut().unwrap().fragments.clear();
        shown_a.apply(&mut files);
        assert!(files[0].path_fragments.is_none());
    }
}
