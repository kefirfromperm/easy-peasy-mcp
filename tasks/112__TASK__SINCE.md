# Task 112: Changes since a previous result: `since`

Status: in progress

Depends on: task 111

## Goal

Check progress while editing: `{ "since": "r12", "query": ... }` returns only the differences.

## Scope

- `+` added, `-` removed (also deleted), `~` changed (different matched lines by the fingerprint)
- the header `since: r12  unchanged: N`; `change`, `since`, `unchanged` in JSON
- changed files show all current matched lines, removed files only the path; counts of the current result
- `result_expired`

## Spec

- [OUTPUT.md#changes-since](../OUTPUT.md#changes-since)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`112__EXAMPLES__SINCE.md` covers:

- a session where the test edits, adds and deletes fixture files between two requests

## Acceptance criteria

- every example in `112__EXAMPLES__SINCE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
