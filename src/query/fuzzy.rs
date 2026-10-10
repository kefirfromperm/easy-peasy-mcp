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

/// The weight of a matched character at a word start.
const AT_WORD_START: f64 = 1.0;
/// The weight of a matched character right after the previous one.
const CONSECUTIVE: f64 = 0.8;
/// The weight of a matched character in the word of the previous one.
const SAME_WORD: f64 = 0.6;
/// The weight of a matched character in the middle of another word.
const MID_WORD: f64 = 0.1;
/// The factor of the weights of characters in the directory part of a path.
const IN_DIRECTORY: f64 = 0.8;

impl Fuzzy {
    /// The fuzzy value of `value` for a `name` or a `path`.
    pub fn new(value: &str, path: bool) -> Self {
        let terms = value
            .split(' ')
            .filter(|t| !t.is_empty())
            .map(|t| t.chars().map(lower).collect())
            .collect();
        Self { terms, path }
    }

    /// The score from 0 to 1, or `None` when a term doesn't match: the mean of the term scores times
    /// `0.8 + 0.2 · min(1, k / n)`, `k` the characters of the terms, `n` the characters of the haystack.
    pub fn score(&self, haystack: &str) -> Option<f64> {
        if self.terms.is_empty() {
            return Some(1.0);
        }
        let hay = Haystack::new(haystack, self.path);
        let mut sum = 0.0;
        for term in &self.terms {
            sum += hay.term_score(term)?;
        }
        let k: usize = self.terms.iter().map(Vec::len).sum();
        #[allow(clippy::cast_precision_loss)]
        // values and paths are far shorter than 2^52 characters
        let (terms, k, n) = (self.terms.len() as f64, k as f64, hay.chars.len() as f64);
        Some(sum / terms * (0.8 + 0.2 * (k / n).min(1.0)))
    }
}

/// A character in lower case, for comparing without case.
fn lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// The haystack with what the scoring needs per character.
struct Haystack {
    /// The characters, lower case.
    chars: Vec<char>,
    /// Whether a word starts at the character: the first one, after `/`, `_`, `-`, `.`, a space, or an upper case
    /// letter after a lower case letter or a digit.
    starts: Vec<bool>,
    /// The index of the start of the word of the character.
    word: Vec<usize>,
    /// The factor of the weight of the character: less in the directory part of a path.
    factor: Vec<f64>,
}

impl Haystack {
    fn new(text: &str, path: bool) -> Self {
        let original: Vec<char> = text.chars().collect();
        let dir_end = if path {
            original
                .iter()
                .rposition(|&c| c == '/')
                .map_or(0, |i| i + 1)
        } else {
            0
        };
        let starts: Vec<bool> = (0..original.len())
            .map(|i| {
                i == 0 || {
                    let (prev, c) = (original[i - 1], original[i]);
                    matches!(prev, '/' | '_' | '-' | '.' | ' ')
                        || (c.is_uppercase() && (prev.is_lowercase() || prev.is_numeric()))
                }
            })
            .collect();
        let mut word = Vec::with_capacity(original.len());
        for (i, &start) in starts.iter().enumerate() {
            word.push(if start {
                i
            } else {
                word.last().copied().unwrap_or(0)
            });
        }
        Self {
            chars: original.iter().map(|&c| lower(c)).collect(),
            starts,
            word,
            factor: (0..original.len())
                .map(|i| if i < dir_end { IN_DIRECTORY } else { 1.0 })
                .collect(),
        }
    }

    /// The mean weight of the characters of `term` in their best alignment, or `None` if they don't appear in
    /// order.
    fn term_score(&self, term: &[char]) -> Option<f64> {
        let n = self.chars.len();
        // best[p]: the best sum of the weights of the term characters so far with the last one at p
        let mut best: Vec<Option<f64>> = vec![None; n];
        for (i, &t) in term.iter().enumerate() {
            let previous = best;
            // the best sum ending at or before p
            let mut up_to: Vec<Option<f64>> = Vec::with_capacity(n);
            // the best sum ending in the word of p, at or before p
            let mut in_word: Vec<Option<f64>> = Vec::with_capacity(n);
            for (&sum, &start) in previous.iter().zip(&self.starts) {
                up_to.push(max(up_to.last().copied().flatten(), sum));
                let same = if start {
                    None
                } else {
                    in_word.last().copied().flatten()
                };
                in_word.push(max(same, sum));
            }
            best = (0..n)
                .map(|p| {
                    if self.chars[p] != t {
                        return None;
                    }
                    let weight = |w: f64| w * self.factor[p];
                    if i == 0 {
                        return Some(weight(if self.starts[p] {
                            AT_WORD_START
                        } else {
                            MID_WORD
                        }));
                    }
                    let before = p.checked_sub(1).and_then(|q| up_to[q])?;
                    if self.starts[p] {
                        return Some(before + weight(AT_WORD_START));
                    }
                    let consecutive = previous[p - 1].map(|s| s + weight(CONSECUTIVE));
                    let same_word = (p >= self.word[p] + 2)
                        .then(|| in_word[p - 2])
                        .flatten()
                        .map(|s| s + weight(SAME_WORD));
                    let other_word = self.word[p]
                        .checked_sub(1)
                        .and_then(|q| up_to[q])
                        .map(|s| s + weight(MID_WORD));
                    max(max(consecutive, same_word), other_word)
                })
                .collect();
        }
        let sum = best.into_iter().flatten().reduce(f64::max)?;
        #[allow(clippy::cast_precision_loss)] // terms are far shorter than 2^52 characters
        Some(sum / term.len() as f64)
    }
}

/// The larger of two optional sums.
fn max(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (a, b) => a.or(b),
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
        // c l r at word starts, f g d in the same words: 4.8 / 6 · (0.8 + 0.2 · 6 / 16)
        assert!(close(name("cfgldr", "config_loader.rs"), 0.7));
        assert!(close(name("cfgldr", "config-loader.rs"), 0.7));
        // r in the same word: 4.4 / 6 · (0.8 + 0.2 · 6 / 16)
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
