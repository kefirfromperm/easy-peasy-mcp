# Summary of task 103: hints of an empty result

Server version: `0.103.0`.

## What was built

- With an empty result and a shown funnel the response has `hints:` after the funnel (JSON `hints`, a list of the
  hint lines, after `funnel`), found within 20% of the timeout (5 s without one):
  - the variations of the condition of the first step with 0 files (every branch of `or`) which is one `text`,
    `name` or `path` condition with a string: `/i`, `/n` (two words or more), without `/w`, `has` instead of `=`;
    each is searched with the steps before it and reported when it matches, with its number of files aligned with
    the funnel and up to three examples (the matched texts, the names or paths of the files)
  - `skipped:` the files which match when the ignore files, the skipping of generated files, the size limit or the
    skipping of binary files is relaxed
  - `name = '...'`: similar names (a third of the characters away at most); `path = / starts '...'` with a missing
    directory: `no such directory` and similar directories
- `funnel = never` turns the hints off too.

## Architecture

- `src/query/mod.rs`: `FunnelStep.condition`, `target`, `variations`, `lookup` (`Lookup`), made by `funnel_step`,
  `variations`, `rewritten` (tokens with one replaced or left out) and `lookup`; `edit_distance` is shared.
- `src/hints.rs` (new): `empty(Context)` → `Vec<Hint>`; every hint is a search (`engine::search_all`, or
  `search::search` per working directory for the names and directories) with the deadline of the hints; `OutOfTime`
  stops them.
- `src/output/funnel.rs`: `Hint` (a variation or a line), `lines` writes the hints; `Response.hints`, JSON `hints`.
- `src/engine.rs`: the hint context and budget; `search_sorted` split from `try_find`.

## Deviations from the plan

- OUTPUT.md got the details of the hints (separate commit); the examples of task 102 show the hints of their empty
  result, and E103.1, E103.3 and E103.9 got the warnings of the search (separate commits).
- The ignored files are counted with all ignore settings relaxed at once, so the hint names the setting
  (`traversal.ignore_files`), not the ignore file.

## Tests

- examples E103.1–E103.9 (fixture `hints`)
- unit: `query::tests::the_variations_of_a_step`, `output::funnel::tests::hints_after_the_funnel`

## Limitations and known gaps

- Hints of huge results come with task 104.

## Try it

`{ "query": "lang = rust and text has 'parseQuery'" }` in a project which writes `parse_query`.
