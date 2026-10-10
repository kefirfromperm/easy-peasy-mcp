# Task 10: `empty` target

Status: in progress

Depends on: task 9

## Goal

Find empty files and empty directories: `empty`, `type = dir and empty`.

## Scope

- `empty` stands alone, without operator and value
- true for a file of size 0 and a directory without entries
- whether ignored or hidden entries make a directory non-empty is decided in the plan and written into the spec

## Spec

- [QUERY.md#targets](../QUERY.md#targets)
- [QUERY.md#grammar](../QUERY.md#grammar)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`10__EXAMPLES__EMPTY_TARGET.md` covers:

- empty files, `not empty`, empty directories created by the test setup

## Acceptance criteria

- every example in `10__EXAMPLES__EMPTY_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
