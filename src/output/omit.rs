//! The parts of a response which `output.omit` leaves out (OUTPUT.md#omit).

use crate::config::KeyValue;

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
    Locations,
    Warnings,
}

impl Part {
    /// All parts in the order of the spec.
    pub const ALL: [Part; 16] = [
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
        Self::Locations,
        Self::Warnings,
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
            Self::Locations => "locations",
            Self::Warnings => "warnings",
        }
    }

    /// The part of a name.
    #[must_use]
    pub fn find(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.name() == name)
    }
}

/// The parts a response leaves out; none by default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Omit(u16);

impl Omit {
    /// The parts of a checked value of `output.omit`: `"all"`, or a list of names.
    #[must_use]
    pub fn from_value(value: &KeyValue) -> Self {
        match value {
            KeyValue::Text(all) if all == "all" => Self::from_parts(&Part::ALL),
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
        Self(parts.iter().fold(0, |set, p| set | 1 << *p as u16))
    }

    /// Whether the part is left out.
    #[must_use]
    pub fn has(self, part: Part) -> bool {
        self.0 & (1 << part as u16) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeyValue;

    #[test]
    fn parts_by_name() {
        assert_eq!(Part::ALL.len(), 16);
        assert_eq!(Part::find("dir"), Some(Part::Dir));
        assert_eq!(Part::find("match_ids"), Some(Part::MatchIds));
        assert_eq!(Part::find("colour"), None);
        assert_eq!(Part::LineNumbers.name(), "line_numbers");
        assert_eq!(
            Part::ALL.map(Part::name).join(", "),
            "counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings"
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
}
