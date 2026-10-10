//! `/e(style, levels)`: the escaped forms of a value (QUERY.md#escaping).

use std::fmt::Write as _;
use std::ops::RangeInclusive;

/// The highest escape level of `/e(style, any)` and of a query; `escape.max_level` comes with task 61.
pub const MAX_LEVEL: u8 = 3;

/// How a format escapes text (QUERY.md#styles).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Backslash,
    QuoteDoubling,
    Xml,
    Url,
}

impl Style {
    /// All styles in the order of the spec.
    pub const ALL: [Style; 4] = [Self::Backslash, Self::QuoteDoubling, Self::Xml, Self::Url];

    /// The style as written in a query.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Backslash => "backslash",
            Self::QuoteDoubling => "quote_doubling",
            Self::Xml => "xml",
            Self::Url => "url",
        }
    }

    /// The style written as `word`, in any case.
    pub fn from_keyword(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|s| s.keyword().eq_ignore_ascii_case(word))
    }
}

/// The forms of `c` escaped once.
fn escape_char(style: Style, c: char) -> Vec<String> {
    let one = |s: &str| vec![s.to_string()];
    match (style, c) {
        (Style::Backslash, '\\') => one("\\\\"),
        (Style::Backslash, '"') => one("\\\""),
        // JSON doesn't escape `'`
        (Style::Backslash, '\'') => vec!["'".to_string(), "\\'".to_string()],
        (Style::Backslash, '\n') => one("\\n"),
        (Style::Backslash, '\t') => one("\\t"),
        (Style::QuoteDoubling, '\'') => one("''"),
        (Style::QuoteDoubling, '"') => one("\"\""),
        (Style::Xml, '&') => one("&amp;"),
        (Style::Xml, '<') => one("&lt;"),
        (Style::Xml, '>') => one("&gt;"),
        (Style::Xml, '"') => one("&quot;"),
        (Style::Xml, '\'') => vec!["&apos;".to_string(), "&#39;".to_string()],
        (Style::Url, ' ') => vec!["%20".to_string(), "+".to_string()],
        (Style::Url, c) if !(c.is_ascii_alphanumeric() || "-._~".contains(c)) => {
            let mut bytes = [0; 4];
            let mut form = String::new();
            for b in c.encode_utf8(&mut bytes).bytes() {
                let _ = write!(form, "%{b:02X}");
            }
            vec![form]
        }
        _ => vec![c.to_string()],
    }
}

/// The forms of `s` escaped once: every combination of the forms of its characters.
fn escape_str(style: Style, s: &str) -> Vec<String> {
    s.chars().fold(vec![String::new()], |prefixes, c| {
        let forms = escape_char(style, c);
        prefixes
            .iter()
            .flat_map(|p| forms.iter().map(move |f| format!("{p}{f}")))
            .collect()
    })
}

/// The forms of `c` escaped `level` times.
pub fn forms(style: Style, c: char, level: u8) -> Vec<String> {
    let mut forms = std::collections::BTreeSet::from([c.to_string()]);
    for _ in 0..level {
        forms = forms.iter().flat_map(|f| escape_str(style, f)).collect();
    }
    forms.into_iter().collect()
}

