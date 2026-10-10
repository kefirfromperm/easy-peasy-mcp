# Summary of task 79: `output.detail`

Server version: `0.79.0`.

## What was built

- `output.detail` (CONFIG.md#output, OUTPUT.md#detail-levels), default `"lines"`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by `output.detail`.
- `exists`: `exists: yes` / `exists: no` (after a timeout with `truncated: yes (timeout)`) and the warnings; JSON
  `version`, `exists`; no result id. The walk quits at the first matched entry, the next working directories are not
  searched.
- `count`: the totals with the result id and the warnings; JSON without `legend` and `files`.
- `files`: the totals, the legend and the files with their conditions and counts, without their lines; JSON files
  without `blocks`.
- `lines`: as before.
- `facets` is a valid value of the spec and fails with `not_implemented` until task 105.

## Architecture

- `src/output/mod.rs`: `Detail`, `Response.detail` for the text; `src/output/json.rs` for JSON.
- `src/search/mod.rs`: `SearchOptions.first_only`, `Walk.done` stops the threads of the walk.
- `src/engine.rs`: `search_all` (the working directories), `keep_result` (no result id for `exists`).
- `src/config/keys.rs`: `Key::Detail`, `facets` in `UNIMPLEMENTED`; `src/description.rs`: `detail` in `output`.

## Deviations from the plan

- The `get_config` examples of tasks 35–78 show the key (separate commit).

## Tests

- examples E79.1–E79.10 (fixture `detail`)
- unit: `request::tests::output`

## Limitations and known gaps

- `facets` comes with task 105, the funnel of `count` and higher with task 102.

## Try it

`{ "query": "text has 'dbg!('", "output": { "detail": "exists" } }`
