# Summary of task 17: `git` target: working tree statuses

Server version: `0.17.0`.

## What was built

- `git` with `=`, `!=`, `in ( ... )` and the values `modified`, `staged`, `untracked`, `conflicted`, `changed`,
  `clean`, in any case (QUERY.md#git); a file can have several statuses.
- The repository is found per file (the nearest `.git` upwards), so nested and several repositories work.
- Outside of a repository every `git` condition is false, also `!=`, and the file gets `not_a_git_repo`;
  directories have no git status (spec clarified).
- `git = branch` is `not_implemented` until task 18; `unknown_value` for other values.
- The status is read only for queries with a `git` condition, by the `git` program with `core.fsmonitor` off (spec
  clarified); without git every file is outside of a repository.
- The `find` description shows `git`.
- Fixture setups run git with a fixed identity and no user configuration (`git <dir> <args>`, `git!`).

## Architecture

- `git.rs`: `GitValue`, `GitStatus`, `GitIndex` (repositories cached per directory for one search), the porcelain
  v2 parser.
- `Filter::Git`, `FileValues.git`, `Needs.git`; the subset check marks the value `branch`.
- `search`: the per-entry work is in `Walk::visit` with `dates` and `git_status`; `warn` collects warnings.

## Deviations from the plan

- the walk was restructured into the `Walk` struct to keep functions short.

## Tests

- examples E17.1–E17.10 (fixture `gitrepo` built by git in the setup)
- unit: porcelain v2 records, status values, repositories per file, `branch` in the subset

## Limitations and known gaps

- `git = branch` (task 18); the `git` facet (task 105)
- `git status` may run clean filters of `.gitattributes` for files whose stat changed

## Try it

`{ "query": "git = changed and text has 'dbg!('" }` (the `text` part comes with task 19); now
`{ "query": "git in (modified, untracked)" }`
