//! The funnel of a response (OUTPUT.md#funnel).

use std::fmt::Write as _;

/// The columns after the indent where the numbers of files end, unless a step is longer.
const WIDTH: usize = 40;

/// The text lines of the funnel: `funnel:` and a line per step, the numbers right-aligned in one column.
pub fn lines(steps: &[(String, usize)]) -> String {
    let width = steps
        .iter()
        .map(|(text, files)| text.chars().count() + 2 + files.to_string().len())
        .fold(WIDTH, usize::max);
    let mut out = String::from("funnel:\n");
    for (text, files) in steps {
        let files = files.to_string();
        let pad = width - text.chars().count() - files.len();
        let _ = writeln!(out, "  {text}{}{files}", " ".repeat(pad));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_end_in_one_column() {
        let steps = [
            ("lang = rust".to_string(), 312),
            ("and modified > -7d".to_string(), 41),
        ];
        assert_eq!(
            lines(&steps),
            "funnel:\n  lang = rust                          312\n  and modified > -7d                    41\n"
        );
        let long = [("x".repeat(40), 5), ("y".to_string(), 10)];
        let text = lines(&long);
        assert!(
            text.contains(&format!("  {}  5\n", "x".repeat(40))),
            "{text}"
        );
        assert!(
            text.contains(&format!("  y{}10\n", " ".repeat(40))),
            "{text}"
        );
    }
}
