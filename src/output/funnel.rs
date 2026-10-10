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
            lines(&steps, &[]),
            "funnel:\n  lang = rust                          312\n  and modified > -7d                    41\n"
        );
        let long = [("x".repeat(40), 5), ("y".to_string(), 10)];
        let text = lines(&long, &[]);
        assert!(
            text.contains(&format!("  {}  5\n", "x".repeat(40))),
            "{text}"
        );
        assert!(
            text.contains(&format!("  y{}10\n", " ".repeat(40))),
            "{text}"
        );
    }

    #[test]
    fn hints_after_the_funnel() {
        let steps = [("text has 'parseQuery'".to_string(), 0)];
        let hints = [
            Hint::Variation {
                condition: "text has 'parseQuery' /i".to_string(),
                files: 1,
                examples: vec!["ParseQuery".to_string()],
            },
            Hint::Variation {
                condition: "text has 'parseQuery' /n".to_string(),
                files: 2,
                examples: vec!["ParseQuery".to_string(), "parse_query".to_string()],
            },
            Hint::Line("skipped: 1 generated (traversal.skip_generated)".to_string()),
        ];
        assert_eq!(
            lines(&steps, &hints),
            "funnel:\n  text has 'parseQuery'                  0\nhints:\n  text has 'parseQuery' /i               1 file: ParseQuery\n  text has 'parseQuery' /n               2 files: ParseQuery, parse_query\n  skipped: 1 generated (traversal.skip_generated)\n"
        );
        assert_eq!(hints[0].json(), "text has 'parseQuery' /i  1 file: ParseQuery");
        assert_eq!(hints[2].json(), "skipped: 1 generated (traversal.skip_generated)");
    }
}
