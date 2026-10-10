# Summary of task 49: `traversal.git_excludes`

Server version: `0.49.0`.

## What was built

- `traversal.git_excludes` (CONFIG.md#traversal, CONFIG.md#ignore-files), default `true`: from the config file,
  shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `traversal.git_excludes`; invalid values like every boolean key.
- With `true` the walker respects `.git/info/exclude` and the global excludes file of git (`core.excludesFile`); with
  `false` neither.

## Architecture

- `src/config.rs`: `Settings.git_excludes` with its source, `Mutable::GitExcludes` (added by a script for the
  changeable keys of the design of task 48).
- `src/search/mod.rs`: `SearchOptions.git_excludes` → `WalkBuilder::git_exclude` and `git_global`; the test hook
  `EngineOptions.git_global_excludes` keeps the machine's global excludes out of the tests.
- `src/request.rs`: `Traversal.git_excludes`.

## Deviations from the plan

- The `get_config` examples of tasks 35–48 show the key (separate commit).

## Tests

- examples E49.1–E49.7 (fixture `excludes`: the setup makes a repository with `.git/info/exclude`)
- unit: `request::tests::traversal`, the file values

## Limitations and known gaps

None.

## Try it

`{ "query": "name ~ '*.tmp'", "traversal": { "git_excludes": false } }`
