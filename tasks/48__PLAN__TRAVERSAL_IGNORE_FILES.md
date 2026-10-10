# Plan for task 48: `traversal.ignore_files`

## Design

- `src/config.rs`: `traversal.ignore_files` (a list of names, may be empty) in the file values, `Settings` with its
  source and as a `Mutable` key; `get_config` shows it.
- `src/request.rs`: `traversal.ignore_files` (`'traversal.ignore_files' must be a list of strings`).
- `src/search/mod.rs`: `SearchOptions.ignore_files` replaces the constant list: every name is added to the walker as
  a custom ignore file name, in order — the walker looks it up in every directory, also outside of git, a deeper file
  wins and at the same level a later name wins; `[]` adds none.
- `src/engine.rs`: the request value, else the settings.

## Tests

- examples E48.1–E48.9 (fixture `ignores`; its `*.log` files are committed with `git add -f`)
- unit: the request field, the file value

## Steps

1. failing tests
2. key, request field, walker

## Risks

- none: the walker has done the lookup since task 1, only the list becomes configurable
