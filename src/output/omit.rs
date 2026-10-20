//! The parts of a response which `output.omit` leaves out (OUTPUT.md#omit).

use std::collections::HashSet;

use super::lines::line_counts;
use crate::config::KeyValue;
use crate::query::ast::CondId;
use crate::search::Found;

/// A part of a response which can be left out, in the order of the table of OUTPUT.md#omit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Counts,
    Result,
    Dir,
    Legend,
    Funnel,
    Hints,
    Conditions,
    FileCounts,
    MatchCounts,
    MatchIds,
    Groups,
    LineNumbers,
    LineCounts,
    Gaps,
    Indent,
    MoreLines,
    Locations,
    Warnings,
    LeftOut,
}

impl Part {
    /// All parts in the order of the spec.
    pub const ALL: [Part; 19] = [
        Self::Counts,
        Self::Result,
        Self::Dir,
        Self::Legend,
        Self::Funnel,
        Self::Hints,
        Self::Conditions,
        Self::FileCounts,
        Self::MatchCounts,
        Self::MatchIds,
        Self::Groups,
        Self::LineNumbers,
        Self::LineCounts,
        Self::Gaps,
        Self::Indent,
        Self::MoreLines,
        Self::Locations,
        Self::Warnings,
        Self::LeftOut,
    ];

    /// The name in `output.omit`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Counts => "counts",
            Self::Result => "result",
            Self::Dir => "dir",
            Self::Legend => "legend",
            Self::Funnel => "funnel",
            Self::Hints => "hints",
            Self::Conditions => "conditions",
            Self::FileCounts => "file_counts",
            Self::MatchCounts => "match_counts",
            Self::MatchIds => "match_ids",
            Self::Groups => "groups",
            Self::LineNumbers => "line_numbers",
            Self::LineCounts => "line_counts",
            Self::Gaps => "gaps",
            Self::Indent => "indent",
            Self::MoreLines => "more_lines",
            Self::Locations => "locations",
            Self::Warnings => "warnings",
            Self::LeftOut => "left_out",
        }
    }

    /// The part of a name.
    #[must_use]
    pub fn find(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.name() == name)
    }
}

/// What tells the files and the matches of a result apart (OUTPUT.md#the-same-everywhere): different conditions of
/// two files, different ids of two matches; and whether a matched line has another number of matches than one
/// (OUTPUT.md#one-match-per-line).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Differ {
    pub conditions: bool,
    pub match_ids: bool,
    pub match_counts: bool,
}

impl Differ {
    /// What differs between all the files of a result.
    #[must_use]
    pub fn of(files: &[Found]) -> Self {
        let conditions = files.iter().map(|f| ids(f.conditions.iter()));
        let match_ids = files.iter().flat_map(|f| {
            let content = f.matches.iter().flat_map(|m| &m.fragments);
            let path = f.path_fragments.iter().flat_map(|p| &p.fragments);
            content
                .chain(path)
                .map(|f| ids(f.ids.iter().map(|(_, id)| id)))
        });
        let match_counts = files.iter().filter_map(|f| f.matches.as_ref()).any(|m| {
            line_counts(&m.content, &m.fragments)
                .into_iter()
                .any(|(_, count)| count != 1)
        });
        Self {
            conditions: differ(conditions),
            match_ids: differ(match_ids),
            match_counts,
        }
    }

    /// No ids differ unless `many`: the rule of the counted conditions; the counts of the matches stay.
    #[must_use]
    pub fn and(self, many: bool) -> Self {
        if many {
            self
        } else {
            Self {
                match_counts: self.match_counts,
                ..Self::default()
            }
        }
    }
}

/// What a response writes of the ids (OUTPUT.md#the-legend-of-the-written-ids).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Written {
    /// The conditions of the files are written.
    pub conditions: bool,
    /// The ids of the matches are written: the part `match_ids`, shown matches, a highlight which writes ids.
    pub match_ids: bool,
    /// The ids written in the `condition` facet.
    pub facet: Vec<String>,
}

