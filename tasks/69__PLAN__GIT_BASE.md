# Plan for task 69: `git.base`

## Design

- A base which doesn't exist (the decision of this task, written into CONFIG.md#git and OUTPUT.md#warnings): the base
  is looked up per repository, so it can't fail the request; in a repository without it `git = branch` is `changed`,
  and with a base set by name its files get the new warning `git_base_not_found`. `auto` without any of its branches
  stays silent, as before.
- `src/config/keys.rs`: `Key::Base` in the section `git`, `Type::GitBase` — `auto` or a non-empty name which doesn't
  start with `-` (it is passed to `git` as an argument, so it must never be an option); default `"auto"`. The request
  object `git` holds the keys of the section.
- `src/git.rs`: `GitIndex::new(isolated, base)`; `Repo.base_missing`; `branch_files` verifies a named base with
  `rev-parse --verify -q <base>^{commit}`; `GitIndex::base_missing(path)`.
- `src/warning.rs`: `WarningCode::GitBaseNotFound`, after `not_a_git_repo`.
- `src/search/mod.rs`: `SearchOptions.git_base`; `git_status` gives the warning.
- `src/description.rs`: the `git` object in the input schema.

## Tests

- examples E69.1–E69.8 (fixture `gitbase`: `main` → `develop` → `feature`, an untracked file)
- unit: the base type, the request field

## Steps

1. failing tests
2. key and type, git index, warning

## Risks

- none
