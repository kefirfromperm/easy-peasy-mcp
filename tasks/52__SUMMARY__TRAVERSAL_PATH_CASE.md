# Summary of task 52: `traversal.path_case`

Server version: `0.52.0`.

## What was built

- `traversal.path_case` (CONFIG.md#traversal), default `"native"`, one of `native`, `sensitive`, `insensitive`: from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `traversal.path_case`.
- It decides the case sensitivity of `=`, `has`, `starts`, `ends`, `~`, `=~` on `path` and `name`, of `exclude` and
  of the wildcards of the kinds; `native` is case-sensitive on Linux and case-insensitive on macOS and Windows. `/i`,
  `~~` and `/n` stay case-insensitive.
- Errors (spec added in its own commit for every key with a fixed set of values): `invalid value for
  'traversal.path_case': expected one of: native, sensitive, insensitive` (file, `update_config` hint), in a request
  `'traversal.path_case' must be one of: native, sensitive, insensitive`.

## Architecture

- `src/config/keys.rs`: `Key::PathCase`, `Type::Choice` (a string of a fixed set).
- `src/engine.rs`: the setting → `case_insensitive_paths` for the query, the walk (kinds) and `exclude`.

## Deviations from the plan

- The `get_config` examples of tasks 35–51 show the key (separate commit).

## Tests

- examples E52.1–E52.10 (fixture `path-case`); E52.1 runs on Linux only, where `native` is case-sensitive
- unit: `config::keys::tests::types_check_toml_and_json` (the choice type), `request::tests::traversal`

## Limitations and known gaps

- The case-insensitive `native` of macOS and Windows is checked by the platform tasks 119–120.

## Try it

`{ "query": "name ~ 'readme*'", "traversal": { "path_case": "insensitive" } }`