/// The entries of `legend` whose ids the response of the result `files` writes, and every labelled condition, in
/// the order of the legend (OUTPUT.md#the-legend-of-the-written-ids).
#[must_use]
pub fn written_legend(
    legend: &[(CondId, String)],
    files: &[Found],
    written: &Written,
) -> Vec<(CondId, String)> {
    let mut ids: HashSet<String> = written.facet.iter().cloned().collect();
    for file in files {
        if written.conditions {
            ids.extend(file.conditions.iter().map(ToString::to_string));
        }
        if written.match_ids {
            let content = file.matches.iter().flat_map(|m| &m.fragments);
            let path = file.path_fragments.iter().flat_map(|p| &p.fragments);
            for fragment in content.chain(path) {
                ids.extend(fragment.ids.iter().map(|(_, id)| id.to_string()));
            }
        }
    }
    legend
        .iter()
        .filter(|(id, _)| {
            if matches!(id, CondId::Label(_)) {
                return true;
            }
            let id = id.to_string();
            // a sub-group (`#2.1`) is explained by its condition
            ids.contains(&id)
                || ids.iter().any(|written| {
                    written
                        .strip_prefix(id.as_str())
                        .is_some_and(|rest| rest.starts_with('.'))
                })
        })
        .cloned()
        .collect()
}

/// A set of ids, whatever their order.
fn ids<'a>(ids: impl Iterator<Item = &'a CondId>) -> HashSet<&'a CondId> {
    ids.collect()
}

/// Whether two of the sets differ.
fn differ<'a>(mut sets: impl Iterator<Item = HashSet<&'a CondId>>) -> bool {
    sets.next()
        .is_some_and(|first| sets.any(|set| set != first))
}

/// The parts a response leaves out; none by default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Omit(u32);

impl Omit {
    /// The parts of a checked value of `output.omit`: `"all"`, or a list of names.
    #[must_use]
    pub fn from_value(value: &KeyValue) -> Self {
        match value {
            KeyValue::Text(all) if all == "all" => Self::all(),
            KeyValue::List(names) => Self::from_parts(
                &names
                    .iter()
                    .filter_map(|n| Part::find(n))
                    .collect::<Vec<_>>(),
            ),
            _ => Self::default(),
        }
    }

    /// The set of these parts.
    #[must_use]
    pub fn from_parts(parts: &[Part]) -> Self {
        Self(parts.iter().fold(0, |set, p| set | 1 << *p as u32))
    }

    /// Whether the part is left out.
    #[must_use]
    pub fn has(self, part: Part) -> bool {
        self.0 & (1 << part as u32) != 0
    }

    /// Every part.
    #[must_use]
    pub fn all() -> Self {
        Self::from_parts(&Part::ALL)
    }

