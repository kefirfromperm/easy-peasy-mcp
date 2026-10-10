//! The relevance score of the matched files (OUTPUT.md#relevance).

use crate::query::{Filter, Kind, TextMatcher, TextTarget, naming};
use crate::search::Found;

/// The weight of the best `~~` score.
const FUZZY: f64 = 0.35;
/// The weight of the query words in the path.
const WORDS: f64 = 0.25;
/// The weight of the matches per line.
const DENSITY: f64 = 0.20;
/// The weight of the share of the conditions which made the file match.
const CONDITIONS: f64 = 0.10;
/// The weight of the kind.
const KIND: f64 = 0.04;
/// The weight of the recency.
const RECENCY: f64 = 0.03;
/// The weight of the depth.
const DEPTH: f64 = 0.03;
/// The age in days which halves the recency.
const RECENCY_DAYS: f64 = 7.0;

/// What the score of a query looks at.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ranking {
    /// The query has `~~` conditions.
    pub fuzzy: bool,
    /// The words of the values of its `text` conditions, lower case, in their order.
    pub words: Vec<String>,
    /// The query reads the content: the matches per line count.
    pub content: bool,
    /// The number of its conditions outside of `not` (a `line( ... )` or `near( ... )` group is one).
    pub conditions: usize,
}

impl Ranking {
    /// Adds the words of a `text` value: split like `/n`, only letters and digits, at least two.
    pub fn add_words(&mut self, value: &str) {
        for word in naming::words(value) {
            let word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            if word.chars().count() >= 2 && !self.words.contains(&word) {
                self.words.push(word);
            }
        }
    }
}

/// The number of conditions of a filter outside of `not`.
pub fn conditions(filter: &Filter) -> usize {
    match filter {
        Filter::And(items) | Filter::Or(items) | Filter::Xor(items) => {
            items.iter().map(conditions).sum()
        }
        Filter::Not(_) => 0,
        _ => 1,
    }
}

/// The signals of a matched entry which don't need the other entries, each from 0 to 1 (the density and the depth
/// as they are).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Signals {
    /// The best score of the contributing `~~` conditions.
    pub fuzzy: f64,
    /// The query words in the name (1 each) or only in the directories (0.5 each), on average.
    pub words: f64,
    /// Matches per line of the content.
    pub density: f64,
    /// The share of the conditions which made the entry match.
    pub conditions: f64,
    /// The number of directories between the working directory and the entry.
    pub depth: usize,
}

/// The signals of an entry named `name` at `path` (NFC, `/` as the separator) which matched by the conditions
/// `leaves`, with the number of lines of its content and of its content fragments, if it was read.
#[allow(clippy::cast_precision_loss)] // counts far below 2^52
pub fn signals(
    ranking: &Ranking,
    leaves: &[&Filter],
    name: &str,
    path: &str,
    content: Option<(usize, usize)>,
) -> Signals {
    let fuzzy = leaves
        .iter()
        .filter_map(|leaf| match leaf {
            Filter::Text {
                target,
                matcher: TextMatcher::Fuzzy { fuzzy, .. },
                ..
            } => fuzzy.score(match target {
                TextTarget::Name => name,
                TextTarget::Path => path,
            }),
            _ => None,
        })
        .fold(0.0, f64::max);
    let name_words = naming::words(name);
    let dir_words: Vec<String> = path
        .rsplit_once('/')
        .map(|(dirs, _)| naming::words(&dirs.replace('/', " ")))
        .unwrap_or_default();
    let words = if ranking.words.is_empty() {
        0.0
    } else {
        let found: f64 = ranking
            .words
            .iter()
            .map(|w| {
                if name_words.contains(w) {
                    1.0
                } else if dir_words.contains(w) {
                    0.5
                } else {
                    0.0
                }
            })
            .sum();
        found / ranking.words.len() as f64
    };
    Signals {
        fuzzy,
        words,
        density: content.map_or(0.0, |(lines, matches)| matches as f64 / lines.max(1) as f64),
        conditions: if ranking.conditions == 0 {
            1.0
        } else {
            (leaves.len() as f64 / ranking.conditions as f64).min(1.0)
        },
        depth: path.matches('/').count(),
    }
}

/// The score from 0 to 1: the weighted mean of the signals which apply to the query; `age` is the number of
/// seconds the entry is older than the newest matched one.
#[allow(clippy::cast_precision_loss)] // depths and ages far below 2^52
pub fn score(ranking: &Ranking, signals: &Signals, kind: Option<Kind>, age: Option<i64>) -> f64 {
    let kind = match kind {
        Some(Kind::Source) => 1.0,
        Some(Kind::Test) => 0.7,
        Some(Kind::Config) => 0.5,
        Some(Kind::Doc) => 0.35,
        Some(Kind::Generated) => 0.1,
        Some(Kind::Other) | None => 0.0,
    };
    let recency = age.map_or(0.0, |seconds| {
        1.0 / (1.0 + seconds.max(0) as f64 / 86_400.0 / RECENCY_DAYS)
    });
    let density = (1.0 + 4.0 * signals.density).ln() / 5_f64.ln();
    let parts = [
        (ranking.fuzzy, FUZZY, signals.fuzzy),
        (!ranking.words.is_empty(), WORDS, signals.words),
        (ranking.content, DENSITY, density.min(1.0)),
        (true, CONDITIONS, signals.conditions),
        (true, KIND, kind),
        (true, RECENCY, recency),
        (true, DEPTH, 1.0 / (1.0 + signals.depth as f64)),
    ];
    let (sum, weights) = parts
        .iter()
        .filter(|(applies, _, _)| *applies)
        .fold((0.0, 0.0), |(sum, weights), (_, w, s)| {
            (sum + w * s, weights + w)
        });
    sum / weights
}

/// The score as it is shown: rounded to two decimals, half away from zero.
pub fn shown_score(score: f64) -> f64 {
    (score * 100.0).round() / 100.0
}

/// Scores the files, which are in path order, and sorts them by the score, higher first; equal scores stay in path
/// order. The recency is measured from the newest matched file, so the order doesn't depend on the clock.
pub fn rank(files: &mut [Found], ranking: &Ranking) {
    let newest = files.iter().filter_map(|f| f.meta.as_ref()?.modified).max();
    for file in files.iter_mut() {
        let Some(signals) = &file.signals else {
            continue;
        };
        let meta = file.meta.as_ref();
        let age = meta
            .and_then(|m| m.modified)
            .zip(newest)
            .map(|(m, n)| n - m);
        file.score = Some(score(ranking, signals, meta.and_then(|m| m.kind), age));
    }
    files.sort_by(|a, b| b.score.unwrap_or(0.0).total_cmp(&a.score.unwrap_or(0.0)));
}

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
        // a regex has no words: the density, the conditions, the kind, the recency and the depth apply
        let ranking = query("text =~ 'TODO'").ranking;
        let signals = Signals {
            fuzzy: 0.0,
            words: 0.0,
            density: 1.0,
            conditions: 1.0,
            depth: 0,
        };
        let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
        assert!(close(
            score(&ranking, &signals, Some(Kind::Source), Some(0)),
            1.0
        ));
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
