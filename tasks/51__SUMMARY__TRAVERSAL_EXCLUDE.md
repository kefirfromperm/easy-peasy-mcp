# Summary of task 51: `traversal.exclude`

Server version: `0.51.0`.

## What was built

- `traversal.exclude` (CONFIG.md#traversal), default `[]`: wildcards of paths which are never walked, relative to
  the working directory and matched like `path ~`; from the config file, shown by `get_config` with its source,
  changed and reset by `update_config`, replaced for one request by `traversal.exclude` (a request list replaces the
  configured one).
- An entry is excluded when a wildcard matches its path, a directory also when a wildcard matches its path followed
  by `/`: `**/target/**` and `target/**` prune the directory `target` itself, it is neither read nor returned by
  `type = dir` (spec clarified in its own commit).
- Errors (spec added in its own commit): `invalid value for 'traversal.exclude': a list of wildcards is expected`,
  `... invalid wildcard '[a-': unclosed '['` (file, `update_config` hint); in a request `'traversal.exclude' must be
  a list of strings`, `invalid wildcard '[a-' in 'traversal.exclude'` with the hint `unclosed '['`.
- Case sensitivity: the native one of the platform until `path_case` (task 52).

## Architecture

- `src/config/keys.rs`: `Key::Exclude`, `Type::Wildcards`; `Type::check_toml` / `check_json` check the shape and
  then the value (`Problem::Wildcard`), so a type can reject a value of the right shape with its own message.
- `src/engine.rs`: the wildcards compiled with `query::wildcard::compile` (NFC).
- `src/search/mod.rs`: `SearchOptions.exclude`; one `filter_entry` of the walker for the exclusions and the symlinks
  outside of the allowed roots (`is_excluded`, `excluded`).

## Deviations from the plan

- The `get_config` unit test checks only the start of the full configuration; the examples show it in full.
- The `get_config` examples of tasks 35–50 show the key (separate commit).

## Tests

- examples E51.1–E51.11 (fixture `exclude`)
- unit: `search::tests::excluded_paths_and_directories`, `config::keys::tests::types_check_toml_and_json`,
  `request::tests::traversal`, `config::tests::valid_files`

## Limitations and known gaps

- `path_case` comes with task 52.

## Try it

`{ "query": "text has 'TODO'", "traversal": { "exclude": ["**/target/**", "**/node_modules/**"] } }`
