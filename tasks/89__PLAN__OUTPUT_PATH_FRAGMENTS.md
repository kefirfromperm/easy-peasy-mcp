# Plan for task 89: `output.path_fragments`

## Design

- `src/config/keys.rs`: `Key::PathFragments` after `After`, `Type::Bool`, default `false`.
- `src/query/eval.rs`: `Filter::path_ranges` — the byte ranges of a `name` / `path` condition in the path: every
  regex match (the whole value for `=` and `~`), the runs of matched characters of `~~`; a `name` range shifted to the
  name at the end of the path.
- `src/query/fuzzy.rs`: `Fuzzy::positions` — the matched characters of the best alignment of every term (the scoring
  DP keeps its rows and is walked back).
- `src/search/mod.rs`: `SearchOptions.path_fragments`; `Found.path_fragments: Option<PathFragments>` — the path as
  shown and the fragments of the contributing `name` / `path` conditions, identical ones merged.
- `src/engine.rs`: `search_all` puts the working directory before the path of the fragments too.
- `src/output/mod.rs`, `src/output/json.rs`: the text line `  path: …` (markers) / `  path @ …` (columns) after the
  file line, the JSON field `path_fragments` (OUTPUT.md#path-fragments, in its own commit).
- `src/description.rs`: the schema.

## Tests

- examples E89.1–E89.14 (fixture `path-frags`)
- unit: the ranges of the operators, the fuzzy positions, the request field

## Steps

1. failing tests
2. key, the ranges, the fuzzy positions, the output

## Risks

- the path as shown can differ from the NFC path a condition sees (a decomposed name on macOS); the fragments are
  computed on the NFC path, which is what the `path:` line shows
