# Summary of task 47: `traversal.hidden`

Server version: `0.47.0`.

## What was built

- `traversal.hidden` (CONFIG.md#traversal), default `false`: from the config file, shown by `get_config` with its
  source, changed and reset by `update_config`, replaced for one request by `traversal.hidden`; invalid values like
  every boolean key.
- With `true` the walk includes hidden entries — names starting with `.` — and descends into hidden directories.

## Architecture

- `src/config.rs`: `Settings.hidden` with its source, `FileValues.hidden`, `Mutable::Hidden` (added by a reusable
  script for boolean traversal keys).
- `src/request.rs`: `Traversal.hidden`; `src/search/mod.rs`: `SearchOptions.hidden` → `WalkBuilder::hidden`;
  `src/engine.rs`: the request value, else the settings.

## Deviations from the plan

- The `get_config` examples of tasks 35–46 show the key (separate commit).

## Tests

- examples E47.1–E47.8 (fixture `hidden`)
- unit: `request::tests::traversal`, the file value

## Limitations and known gaps

- The hidden flag of macOS and the hidden attribute of Windows come with tasks 119–120.

## Try it

`{ "query": "name = '.env'", "traversal": { "hidden": true } }`
