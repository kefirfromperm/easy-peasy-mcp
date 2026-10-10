//! The funnel of a response (OUTPUT.md#funnel).

use std::fmt::Write as _;

/// The columns after the indent where the numbers of files end, unless a step is longer.
const WIDTH: usize = 40;

/// A hint of a response (OUTPUT.md#hints).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hint {
    /// A variation of a condition which matches: its number of files and examples of what it matched.
    Variation {
        condition: String,
        files: usize,
        examples: Vec<String>,
    },
    /// Any other hint, as it is written.
    Line(String),
}

impl Hint {
    /// The hint in JSON: as in the text format, a variation with two spaces before its number of files.
    #[must_use]
    pub fn json(&self) -> String {
        match self {
            Self::Variation { condition, .. } => format!("{condition}  {}", self.files_text()),
            Self::Line(text) => text.clone(),
        }
    }

    /// `N file(s): a, b` of a variation.
    fn files_text(&self) -> String {
        let Self::Variation {
            files, examples, ..
        } = self
        else {
            return String::new();
        };
        let word = if *files == 1 { "file" } else { "files" };
        if examples.is_empty() {
            format!("{files} {word}")
        } else {
            format!("{files} {word}: {}", examples.join(", "))
        }
    }
}

/// The text lines of the funnel and the hints: `funnel:` and a line per step, `hints:` and a line per hint; the
/// numbers of the steps and of the variations right-aligned in one column.
pub fn lines(steps: &[(String, usize)], hints: &[Hint]) -> String {
    let variations = hints.iter().filter_map(|hint| match hint {
        Hint::Variation {
            condition, files, ..
        } => Some((condition, *files)),
        Hint::Line(_) => None,
    });
    let width = steps
        .iter()
        .map(|(text, files)| (text, *files))
        .chain(variations)
        .map(|(text, files)| text.chars().count() + 2 + files.to_string().len())
        .fold(WIDTH, usize::max);
    let aligned = |text: &str, files: usize| {
        let files = files.to_string();
        format!(
            "{text}{}{files}",
            " ".repeat(width - text.chars().count() - files.len())
        )
    };
    let mut out = String::from("funnel:\n");
    for (text, files) in steps {
        let _ = writeln!(out, "  {}", aligned(text, *files));
    }
    if !hints.is_empty() {
        out.push_str("hints:\n");
    }
    for hint in hints {
        match hint {
            Hint::Variation {
                condition, files, ..
            } => {
                let files_text = hint.files_text();
                let count = files.to_string();
                let _ = writeln!(
                    out,
                    "  {}{}",
                    aligned(condition, *files),
                    &files_text[count.len()..]
                );
            }
            Hint::Line(text) => {
                let _ = writeln!(out, "  {text}");
            }
        }
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
        assert_eq!(
            hints[0].json(),
            "text has 'parseQuery' /i  1 file: ParseQuery"
        );
        assert_eq!(
            hints[2].json(),
            "skipped: 1 generated (traversal.skip_generated)"
        );
    }
}
