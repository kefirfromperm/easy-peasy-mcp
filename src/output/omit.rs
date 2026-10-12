//! The parts of a response which `output.omit` leaves out (OUTPUT.md#omit).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KeyValue;

    #[test]
    fn parts_by_name() {
        assert_eq!(Part::ALL.len(), 12);
        assert_eq!(Part::find("match_ids"), Some(Part::MatchIds));
        assert_eq!(Part::find("colour"), None);
        assert_eq!(Part::LineNumbers.name(), "line_numbers");
        assert_eq!(
            Part::ALL.map(Part::name).join(", "),
            "counts, result, legend, funnel, hints, conditions, file_counts, match_ids, line_numbers, gaps, indent, warnings"
        );
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
