# Summary of task 112: `since`

Server version: `0.112.0`.

## What was built

- `since: "r12"` (OUTPUT.md#changes-since): the request runs as usual and the response lists only the differences to
  the kept result: `+` added, `~` changed (another fingerprint of the matched line texts), `-` removed (or deleted);
  the unchanged files are counted. The header has `since: r12  unchanged: N`, JSON `since`, `unchanged` and every
  file's `change`.
- The files are written with full paths and their mark, without directory groups; the added and changed ones in the
  order of `output.sort` with their current lines, then the removed ones (path only) in the order of the kept result.
  The counts are of the current result, which is kept with an id of its own.
- `'since' must be a result id like 'r12'`, `result_expired` as for `within`.

## Architecture

- `src/engine.rs`: `differences` (the same file: the same path and member); `Run.since`.
- `src/output/mod.rs`: `Change`, `Since`, `Response.since` (`file_count`, `change`, the totals of the current
  result); `src/output/json.rs`: `since`, `unchanged`, `File.change`, a removed file with its path only.
- `src/search/mod.rs`: `Found::path_only`; `src/request.rs`: `Request.since`; `src/description.rs`.

## Deviations from the plan

- OUTPUT.md got how the differences are written (separate commit); E112.1 and E112.2 were fixed (the relevance order
  of their first request).

## Tests

- examples E112.1–E112.5 (fixture `funnel` of task 102)
- unit: `request::tests::since`; `tests/search.rs`: `since_sees_changed_and_deleted_files`

## Limitations and known gaps

- A file whose matched lines only moved (the same texts) is unchanged.

## Try it

`{ "query": "lang = rust and text has 'unwrap(' /c" }`, edit, then `{ "since": "r1", "query": "lang = rust and text
has 'unwrap(' /c" }`.
