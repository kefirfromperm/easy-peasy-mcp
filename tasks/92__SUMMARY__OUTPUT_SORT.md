# Summary of task 92: `output.sort`

Server version: `0.92.0`.

## What was built

- `output.sort` (CONFIG.md#output, OUTPUT.md#options), default `"relevance"`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by `output.sort`;
  invalid values like every choice key.
- `path`, `modified`, `size`, `matches` with `asc` / `desc`: equal values in ascending path order also with `desc`,
  missing values (the size of a directory, a member without a date) last. The files are sorted right after the
  search, before the result is kept and before any limit.
- `relevance` keeps the path order until task 93.

## Architecture

- `src/output/sort.rs` (new): `Sort::parse`, `Sort::needs_meta`, `sort_files` (a stable sort of the path order).
- `src/engine.rs`: the search computes the metadata also for `modified` and `size`; it is dropped again without
  `meta`.
- `src/config/keys.rs`: `Key::Sort`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–91 show the key and OUTPUT.md says how equal and missing values are ordered
  (separate commits).

## Tests

- examples E92.1–E92.11 (fixture `sorting`)
- unit: `output::sort::tests::by_the_value_then_by_path`, `request::tests::output`

## Limitations and known gaps

- Sorting by size or date reads the language and kind of every matched file too (the metadata is computed whole).

## Try it

`{ "query": "text has 'TODO'", "output": { "detail": "files", "sort": "modified desc" } }`
