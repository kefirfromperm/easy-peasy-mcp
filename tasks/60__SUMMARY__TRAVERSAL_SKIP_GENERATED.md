# Summary of task 60: `traversal.skip_generated`

Server version: `0.60.0`.

## What was built

- `traversal.skip_generated` (CONFIG.md#traversal, QUERY.md#generated-files), default `true`: from the config file,
  shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `traversal.skip_generated`; invalid values like every boolean key.
- `true`: files of kind `generated` (by the wildcards of the kinds or a marker in the first lines) are skipped unless
  the query has a `kind` condition; `false`: they are returned like other files. Members of archives follow it too
  (by their path).

## Architecture

- `src/config/keys.rs`: `Key::SkipGenerated`.
- `src/search/mod.rs`: `SearchOptions.skip_generated`, `Walk::skips_generated`.

## Deviations from the plan

- The `get_config` examples of tasks 35–59 show the key (separate commit). The `traversal` section is complete now.

## Tests

- examples E60.1–E60.8 (fixture `generated`)
- unit: `request::tests::traversal`

## Limitations and known gaps

- The kinds rules themselves become configurable with tasks 63–68.

## Try it

`{ "query": "text has 'version'", "traversal": { "skip_generated": false } }`
