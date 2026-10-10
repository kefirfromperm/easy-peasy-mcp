//! The relevance score of the matched files (OUTPUT.md#relevance).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{Kind, ParseOptions, parse};

    fn query(text: &str) -> crate::query::Query {
        let options = ParseOptions {
            case_insensitive_paths: false,
            now: jiff::Timestamp::UNIX_EPOCH,
            time_zone: jiff::tz::TimeZone::UTC,
            languages: std::sync::Arc::new(crate::languages::LanguageTable::builtin()),
            max_escape_level: 3,
            fuzzy_min_score: 0.5,
        };
        parse(text, &options).unwrap()
    }

    #[test]
    fn what_a_query_ranks_by() {
        let q = query("text has 'parse query' /n and text =~ 'xyz+' and name has 'abc'");
        assert_eq!(q.ranking.words, ["parse", "query"]);
        assert!(q.ranking.content);
        assert!(!q.ranking.fuzzy);
        assert_eq!(q.ranking.conditions, 3);
        assert_eq!(query("text has 'unwrap('").ranking.words, ["unwrap"]);
        assert!(query("text has 'a b'").ranking.words.is_empty());
        let q = query("name ~~ 'cfg' or not text has 'x'");
        assert!(q.ranking.fuzzy);
        assert_eq!(q.ranking.conditions, 1, "nothing under not");
    }

    #[test]
    fn the_weighted_mean_of_the_signals() {
        let ranking = query("text has 'TODO'").ranking;
        let signals = Signals {
            fuzzy: 0.0,
            words: 0.0,
            density: 1.0,
            conditions: 1.0,
            depth: 0,
        };
        let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
        assert!(close(score(&ranking, &signals, Some(Kind::Source), Some(0)), 1.0));
        // two weeks older than the newest file: recency 1 / 3
        let two_weeks = 14 * 86_400;
        assert!(close(
            score(&ranking, &signals, Some(Kind::Source), Some(two_weeks)),
            0.95
        ));
        // a test file one level down with a match on every tenth line
        let deeper = Signals {
            density: 0.1,
            depth: 1,
            ..signals
        };
        let density = (1.4_f64).ln() / 5_f64.ln();
        assert!(close(
            score(&ranking, &deeper, Some(Kind::Test), Some(0)),
            (0.20 * density + 0.10 + 0.04 * 0.7 + 0.03 + 0.03 * 0.5) / 0.40
        ));
    }
}
