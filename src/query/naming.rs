//! `/n`: an identifier in any naming style (QUERY.md#n--any-naming-style).

/// The words of `value`, lower case.
pub fn words(_value: &str) -> Vec<String> {
    Vec::new()
}

/// The regex of the words of `value` joined by nothing or by one separator, without anchors and flags.
pub fn pattern(_value: &str) -> String {
    String::new()
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
