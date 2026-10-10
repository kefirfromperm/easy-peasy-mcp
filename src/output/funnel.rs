//! The funnel of a response (OUTPUT.md#funnel).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_end_in_one_column() {
        let steps = [("lang = rust".to_string(), 312), ("and modified > -7d".to_string(), 41)];
        assert_eq!(
            lines(&steps),
            "funnel:\n  lang = rust                          312\n  and modified > -7d                    41\n"
        );
        let long = [("x".repeat(40), 5), ("y".to_string(), 10)];
        let text = lines(&long);
        assert!(text.contains(&format!("  {}  5\n", "x".repeat(40))), "{text}");
        assert!(text.contains(&format!("  y{}10\n", " ".repeat(40))), "{text}");
    }
}
