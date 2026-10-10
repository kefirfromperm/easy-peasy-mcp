# Task 101: `output.cursor`: pagination

Status: in progress

Depends on: task 94, task 95, task 96, task 100

## Goal

Get the next page of a truncated response: repeat the request with `output.cursor`.

## Scope

- `output.cursor` in the request only; it is not a configuration key
- `next cursor: ...` in the header, `next_cursor` in JSON, only when a cursor limit was hit (not for per file limits and the timeout)
- an opaque cursor with the request hash, the position and the totals; no paths
- every page runs the search again
- `cursor_mismatch`, `cursor_invalid`, the `results_changed` warning

## Spec

- [OUTPUT.md#pagination](../OUTPUT.md#pagination)
- [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`101__EXAMPLES__OUTPUT_CURSOR.md` covers:

- two and three pages, a cursor with another request, a broken cursor, files changed between pages

## Acceptance criteria

- every example in `101__EXAMPLES__OUTPUT_CURSOR.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
