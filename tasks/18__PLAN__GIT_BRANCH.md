# Plan for task 18: `git = branch`

## Design

- `git.rs`: when a repository is read, also the files of the branch: the base `auto` is the first of `origin/HEAD`,
  `main`, `master` which `git rev-parse --verify -q` finds; `git merge-base <base> HEAD`; the files of
  `git diff --name-only -z --no-renames <merge base> HEAD`. On the base branch the merge base is `HEAD` and the diff
  is empty, so `branch` is `changed`. Without a base `branch` is `changed`.
- `GitStatus.branch` — changed by the commits of the branch; `has(Branch)` is `changed` or `branch`; a committed file
  of the branch is still `clean` for the working tree values.
- a tracked file which only the branch changed gets a status entry too.
- `query/subset.rs`: `branch` is supported.
- `description.rs`: `git = branch`.

## Tests

- examples E18.1–E18.4 (fixtures `gitfeature`, `gitmain`)
- unit: `has(Branch)` for a committed change, an uncommitted change and a clean file

## Steps

1. failing unit tests
2. base, merge base, diff; subset; description

## Risks

None known; `git.base` (task 69) makes the base configurable.
