//! `/n`: an identifier in any naming style (QUERY.md#n--any-naming-style).

/// The separators between the words of a value and of an identifier.
const SEPARATORS: [&str; 4] = ["_", "-", ".", " "];

/// The words of `value`, lower case: split at spaces, `_`, `-`, `.` and case changes; an upper case run is one word,
/// digits stay with the preceding word.
pub fn words(value: &str) -> Vec<String> {
    split(value).iter().map(|w| w.to_lowercase()).collect()
}

/// The words of `value` like `words`, in their case.
fn split(value: &str) -> Vec<String> {
    let mut words = Vec::new();
    for token in value.split([' ', '_', '-', '.']) {
        let chars: Vec<char> = token.chars().collect();
        let mut word = String::new();
        for (i, &c) in chars.iter().enumerate() {
            let previous = i.checked_sub(1).map(|p| chars[p]);
            let next = chars.get(i + 1);
            let starts_word = c.is_uppercase()
                && previous.is_some_and(|p| {
                    p.is_lowercase()
                        || p.is_numeric()
                        || (p.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
                });
            if starts_word && !word.is_empty() {
                words.push(std::mem::take(&mut word));
            }
            word.push(c);
        }
        if !word.is_empty() {
            words.push(word);
        }
    }
    words
}

/// The case of a word of an identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Case {
    Lower,
    Upper,
    Capitalized,
}

impl Case {
    /// The case of `word`: upper with an upper case letter and no lower case one, capitalized with an upper case
    /// first letter, lower otherwise.
    fn of(word: &str) -> Self {
        if word.chars().any(char::is_uppercase) && !word.chars().any(char::is_lowercase) {
            Self::Upper
        } else if word.chars().next().is_some_and(char::is_uppercase) {
            Self::Capitalized
        } else {
            Self::Lower
        }
    }

    /// `word` (lower case) in this case.
    fn write(self, word: &str) -> String {
        match self {
            Self::Lower => word.to_string(),
            Self::Upper => word.to_uppercase(),
            Self::Capitalized => {
                let mut chars = word.chars();
                chars.next().map_or_else(String::new, |first| {
                    first.to_uppercase().chain(chars).collect()
                })
            }
        }
    }
}

/// `new` written in the naming style of `matched` (TOOLS.md#templates): its words joined by the separator of
/// `matched`, each in the case of the word of `matched` at its position (after the last one in the case of the last);
/// a match of one word gives `camelCase`, `PascalCase` or `UPPER_SNAKE`. `new` without words stays as it is.
pub fn restyle(new: &str, matched: &str) -> String {
    let words = words(new);
    let style = split(matched);
    if words.is_empty() || style.is_empty() {
        return new.to_string();
    }
    let separator = matched
        .chars()
        .find(|c| [' ', '_', '-', '.'].contains(c))
        .map(String::from)
        .unwrap_or_default();
    let cases: Vec<Case> = style.iter().map(|w| Case::of(w)).collect();
    let (cases, separator) = match (cases.as_slice(), separator.is_empty()) {
        ([Case::Lower], true) => (vec![Case::Lower, Case::Capitalized], separator),
        ([Case::Upper], true) => (cases, "_".to_string()),
        _ => (cases, separator),
    };
    let last = *cases.last().expect("a match has a word");
    words
        .iter()
        .enumerate()
        .map(|(i, word)| cases.get(i).copied().unwrap_or(last).write(word))
        .collect::<Vec<String>>()
        .join(&separator)
}

/// The regex of the words of `value` joined by nothing or by one separator, the same between all words, without
/// anchors and flags.
pub fn pattern(value: &str) -> String {
    let words: Vec<String> = words(value).iter().map(|w| regex::escape(w)).collect();
    if words.len() < 2 {
        return words.concat();
    }
    let joined: Vec<String> = std::iter::once(words.concat())
        .chain(
            SEPARATORS
                .iter()
                .map(|separator| words.join(&regex::escape(separator))),
        )
        .collect();
    format!("(?:{})", joined.join("|"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_of_every_style() {
        for value in [
            "parse query",
            "parseQuery",
            "ParseQuery",
            "parse_query",
            "PARSE_QUERY",
            "parse-query",
            "parse.query",
            " parse  query ",
        ] {
            assert_eq!(words(value), ["parse", "query"], "{value}");
        }
        assert_eq!(words("parsequery"), ["parsequery"]);
    }

    #[test]
    fn upper_case_runs_and_digits() {
        assert_eq!(words("HTTPServer"), ["http", "server"]);
        assert_eq!(words("getHTTP"), ["get", "http"]);
        assert_eq!(words("utf8Decode"), ["utf8", "decode"]);
        assert_eq!(words("utf8decode"), ["utf8decode"]);
        assert_eq!(words("v2API"), ["v2", "api"]);
        assert_eq!(words("ÉtéChaud"), ["été", "chaud"]);
        assert!(words("_-.").is_empty());
    }

    #[test]
    fn the_style_of_the_match() {
        for (matched, new) in [
            ("parseQuery", "buildRequest"),
            ("ParseQuery", "BuildRequest"),
            ("parse_query", "build_request"),
            ("PARSE_QUERY", "BUILD_REQUEST"),
            ("parse-query", "build-request"),
            ("parse.query", "build.request"),
            ("parse query", "build request"),
            ("Parse Query", "Build Request"),
        ] {
            assert_eq!(restyle("build request", matched), new, "{matched}");
        }
        assert_eq!(
            restyle("build new request", "parseQuery"),
            "buildNewRequest"
        );
        assert_eq!(restyle("buildRequest", "PARSE_QUERY"), "BUILD_REQUEST");
        assert_eq!(restyle("build request", "parse"), "buildRequest");
        assert_eq!(restyle("build request", "Parse"), "BuildRequest");
        assert_eq!(restyle("build request", "PARSE"), "BUILD_REQUEST");
        assert_eq!(restyle("x", "PARSE_QUERY"), "X");
        assert_eq!(restyle("", "parseQuery"), "");
        assert_eq!(restyle("été chaud", "ParseQuery"), "ÉtéChaud");
    }

    #[test]
    fn patterns() {
        let re = regex::Regex::new(&format!("(?i)^{}$", pattern("parse query"))).unwrap();
        for hay in [
            "parseQuery",
            "PARSE_QUERY",
            "parse-query",
            "parse.query",
            "parse query",
            "PaRsEqUeRy",
        ] {
            assert!(re.is_match(hay), "{hay}");
        }
        for hay in ["parse__query", "parse_query-", "parse-query_x", "parse"] {
            assert!(!re.is_match(hay), "{hay}");
        }
        assert_eq!(pattern("a.b"), r"(?:ab|a_b|a\-b|a\.b|a b)");
        assert_eq!(pattern("one"), "one");
    }
}
