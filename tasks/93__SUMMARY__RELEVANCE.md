# Summary of task 93: Relevance ranking

Server version: `0.93.0`.

## What was built

- `sort = relevance` (the default) orders the files by a score from 0 to 1, higher first; equal scores stay in path
  order. The score is deterministic: the recency is measured from the newest matched file, not from the clock.
- With `meta` every file shows its score, rounded to two decimals half away from zero: text `rust source 0.82`, JSON
  `"score": 0.82` after `encoding`. Another `sort` shows no score.

## The score

The weighted mean of the signals which apply to the query:

| Signal     | Weight | Applies                     | Value                                                          |
|------------|--------|-----------------------------|----------------------------------------------------------------|
| fuzzy      | 0.35   | the query has `~~`          | the best score of the contributing `~~` conditions             |
| words      | 0.25   | the `text` values have words | per query word 1 in the file name, 0.5 only in its directories, averaged |
| density    | 0.20   | the query reads the content | `min(1, ln(1 + 4d) / ln 5)`, `d` matches per line              |
| conditions | 0.10   | always                      | contributing conditions / conditions outside of `not`         |
| kind       | 0.04   | always                      | source 1, test 0.7, config 0.5, doc 0.35, generated 0.1, other 0 |
| recency    | 0.03   | always                      | `1 / (1 + days / 7)`, days older than the newest matched file  |
| depth      | 0.03   | always                      | `1 / (1 + depth)`, directories below the working directory     |

- query words: the values of `text` conditions with `=`, `has`, `starts`, `ends`, split like `/n` (spaces, `_`, `-`,
  `.`, case changes), only letters and digits, at least two characters; a file name's words are split the same way
- a directory and an unfollowed symlink have no kind (0)

## Architecture

- `src/relevance.rs` (new): `Ranking` (from the query), `Signals` (per matched entry), `score`, `rank`,
  `shown_score`.
- `src/query/mod.rs`: `Query.ranking`; `src/search/mod.rs`: `SearchOptions.rank`, `Found.signals`, `Found.score`,
  `Walk::left_out` (split from `visit`).
- `src/engine.rs`: with `relevance` the search computes the signals and the metadata (kind, date) and the files are
  ranked; `src/output`: the score with `meta`.

## Deviations from the plan

- Earlier examples in the new order or with the score: E9.4 (an unfollowed symlink last), E90.2 (source before docs),
  the examples of task 91 (scores), each in its own commit; E93.9 was fixed (the word `todo` of the query counts).

## Tests

- examples E93.1–E93.10 (fixture `relevance`: one signal per directory)
- unit: `relevance::tests::what_a_query_ranks_by`, `relevance::tests::the_weighted_mean_of_the_signals`

## Limitations and known gaps

- The relevance reads the language and kind of every matched file (first lines for shebangs and generated markers).

## Try it

`{ "query": "name ~~ 'cfgldr'", "output": { "detail": "files", "meta": true } }`
