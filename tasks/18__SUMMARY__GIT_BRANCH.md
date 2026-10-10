# Summary of task 18: `git = branch`

Server version: `0.18.0`.

## What was built

- `git = branch` (QUERY.md#git, CONFIG.md#git): `changed`, or changed by the commits between the merge base of the
  base and `HEAD`. The base `auto` is the first of `origin/HEAD`, `main`, `master` which exists; on the base branch
  itself and without a base `branch` is `changed`.
- A file changed only by commits of the branch is still `clean` for the working tree values.
- The `find` description shows `branch`.

## Architecture

`GitIndex::branch_files` runs `rev-parse --verify`, `merge-base` and `diff --name-only -z` once per repository;
`GitStatus.branch` marks those files.

## Deviations from the plan

None.

## Tests

- examples E18.1–E18.4 (fixtures `gitfeature` and `gitmain`)
- unit: `has(Branch)` for committed, uncommitted and clean files

## Limitations and known gaps

- the base can't be configured yet (task 69)

## Try it

`{ "query": "git = branch and kind = source" }`
