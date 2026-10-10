# Plan for task 49: `traversal.git_excludes`

## Design

- `src/config.rs`: `traversal.git_excludes` (a boolean, default `true`) as a changeable key; `get_config` shows it.
- `src/request.rs`: `traversal.git_excludes`.
- `src/search/mod.rs`: `SearchOptions.git_excludes` → `WalkBuilder::git_exclude` (`.git/info/exclude`) and
  `WalkBuilder::git_global` (`core.excludesFile` of the global git config); the test hook which keeps the machine's
  global excludes out of the tests stays (`EngineOptions.git_global_excludes`).
- `src/engine.rs`: the request value, else the settings.

## Tests

- examples E49.1–E49.7 (fixture `excludes`: a repository with `.git/info/exclude`)
- unit: the request field

## Steps

1. failing tests
2. key, request field, walker

## Risks

- none
