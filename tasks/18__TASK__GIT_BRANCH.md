# Task 18: `git = branch`

Status: not implemented

Depends on: task 17

## Goal

Find the files changed on the current branch: `git = branch and kind = source`.

## Scope

- `branch`: `changed` plus the files changed by the commits between the merge base of the base and `HEAD`
- the base `auto`: `origin/HEAD`, `main`, `master`, the first which exists; on the base branch itself `branch` equals `changed`

## Spec

- [QUERY.md#git](../QUERY.md#git)
- [CONFIG.md#git](../CONFIG.md#git)

## Out of scope

- `git.base` configuration — task 69

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`18__EXAMPLES__GIT_BRANCH.md` covers:

- a repository with `main` and a feature branch with commits and uncommitted changes
- the same query on the base branch

## Acceptance criteria

- every example in `18__EXAMPLES__GIT_BRANCH.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