/// The regex of `value` escaped on each of `levels`: one capture group per level in ascending order, without anchors.
pub fn pattern(style: Style, value: &str, levels: RangeInclusive<u8>) -> String {
    let level = |level: u8| -> String {
        let mut out = String::new();
        for c in value.chars() {
            let mut forms = forms(style, c, level);
            if let [form] = forms.as_slice() {
                out.push_str(&regex::escape(form));
            } else {
                forms.sort_by_key(|f| std::cmp::Reverse(f.len()));
                let alternatives: Vec<String> = forms.iter().map(|f| regex::escape(f)).collect();
                let _ = write!(out, "(?:{})", alternatives.join("|"));
            }
        }
        out
    };
    let groups: Vec<String> = levels.map(|l| format!("({})", level(l))).collect();
    format!("(?:{})", groups.join("|"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(style: Style, c: char, level: u8) -> Vec<String> {
        let mut f = forms(style, c, level);
        f.sort();
        f
    }

    #[test]
    fn the_levels_table() {
        let quote = |style| (0..=2).map(|l| sorted(style, '"', l)).collect::<Vec<_>>();
        assert_eq!(quote(Style::Backslash), [["\""], ["\\\""], ["\\\\\\\""]]);
        assert_eq!(
            quote(Style::QuoteDoubling),
            [["\""], ["\"\""], ["\"\"\"\""]]
        );
        assert_eq!(quote(Style::Xml), [["\""], ["&quot;"], ["&amp;quot;"]]);
        assert_eq!(quote(Style::Url), [["\""], ["%22"], ["%2522"]]);
    }

    #[test]
    fn styles() {
        assert_eq!(sorted(Style::Backslash, '\\', 1), ["\\\\"]);
        assert_eq!(sorted(Style::Backslash, '\n', 1), ["\\n"]);
        assert_eq!(sorted(Style::Backslash, '\t', 1), ["\\t"]);
        assert_eq!(sorted(Style::Backslash, 'a', 2), ["a"]);
        assert_eq!(sorted(Style::QuoteDoubling, '\'', 1), ["''"]);
        assert_eq!(sorted(Style::Xml, '&', 1), ["&amp;"]);
        assert_eq!(sorted(Style::Xml, '<', 1), ["&lt;"]);
        assert_eq!(sorted(Style::Xml, '>', 1), ["&gt;"]);
        assert_eq!(sorted(Style::Url, '%', 1), ["%25"]);
        assert_eq!(sorted(Style::Url, '~', 1), ["~"]);
        assert_eq!(sorted(Style::Url, 'é', 1), ["%C3%A9"]);
    }

    #[test]
    fn two_forms() {
        assert_eq!(sorted(Style::Backslash, '\'', 1), ["'", "\\'"]);
        assert_eq!(
            sorted(Style::Backslash, '\'', 2),
            ["'", "\\'", "\\\\'", "\\\\\\'"]
        );
        assert_eq!(sorted(Style::Xml, '\'', 1), ["&#39;", "&apos;"]);
        assert_eq!(sorted(Style::Xml, '\'', 2), ["&amp;#39;", "&amp;apos;"]);
        assert_eq!(sorted(Style::Url, ' ', 1), ["%20", "+"]);
        assert_eq!(sorted(Style::Url, ' ', 2), ["%2520", "%2B"]);
    }

    fn level_at(
        style: Style,
        value: &str,
        levels: RangeInclusive<u8>,
        hay: &str,
    ) -> Option<(String, u8)> {
        let re = regex::Regex::new(&pattern(style, value, levels.clone())).unwrap();
        let caps = re.captures(hay)?;
        let group = (1..caps.len()).find(|&i| caps.get(i).is_some())?;
        let level = levels.start() + u8::try_from(group - 1).unwrap();
        Some((caps[0].to_string(), level))
    }

    #[test]
    fn patterns_report_their_level() {
        assert_eq!(
            level_at(Style::Backslash, "a\"b", 0..=2, "x a\\\"b"),
            Some(("a\\\"b".into(), 1))
        );
        assert_eq!(
            level_at(Style::Backslash, "a\"b", 1..=2, "a\\\\\\\"b"),
            Some(("a\\\\\\\"b".into(), 2))
        );
        // a value without characters to escape is the lowest level
        assert_eq!(
            level_at(Style::Xml, "ab", 0..=3, "ab"),
            Some(("ab".into(), 0))
        );
        assert_eq!(
            level_at(Style::Xml, "ab", 2..=3, "ab"),
            Some(("ab".into(), 2))
        );
        assert_eq!(
            level_at(Style::Url, "a.b", 1..=1, "a.b"),
            Some(("a.b".into(), 1))
        );
        assert_eq!(level_at(Style::Url, "a+b", 1..=1, "a+b"), None);
        assert_eq!(
            level_at(Style::Url, "(x)", 1..=1, "%28x%29"),
            Some(("%28x%29".into(), 1))
        );
    }
}
