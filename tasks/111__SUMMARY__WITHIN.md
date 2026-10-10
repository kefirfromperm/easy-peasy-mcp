# Summary of task 111: `within`

Server version: `0.111.0`.

## What was built

- `within: "r12"` (OUTPUT.md#refine-within): only the files of a kept result are searched, without a walk. `dirs` and
  the walk options are ignored, the file and matching options apply. The files are read again; deleted ones are
  skipped, ones outside of the allowed roots now get `permission_denied`. The paths are written as in the result and
  `path` conditions see them relative to its working directories; members of archives are read from their archive.
- A value which isn't a result id is `invalid_option` (`'within' must be a result id like 'r12'`); an unknown or
  evicted id is `result_expired` (`the result r12 is unknown or no longer kept`, hint `the server keeps the last
  server.result_cache results`).
- The response is a result of its own (its own id, the funnel, the sort, the limits, the budget as usual).

## Architecture

- `src/search/mod.rs`: `Entry` (implemented by the walk's `ignore::DirEntry` and by `FileEntry`) for `visit`,
  `visit_members` (with `only` members), `meta` and `left_out`; `search_files`; `Walk::new` / `finish` shared with
  `search`; `Found.root`.
- `src/cache.rs`: a kept result keeps its working directories, every file its root index, its path inside it and its
  member.
- `src/engine.rs`: `kept`, `search_within`; `try_find` split into `run` (the search, `Run`) and `respond`.
- `src/request.rs`: `Request.within`; `src/error.rs`: `ErrorCode::ResultExpired`; `src/description.rs`.

## Deviations from the plan

- OUTPUT.md got the value, the error and the allowed roots of `within` (separate commit).

## Tests

- examples E111.1–E111.5 (fixture `funnel` of task 102)
- unit: `request::tests::within`, `cache::tests::ids_and_eviction`; `tests/search.rs`:
  `within_skips_files_deleted_since_the_result`

## Limitations and known gaps

None.

## Try it

`{ "query": "lang = rust and text has 'retry'" }`, then `{ "within": "r1", "query": "text has 'backoff'" }`.
