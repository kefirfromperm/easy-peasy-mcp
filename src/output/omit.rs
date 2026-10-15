//! The parts of a response which `output.omit` leaves out (OUTPUT.md#omit).

use crate::config::KeyValue;
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
    pub const ALL: [Part; 18] = [
        Self::Counts,
        Self::Result,
        Self::Dir,
        Self::Legend,
        Self::Funnel,
        Self::Hints,
        Self::Conditions,
        Self::FileCounts,
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
/// two files, different ids of two matches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Differ {
    pub conditions: bool,
    pub match_ids: bool,
}

impl Differ {
    /// What differs between all the files of a result.
    #[must_use]
    pub fn of(_files: &[Found]) -> Self {
        Self::default()
    }

    /// Nothing differs unless `many`: the rule of the counted conditions.
    #[must_use]
    pub fn and(self, many: bool) -> Self {
        if many { self } else { Self::default() }
    }
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
    /// also the parts of its rules — the legend, the conditions and the ids of the matches unless the query has
    /// `many_conditions` (the ids with at most one of the `references` of `output.groups`, when it is set), the gaps
    /// without `context` lines while the line numbers are written — except the parts of `show`; spec 1 has no line
    /// of the omitted warnings.
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
                omit = omit.union(Self::from_parts(&[Part::Legend, Part::Conditions]));
            }
            if references.map_or(!differ.match_ids, |n| n <= 1) {
                omit = omit.union(Self::from_parts(&[Part::MatchIds]));
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
    };

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

    #[test]
    fn what_differs_in_a_result() {
        let same = [file(&[1, 2], &[&[2], &[2]]), file(&[1, 2], &[&[2]])];
        assert_eq!(Differ::of(&same), Differ::default());
        // the order of the ids doesn't matter
        let reordered = [file(&[1, 2], &[&[2, 3]]), file(&[2, 1], &[&[3, 2]])];
        assert_eq!(Differ::of(&reordered), Differ::default());
        let branches = [file(&[1], &[]), file(&[2], &[&[2]])];
        assert_eq!(
            Differ::of(&branches),
            Differ {
                conditions: true,
                match_ids: false
            }
        );
        // the matches of one file can differ
        let matches = [file(&[1], &[&[1, 2], &[1, 3]])];
        assert_eq!(
            Differ::of(&matches),
            Differ {
                conditions: false,
                match_ids: true
            }
        );
        assert_eq!(Differ::of(&[]), Differ::default());
        assert_eq!(BOTH.and(false), Differ::default());
        assert_eq!(BOTH.and(true), BOTH);
    }

    #[test]
    fn the_parts_which_tell_nothing_apart() {
        let none = Omit::default();
        let conditions = Differ {
            conditions: true,
            match_ids: false,
        };
        let ids = Differ {
            conditions: false,
            match_ids: true,
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
        assert_eq!(Part::ALL.len(), 18);
        assert_eq!(Part::find("left_out"), Some(Part::LeftOut));
        assert_eq!(Part::find("more_lines"), Some(Part::MoreLines));
        assert_eq!(Part::find("dir"), Some(Part::Dir));
        assert_eq!(Part::find("match_ids"), Some(Part::MatchIds));
        assert_eq!(Part::find("colour"), None);
        assert_eq!(Part::LineNumbers.name(), "line_numbers");
        assert_eq!(
            Part::ALL.map(Part::name).join(", "),
            "counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out"
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
