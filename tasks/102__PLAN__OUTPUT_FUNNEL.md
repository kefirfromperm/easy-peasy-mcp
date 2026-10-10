# Plan for task 102: `output.funnel`

## Design

- `src/config/keys.rs`: `Key::Funnel` after `Detail`, `Type::Choice(&["auto", "always", "never"])`, default `"auto"`.
- `src/query/mod.rs`: `Query.funnel` — the steps: the items of a top-level `and` (metadata steps first, then the
  content steps, each in the order of the query) or `or`, or the whole query; their text as in the legend (the tokens
  of the item with its brackets and `not`), the index of the item in the compiled filter.
- `src/search/mod.rs`: `SearchOptions.funnel`; the evaluation of an entry gives its funnel point — for `and` the
  number of leading steps it passes (unknown doesn't pass; a matched entry passes all, a failed one doesn't pass the
  last step it reached, so the last content step is never evaluated again), for `or` its first true branch; the walk
  adds them up; matched entries which are left out are not counted.
- `src/output/funnel.rs` (new): the text lines (aligned counts) and the JSON field; `Response.funnel`, shown with
  `auto` for an empty result or more than 10 × `max_files` files, with `always` in `count` detail and higher
  (OUTPUT.md#funnel, clarified in its own commit).
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E102.1–E102.12 (fixture `funnel`); earlier examples with an empty result get their funnel
- unit: the steps of a query (order, brackets, `not`, `or`, one condition), the funnel lines, the request field

## Steps

1. failing tests
2. key, the steps, the counts, the output

## Risks

- earlier examples with empty results change (their funnel), fixed in their own commits
