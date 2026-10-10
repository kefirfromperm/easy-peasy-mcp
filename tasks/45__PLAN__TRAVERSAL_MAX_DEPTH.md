# Plan for task 45: `traversal.max_depth`

## Design

- `src/config.rs`: `traversal.max_depth`, a whole number or not set, in the file values, `Settings` with its source
  and as a `Mutable` key. `Settings::value` returns `None` for a value which is not set; the lines of `get_config` and
  of the `update_config` result write it as `# max_depth =  (not set)`, the comments as `(not set)`.
- `src/request.rs`: `traversal.max_depth`.
- `src/search/mod.rs`: `SearchOptions.max_depth` → `WalkBuilder::max_depth` (the children of the working directory
  are depth 1, like the walker counts).
- `src/engine.rs`: the request value, else the settings.

## Tests

- examples E45.1–E45.8 (fixture `depth`)
- unit: the request field, the file value, the not-set lines

## Steps

1. failing tests
2. key, request field, walker

## Risks

- none
