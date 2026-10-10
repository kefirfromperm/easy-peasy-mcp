# Plan for task 93: Relevance ranking

## Design

- `src/query/mod.rs`: `Query.ranking` — what the score of the query looks at: whether it has `~~` conditions, the
  words of its `text` values (`=`, `has`, `starts`, `ends`; split like `/n`, letters and digits, at least two),
  whether it reads the content, the number of its conditions outside of `not`.
- `src/relevance.rs` (new): `Signals` of a matched entry, computed when it is evaluated — the best score of its
  contributing `~~` conditions, the share of the query words in its name (1) or only in its directories (0.5), its
  matches per line, the share of the conditions which made it match, its depth; `rank(files, query)` adds the kind
  and the recency from the newest matched file, computes the score and sorts by it, equal scores in path order.
- the score: the weighted mean of the signals which apply to the query (fuzzy 0.35, words 0.25, density 0.20,
  conditions 0.10, kind 0.04, recency 0.03, depth 0.03); density `min(1, ln(1 + 4d) / ln 5)`, kind source 1, test
  0.7, config 0.5, doc 0.35, generated 0.1, other 0, recency `1 / (1 + days / 7)`, depth `1 / (1 + depth)`.
- `src/search/mod.rs`: `Found.signals`, `Found.score`; `src/output/sort.rs`: `relevance` needs the metadata (kind,
  date) and ranks; `src/output`: the score after the kind with `meta` (two decimals).

## Tests

- examples E93.1–E93.10 (fixture `relevance`), the examples of the earlier tasks in the new default order
- unit: the signals, the score, the query words

## Steps

1. failing tests
2. the signals, the ranking, the score in the output
3. the examples of earlier tasks whose order changes, each in its own commit before the implementation

## Risks

- earlier examples whose files differ in kind, depth, date or density change their order
