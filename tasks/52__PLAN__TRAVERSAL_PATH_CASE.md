# Plan for task 52: `traversal.path_case`

## Design

- `src/config/keys.rs`: `Key::PathCase`, `Type::Choice(&[...])` — a string of a fixed set, `expected one of: ...`
  (file, `update_config` hint) and `must be one of: ...` (request). Default `"native"`.
- `src/engine.rs`: `path_case` → whether paths and names are compared case-insensitively: `native` — the platform
  (`NATIVE_CASE_INSENSITIVE`), `sensitive` — no, `insensitive` — yes. The one value goes to the query (`=`, `has`,
  `starts`, `ends`, `~`, `=~` on `path` and `name`), to `exclude` and to the wildcards of the kinds; `/i`, `~~` and
  `/n` stay case-insensitive as before.

## Tests

- examples E52.1–E52.10 (fixture `path-case`: `README.md`, `readme.txt`, `notes.txt`); E52.1 shows `native` on Linux
- unit: the choice type, the request field

## Steps

1. failing tests
2. key and type, the engine

## Risks

- none: the case sensitivity has been a parameter of the query and the walk since task 3
