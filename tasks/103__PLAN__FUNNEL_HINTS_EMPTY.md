# Plan for task 103: hints of an empty result

## Design

- `src/query/mod.rs`: `FunnelStep.condition` (the step without `and ` / `or `) and `FunnelStep.variations` — for an
  item which is one `text` / `name` / `path` condition with a string, the texts of `/i`, `/n`, without `/w`, `has`
  instead of `=` (OUTPUT.md#hints, clarified in its own commit), made from its tokens; `FunnelStep.target` and `op`
  for the similar names and paths.
- `src/hints.rs` (new): with an empty result and a shown funnel, within 20% of the timeout (5 s without one):
  - every variation of the first step with 0 files (every branch of `or`) is searched together with the steps before
    it; one which matches gives its files and up to three examples (the distinct texts of its fragments, the names
    or paths of the files)
  - the query is searched again with one setting relaxed at a time (the ignore files and git excludes, generated
    files, the size limit, binary files): the files found are the skipped ones
  - `name = '...'`: the names of all files within a third of their characters (edit distance); `path = / starts`
    with a missing directory: the similar directories
- `src/output/mod.rs`, `src/output/funnel.rs`, `src/output/json.rs`: `Response.hints`, `hints:` after the funnel
  with the numbers of the variations in the column of the funnel, JSON `hints` after `funnel`.
- `src/engine.rs`: the context the hints need (working directories, parse and search options, deadline).
- `src/query/suggest.rs`: `edit_distance` shared.

## Tests

- examples E103.1–E103.9 (fixture `hints`)
- unit: the variations of a step, the hint lines

## Steps

1. failing tests
2. the variations, the hint searches, the output

## Risks

- every hint is a search: they are cut by their time budget
