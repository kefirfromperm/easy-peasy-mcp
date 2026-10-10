//! `/n`: an identifier in any naming style (QUERY.md#n--any-naming-style).

/// The separators between the words of a value and of an identifier.
const SEPARATORS: [&str; 4] = ["_", "-", ".", " "];

/// The words of `value`, lower case: split at spaces, `_`, `-`, `.` and case changes; an upper case run is one word,
/// digits stay with the preceding word.
pub fn words(value: &str) -> Vec<String> {
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
            word.extend(c.to_lowercase());
        }
        if !word.is_empty() {
            words.push(word);
        }
    }
    words
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
