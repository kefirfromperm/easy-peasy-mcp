# Summary of task 44: `traversal.min_depth`

Server version: `0.44.0`.

## What was built

- `traversal.min_depth` (CONFIG.md#traversal), default `0`: from the config file, shown by `get_config` in the new
  `[traversal]` section with its source, changed and reset by `update_config`, replaced for one request by
  `traversal.min_depth` of the request.
- The request object `traversal` (TOOLS.md#input-schema) with this key: `'traversal' must be an object`,
  `'traversal.min_depth' must be a whole number`, `unknown field 'traversal.<key>'` with `did you mean` and the keys of
  the section; the other keys of the spec are `not_implemented` until their tasks (spec clarified first).
- Invalid in the file / `update_config`: `invalid value for 'traversal.min_depth': a whole number is expected`.
- Entries closer to a working directory than `min_depth` are not returned (its children are depth 1); the walk goes on
  below them.

## Architecture

- `src/config.rs`: `Settings.min_depth` with its source, `FileValues.min_depth`, `Mutable::MinDepth`; mutable keys of
  any section are read from the config file by one rule; `get_config` renders `[traversal]`; `section_keys`.
- `src/request.rs`: `Traversal`, `parse_traversal`.
- `src/search/mod.rs`: `SearchOptions.min_depth`; `src/engine.rs`: the request value, else the settings.
- `src/description.rs`: `traversal.min_depth` in the schema and the description.

## Deviations from the plan

- The `get_config` examples of tasks 35–43 show the traversal section (separate commit).

## Tests

- examples E44.1–E44.7 (fixture `depth`)
- unit: `request::tests::traversal`, the file value in `config::tests::roots_are_resolved`, the rendering in
  `get_config_arguments`

## Limitations and known gaps

- The other traversal keys come with tasks 45–59.

## Try it

`{ "query": "name ~ '*.md'", "traversal": { "min_depth": 2 } }`
