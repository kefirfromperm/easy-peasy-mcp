# Task 2: Empty query: all files

Status: implemented, see [2__SUMMARY__EMPTY_QUERY.md](2__SUMMARY__EMPTY_QUERY.md)

Depends on: task 1

## Goal

An empty or whitespace-only `query`, or a request without `query`, returns every file the traversal lets through, like `find -type f`.

## Scope

- `query` missing, `""` or only whitespace (and, after task 8, only comments) is the empty query
- the implicit `type = file` applies: only regular files
- the output has no legend and no `[...]` condition list on the files, since there are no conditions
- the funnel steps of an empty query are defined with task 102

## Spec

- [QUERY.md#basics](../QUERY.md#basics)
- [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`2__EXAMPLES__EMPTY_QUERY.md` covers:

- a missing `query`, an empty string and whitespace only — all return the same files
- a fixture with hidden, ignored and generated files and directories, which are not returned

## Acceptance criteria

- every example in `2__EXAMPLES__EMPTY_QUERY.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
- if the spec doesn't say how a file line without conditions looks, the spec is clarified first
