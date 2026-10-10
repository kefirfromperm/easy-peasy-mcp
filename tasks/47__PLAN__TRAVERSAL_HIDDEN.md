# Plan for task 47: `traversal.hidden`

## Design

- `src/config.rs`: `traversal.hidden` (a boolean) in the file values, `Settings` with its source and as a `Mutable`
  key; `get_config` shows it.
- `src/request.rs`: `traversal.hidden`.
- `src/search/mod.rs`: `SearchOptions.hidden` → `WalkBuilder::hidden(!hidden)`: a name starting with `.` is hidden
  (Linux); the platform flags come with tasks 119–120.
- `src/engine.rs`: the request value, else the settings.

## Tests

- examples E47.1–E47.8 (fixture `hidden`)
- unit: the request field, the file value

## Steps

1. failing tests
2. key, request field, walker

## Risks

- none
