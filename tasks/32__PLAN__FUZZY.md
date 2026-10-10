# Plan for task 32: Fuzzy `~~` on `name` and `path`

## Design

- `query/fuzzy.rs` (new): `Fuzzy { terms, dir_part }` and `score(haystack) -> Option<f64>`.
  - the value is split at spaces into terms; every term must appear in the haystack in order, case-insensitive
  - the score of a term is the best alignment (dynamic programming over the term characters and the haystack
    positions, linear in the haystack per character); each matched character weighs
    `1.0` at a word start (the first character, after `/`, `_`, `-`, `.`, a space, or a lower case letter or digit
    before an upper case one), else `0.8` right after the previous matched character, else `0.6` in the same word as
    the previous matched character, else `0.1` (a jump into the middle of a word); characters in the directory part of
    a path weigh 0.8 times as much; the term score is the mean weight
  - the score of the condition is the mean of the term scores times the length factor `0.8 + 0.2 · min(1, k / n)`,
    `k` the characters of all terms, `n` the characters of the haystack
  - no terms: every name matches with the score 1
- `query/eval.rs`: `TextMatcher` becomes an enum: `Regex` or `Fuzzy { fuzzy, min_score }`; `is_match` keeps a fuzzy
  match with a score of at least `min_score`.
- `query/mod.rs`: `compile_text` builds the fuzzy matcher for `~~` on the NFC value; `FUZZY_MIN_SCORE = 0.5` until
  `query.fuzzy_min_score` is configurable (task 62).
- `query/subset.rs`: `~~` is supported; `description.rs`: `~~`.

No new dependencies.

## Tests

- examples E32.1–E32.6 (fixture `fuzzy`)
- unit: word starts, case changes, consecutive characters, the directory penalty, the length factor, several terms,
  a missing term, the scores of the spec examples clearly above 0.5 and of a scattered match clearly below

## Steps

1. failing unit tests
2. fuzzy module, matcher, compile, subset
3. description

## Risks

- the score is a heuristic; the examples keep a margin from the threshold so tuning doesn't change them
