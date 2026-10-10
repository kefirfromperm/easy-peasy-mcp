# Summary of task 69: `git.base`

Server version: `0.69.0`.

## What was built

- `git.base` (CONFIG.md#git, QUERY.md#git), default `"auto"`: the base of `git = branch`, a branch, tag or commit;
  from the config file, shown by `get_config` (a new section `[git]`) with its source, changed and reset by
  `update_config`, replaced for one request by the new request object `git`.
- `auto` takes `origin/HEAD`, `main` or `master`, the first which exists, as before; a base set by name is verified
  in every repository of the search (`rev-parse --verify <base>^{commit}`).
- A base which doesn't exist (the decision of this task, spec added in its own commit): in that repository
  `git = branch` is `changed`, and with a base set by name its files get the new warning `git_base_not_found`; `auto`
  without any of its branches stays silent.
- A base is `auto` or a non-empty name which doesn't start with `-`, so it can never be an option of `git`:
  `invalid value for 'git.base': "auto" or a branch, tag or commit is expected`, in a request `'git.base' must be
  "auto" or a branch, tag or commit`.

## Architecture

- `src/config/keys.rs`: `Key::Base`, `Type::GitBase`.
- `src/git.rs`: `GitIndex::new(isolated, base)`, `Repo.base_missing`, `GitIndex::base_missing`; `branch_files`
  returns `None` for a missing named base.
- `src/warning.rs`: `WarningCode::GitBaseNotFound`, after `not_a_git_repo`.
- `src/search/mod.rs`: `SearchOptions.git_base`, the warning in `git_status`.
- `src/description.rs`: the `git` object in the input schema.

## Deviations from the plan

- The full `get_config` examples of tasks 35–43 show the new section (separate commit).
- Two unit tests of the config module used `git` as the example of a section without an implemented key; they use
  `output` now.

## Tests

- examples E69.1–E69.8 (fixture `gitbase`)
- unit: `request::tests::git`

## Limitations and known gaps

None.

## Try it

`{ "query": "git = branch and kind = source", "git": { "base": "develop" } }`
