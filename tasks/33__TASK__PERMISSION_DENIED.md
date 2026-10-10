# Task 33: Unreadable entries

Status: in progress

Depends on: task 20

## Goal

Files and directories the server can't read are skipped and reported instead of silently missing.

## Scope

- unreadable files and directories are skipped with a `permission_denied` warning; a directory path ends with `/` in the warning
- the server never tries to change permissions

## Spec

- [OUTPUT.md#warnings](../OUTPUT.md#warnings)
- [CONFIG.md#security](../CONFIG.md#security)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`33__EXAMPLES__PERMISSION_DENIED.md` covers:

- permissions removed by the test setup (Unix only; the test is skipped where permissions can't be removed, e.g. running as root)

## Acceptance criteria

- every example in `33__EXAMPLES__PERMISSION_DENIED.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
