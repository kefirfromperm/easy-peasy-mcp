# Summary of task 45: `traversal.max_depth`

Server version: `0.45.0`.

## What was built

- `traversal.max_depth` (CONFIG.md#traversal), not set by default (unlimited): from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `traversal.max_depth`; invalid values like `min_depth`.
- The walk doesn't descend deeper than `max_depth`; the children of a working directory are depth 1.
- A value which is not set (spec clarified first): `# max_depth =  (not set)` in `get_config` and in the
  `update_config` result, `(not set)` in a `# was` comment; `reset` returns to not set.

## Architecture

- `src/config.rs`: `Settings.max_depth: Option<usize>` with its source, `FileValues.max_depth`,
  `Mutable::MaxDepth`; `Settings::value` is `None` for a value which is not set, `Settings::line` and `key_line`
  write the lines of `get_config` and of the result.
- `src/request.rs`: `Traversal.max_depth`, `whole_number`.
- `src/search/mod.rs`: `SearchOptions.max_depth` → `WalkBuilder::max_depth`; `src/engine.rs`: the request value, else
  the settings.

## Deviations from the plan

- The `get_config` examples of tasks 35–44 show the key (separate commit).

## Tests

- examples E45.1–E45.8 (fixture `depth` of task 44)
- unit: `config::tests::values_which_are_not_set`, `request::tests::traversal`, the file value

## Limitations and known gaps

None.

## Try it

`{ "query": "name ~ '*.rs'", "traversal": { "max_depth": 2 } }`