    /// The parts of both.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// The parts a response of the spec version `spec` leaves out (OUTPUT.md#defaults-of-spec-2): these, in spec 2
    /// also the parts of its rules — the conditions and the ids of the matches unless they `differ` (the ids with at
    /// most one of the `references` of `output.groups`, when it is set), the legend unless the conditions or, when
    /// the response has `matches_shown`, the ids of the matches are written, the gaps
    /// without `context` lines while the line numbers are written, the counts of the matches when every matched line
    /// has one match — except the parts of `show`; spec 1 has no line of the omitted warnings.
    #[must_use]
    pub fn effective(
        self,
        show: Self,
        spec: u32,
        differ: Differ,
        matches_shown: bool,
        context: bool,
        references: Option<usize>,
    ) -> Self {
        let mut omit = self;
        if spec >= 2 {
            if !differ.conditions {
                omit = omit.union(Self::from_parts(&[Part::Conditions]));
            }
            if references.map_or(!differ.match_ids, |n| n <= 1) {
                omit = omit.union(Self::from_parts(&[Part::MatchIds]));
            }
            // the legend explains the ids which the rules write, not the ones `show` brings back
            let ids = differ.match_ids && references.is_none_or(|n| n > 1) && matches_shown;
            if !differ.conditions && !ids {
                omit = omit.union(Self::from_parts(&[Part::Legend]));
            }
            if !differ.match_counts {
                omit = omit.union(Self::from_parts(&[Part::MatchCounts]));
            }
            if !context && !omit.has(Part::LineNumbers) {
                omit = omit.union(Self::from_parts(&[Part::Gaps]));
            }
        } else {
            omit = omit.union(Self::from_parts(&[Part::LeftOut]));
        }
        Self(omit.0 & !show.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeyValue;
    use crate::content::Content;
    use crate::output::lines::Fragment;
    use crate::query::ast::CondId;
    use crate::search::{FileMatches, Found};

    const BOTH: Differ = Differ {
        conditions: true,
        match_ids: true,
        match_counts: true,
    };

    /// A file of the content `content` with a fragment of the id 1 over each of `ranges` (start, end).
    fn spans(content: &str, ranges: &[(usize, usize)]) -> Found {
        let mut found = Found::path_only("f".into(), None);
        found.matches = Some(FileMatches {
            content: Content::from_bytes(content.as_bytes()),
            fragments: ranges
                .iter()
                .map(|&(start, end)| Fragment {
                    range: start..end,
                    ids: vec![(1, CondId::Number(1))],
                    escape: None,
                    groups: Vec::new(),
                })
                .collect(),
            lines: 1,
        });
        found
    }

    #[test]
    fn one_match_per_line() {
        // one match in every line of every file
        let one = [spans("a\na\n", &[(0, 1), (2, 3)]), spans("a\n", &[(0, 1)])];
        assert!(!Differ::of(&one).match_counts);
        // two matches in a line of one file
        let two = [
            spans("a\n", &[(0, 1)]),
            spans("a\naa\n", &[(0, 1), (2, 3), (3, 4)]),
        ];
        assert!(Differ::of(&two).match_counts);
        // 0 and 2 add up like 1 and 1: a multiline match continues into line 2
        assert!(Differ::of(&[spans("a a\nb\n", &[(0, 1), (2, 5)])]).match_counts);
        // no matched lines
        assert!(!Differ::of(&[Found::path_only("f".into(), None)]).match_counts);
        assert!(!Differ::of(&[]).match_counts);
        // the rule of the counted conditions doesn't change it
        assert!(BOTH.and(false).match_counts);
    }

    #[test]
    fn the_rule_of_the_match_counts() {
        let none = Omit::default();
        let parts = Omit::from_parts;
        let one = Differ::default();
        let other = Differ {
            match_counts: true,
            ..Differ::default()
        };
        assert!(
            none.effective(none, 2, one, true, false, None)
                .has(Part::MatchCounts)
        );
        assert!(
            !none
                .effective(none, 2, other, true, false, None)
                .has(Part::MatchCounts)
        );
        // spec 1 has no rules, show brings the part back, omit leaves it out whatever the matches
        assert!(
            !none
                .effective(none, 1, one, true, false, None)
                .has(Part::MatchCounts)
        );
        assert!(
            !none
                .effective(parts(&[Part::MatchCounts]), 2, one, true, false, None)
                .has(Part::MatchCounts)
        );
        assert!(
            parts(&[Part::MatchCounts])
                .effective(none, 2, other, true, false, None)
                .has(Part::MatchCounts)
        );
    }

    /// A file matched by the conditions `conditions` whose fragments have the ids `fragments`.
    fn file(conditions: &[u32], fragments: &[&[u32]]) -> Found {
        let id = |n: u32| (n as usize, CondId::Number(n));
        let mut found = Found::path_only("f".into(), None);
        found.conditions = conditions.iter().map(|n| id(*n).1).collect();
        found.matches = Some(FileMatches {
            content: Content::from_bytes(b"x x x x"),
            fragments: fragments
                .iter()
                .enumerate()
                .map(|(i, ids)| Fragment {
                    range: 2 * i..2 * i + 1,
                    ids: ids.iter().map(|n| id(*n)).collect(),
                    escape: None,
                    groups: Vec::new(),
                })
                .collect(),
            lines: 1,
        });
        found
    }

    fn legend(ids: &[CondId]) -> Vec<(CondId, String)> {
        ids.iter()
            .map(|id| (id.clone(), format!("c{id}")))
            .collect()
    }

    fn label(name: &str) -> CondId {
        CondId::Label(name.into())
    }

    #[test]
    fn the_legend_of_the_written_ids() {
        let n = CondId::Number;
        let all = legend(&[n(1), n(2), n(3), n(4)]);
        let files = [file(&[1, 2], &[&[2, 3]]), file(&[1, 2], &[&[2, 4]])];
        // nothing written: no entry
        assert_eq!(written_legend(&all, &files, &Written::default()), []);
        // the ids of the matches
        let ids = Written {
            match_ids: true,
            ..Written::default()
        };
        assert_eq!(
            written_legend(&all, &files, &ids),
            legend(&[n(2), n(3), n(4)])
        );
        // the conditions of the files
        let conditions = Written {
            conditions: true,
            ..Written::default()
        };
        assert_eq!(
            written_legend(&all, &files, &conditions),
            legend(&[n(1), n(2)])
        );
        // the condition facet
        let facet = Written {
            facet: vec!["#3".into(), "#1".into()],
            ..Written::default()
        };
        assert_eq!(written_legend(&all, &files, &facet), legend(&[n(1), n(3)]));
    }

    #[test]
    fn labels_stay_and_sub_groups_bring_their_condition() {
        let n = CondId::Number;
        let all = legend(&[label("rs"), n(1), label("def"), n(2)]);
        let mut found = file(&[1], &[]);
        found
            .matches
            .as_mut()
            .expect("a file with matches")
            .fragments = vec![Fragment {
            range: 0..1,
            ids: vec![(2, CondId::Sub("@def.fname".into()))],
            escape: None,
            groups: Vec::new(),
        }];
        assert_eq!(
            written_legend(&all, &[found.clone()], &Written::default()),
            legend(&[label("rs"), label("def")])
        );
        let all = legend(&[n(1), n(2)]);
        found
            .matches
            .as_mut()
            .expect("a file with matches")
            .fragments[0]
            .ids = vec![(2, CondId::Sub("#2.1".into()))];
        let ids = Written {
            match_ids: true,
            ..Written::default()
        };
        assert_eq!(written_legend(&all, &[found], &ids), legend(&[n(2)]));
    }

    #[test]
    fn what_differs_in_a_result() {
        let same = [file(&[1, 2], &[&[2]]), file(&[1, 2], &[&[2]])];
        assert_eq!(Differ::of(&same), Differ::default());
        // the order of the ids doesn't matter
        let reordered = [file(&[1, 2], &[&[2, 3]]), file(&[2, 1], &[&[3, 2]])];
        assert_eq!(Differ::of(&reordered), Differ::default());
        let branches = [file(&[1], &[]), file(&[2], &[&[2]])];
        assert_eq!(
            Differ::of(&branches),
            Differ {
                conditions: true,
                match_ids: false,
                match_counts: false
            }
        );
        // the matches of one file can differ
        let matches = [file(&[1], &[&[1, 2], &[1, 3]])];
        assert_eq!(
            Differ::of(&matches),
            Differ {
                conditions: false,
                match_ids: true,
                match_counts: true
            }
        );
        assert_eq!(Differ::of(&[]), Differ::default());
        assert_eq!(
            BOTH.and(false),
            Differ {
                match_counts: true,
                ..Differ::default()
            }
        );
        assert_eq!(BOTH.and(true), BOTH);
    }

    #[test]
    fn the_parts_which_tell_nothing_apart() {
        let none = Omit::default();
        let conditions = Differ {
            conditions: true,
            ..Differ::default()
        };
        let ids = Differ {
            match_ids: true,
            ..Differ::default()
        };
        // the files differ: the conditions and the legend
        let o = none.effective(none, 2, conditions, true, false, None);
        assert!(!o.has(Part::Conditions) && !o.has(Part::Legend) && o.has(Part::MatchIds));
        // the matches differ: their ids and, when matches are shown, the legend
        let o = none.effective(none, 2, ids, true, false, None);
        assert!(o.has(Part::Conditions) && !o.has(Part::Legend) && !o.has(Part::MatchIds));
        let o = none.effective(none, 2, ids, false, false, None);
        assert!(o.has(Part::Conditions) && o.has(Part::Legend) && !o.has(Part::MatchIds));
        // the conditions bring the legend without shown matches
        assert!(
            !none
                .effective(none, 2, conditions, false, false, None)
                .has(Part::Legend)
        );
        // show brings each part back
        let o = none.effective(
            Omit::from_parts(&[Part::Conditions]),
            2,
            Differ::default(),
            true,
            false,
            None,
        );
        assert!(!o.has(Part::Conditions) && o.has(Part::Legend));
    }

    #[test]
    fn parts_by_name() {
        assert_eq!(Part::ALL.len(), 19);
        assert_eq!(Part::find("match_counts"), Some(Part::MatchCounts));
        assert_eq!(Part::find("left_out"), Some(Part::LeftOut));
        assert_eq!(Part::find("more_lines"), Some(Part::MoreLines));
        assert_eq!(Part::find("dir"), Some(Part::Dir));
        assert_eq!(Part::find("match_ids"), Some(Part::MatchIds));
        assert_eq!(Part::find("colour"), None);
        assert_eq!(Part::LineNumbers.name(), "line_numbers");
        assert_eq!(
            Part::ALL.map(Part::name).join(", "),
            "counts, result, dir, legend, funnel, hints, conditions, file_counts, match_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out"
        );
        assert_eq!(Part::find("line_counts"), Some(Part::LineCounts));
    }

    #[test]
    fn the_value_of_the_key() {
        let none = Omit::from_value(&KeyValue::List(Vec::new()));
        assert!(Part::ALL.iter().all(|p| !none.has(*p)));
        let all = Omit::from_value(&KeyValue::Text("all".into()));
        assert!(Part::ALL.iter().all(|p| all.has(*p)));
        let some = Omit::from_value(&KeyValue::List(vec!["legend".into(), "gaps".into()]));
        assert!(some.has(Part::Legend) && some.has(Part::Gaps));
        assert!(!some.has(Part::Counts));
        assert_eq!(Omit::default(), none);
    }

    #[test]
    fn the_rules_of_spec_2_and_show() {
        let none = Omit::default();
        let parts = Omit::from_parts;
        // one counted condition, no context lines
        let one = none.effective(none, 2, Differ::default(), true, false, None);
        for part in [Part::Legend, Part::Conditions, Part::MatchIds, Part::Gaps] {
            assert!(one.has(part), "{part:?}");
        }
        assert!(!one.has(Part::Warnings) && !one.has(Part::LeftOut));
        // several counted conditions, context lines
        let several = none.effective(none, 2, BOTH, true, true, None);
        assert!(Part::ALL.iter().all(|p| !several.has(*p)));
        // without line numbers the gaps stay
        assert!(
            !parts(&[Part::LineNumbers])
                .effective(none, 2, BOTH, true, false, None)
                .has(Part::Gaps)
        );
        // show wins over omit and the rules
        let shown = parts(&[Part::Warnings]).effective(
            parts(&[Part::Warnings, Part::Legend]),
            2,
            Differ::default(),
            true,
            false,
            None,
        );
        assert!(
            !shown.has(Part::Warnings) && !shown.has(Part::Legend) && shown.has(Part::Conditions)
        );
        assert_eq!(
            none.effective(Omit::all(), 2, Differ::default(), true, false, None),
            none
        );
        // spec 1: no rules, no line of the omitted warnings
        assert_eq!(
            none.effective(none, 1, Differ::default(), true, false, None),
            parts(&[Part::LeftOut])
        );
        assert_eq!(
            parts(&[Part::Legend]).union(parts(&[Part::Gaps])),
            parts(&[Part::Legend, Part::Gaps])
        );
    }

    #[test]
    fn the_ids_of_the_matches_follow_the_references_of_groups() {
        let none = Omit::default();
        // one reference: the ids are left out, the legend of several conditions stays
        let one = none.effective(none, 2, BOTH, true, false, Some(1));
        assert!(one.has(Part::MatchIds) && !one.has(Part::Legend) && !one.has(Part::Conditions));
        // two references of one condition: the ids stay, the legend of one condition is left out
        let two = none.effective(none, 2, Differ::default(), true, false, Some(2));
        assert!(!two.has(Part::MatchIds) && two.has(Part::Legend));
        // spec 1 has no rules
        assert!(
            !none
                .effective(none, 1, BOTH, true, false, Some(1))
                .has(Part::MatchIds)
        );
    }
}
