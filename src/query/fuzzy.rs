//! `~~`: fuzzy matching of `name` and `path` with a score (QUERY.md#--fuzzy).

/// The lowest score of a kept match; `query.fuzzy_min_score` comes with task 62.
pub const MIN_SCORE: f64 = 0.5;

/// A fuzzy value: its space separated terms, lower case.
#[derive(Debug, Clone, PartialEq)]
pub struct Fuzzy {
    terms: Vec<Vec<char>>,
    /// Whether the haystack is a path, whose directory part weighs less.
    path: bool,
}

impl Fuzzy {
    /// The fuzzy value of `value` for a `name` or a `path`.
    pub fn new(_value: &str, path: bool) -> Self {
        Self {
            terms: Vec::new(),
            path,
        }
    }

    /// The score from 0 to 1, or `None` when a term doesn't match.
    pub fn score(&self, _haystack: &str) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str, haystack: &str) -> Option<f64> {
        Fuzzy::new(value, false).score(haystack)
    }

    fn path(value: &str, haystack: &str) -> Option<f64> {
        Fuzzy::new(value, true).score(haystack)
    }

    fn close(score: Option<f64>, expected: f64) -> bool {
        score.is_some_and(|s| (s - expected).abs() < 1e-3)
    }

    #[test]
    fn word_starts_and_words() {
        // c l at word starts, f g d r in the same words: 4.4 / 6 · (0.8 + 0.2 · 6 / 16)
        assert!(close(name("cfgldr", "config_loader.rs"), 0.6417));
        assert!(close(name("cfgldr", "config-loader.ts"), 0.6417));
        // a case change starts a word; the value in any case
        assert!(close(name("CfgLdr", "ConfigLoader.java"), 0.6384));
    }

    #[test]
    fn jumps_into_words_score_low() {
        // c and g in the middle of words: 2.8 / 6 · (0.8 + 0.2 · 6 / 20)
        assert!(close(name("cfgldr", "scaffold_bigolder.ts"), 0.4013));
    }

    #[test]
    fn consecutive_characters() {
        assert!(close(
            name("load", "config_loader.rs"),
            (1.0 + 0.8 * 3.0) / 4.0 * (0.8 + 0.2 * 4.0 / 16.0)
        ));
        assert!(close(
            name("oad", "loader"),
            (0.1 + 0.8 * 2.0) / 3.0 * (0.8 + 0.2 * 3.0 / 6.0)
        ));
    }

    #[test]
    fn terms_are_averaged_and_all_must_match() {
        assert!(close(name("cfg java", "ConfigLoader.java"), 0.6985));
        assert_eq!(name("cfg rs", "ConfigLoader.java"), None);
        assert_eq!(name("dlc", "config_loader.rs"), None);
        assert_eq!(name("  ", "x"), Some(1.0));
    }

    #[test]
    fn directories_weigh_less() {
        assert!(close(path("srv hndl", "src/server/handlers.rs"), 0.6218));
        assert!(close(path("srv hndl", "services/http/handler.go"), 0.618));
        let in_name = path("ab", "x/ab").unwrap();
        let in_dir = path("ab", "ab/x").unwrap();
        assert!(in_name > in_dir);
    }

    #[test]
    fn shorter_haystacks_score_higher() {
        assert!(name("ab", "ab").unwrap() > name("ab", "ab_c").unwrap());
        assert!(close(name("ab", "ab"), 0.9));
    }
}
