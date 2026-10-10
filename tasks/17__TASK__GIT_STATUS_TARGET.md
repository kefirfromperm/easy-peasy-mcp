# Task 17: `git` target: working tree statuses

Status: not implemented

Depends on: task 16

## Goal

Find files by git status: `git = changed`, `git in (modified, untracked)`, `git = clean`.

## Scope

- values `modified`, `staged`, `untracked`, `conflicted`, `changed`, `clean`; a file can have several
- the repository is found per file: nested repositories and several repositories work
- outside a repository every `git` condition is false and the response has a `not_a_git_repo` warning
- the status is read only when the query has a `git` condition

## Spec

- [QUERY.md#git](../QUERY.md#git)
- [CONFIG.md#git](../CONFIG.md#git)
- [OUTPUT.md#warnings](../OUTPUT.md#warnings)

## Out of scope

- `git = branch` — task 18

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`17__EXAMPLES__GIT_STATUS_TARGET.md` covers:

- git repositories built by the test setup: modified, staged, both, untracked, conflicted, clean
- a directory outside a repository with its warning

## Acceptance criteria

- every example in `17__EXAMPLES__GIT_STATUS_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
