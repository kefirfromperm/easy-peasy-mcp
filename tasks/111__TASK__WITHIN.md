# Task 111: Refine a previous result: `within`

Status: implemented, see [111__SUMMARY__WITHIN.md](111__SUMMARY__WITHIN.md)

Depends on: task 43, task 39

## Goal

Search only the files of a previous result: `{ "within": "r12", "query": "text has 'retry'" }`.

## Scope

- no directory walk: `dirs` and the walk options are ignored; file and matching options apply
- files are read again; deleted files are skipped
- paths written and matched as in the referred result
- every file is checked against the allowed roots again
- `result_expired` for an unknown or evicted id

## Spec

- [OUTPUT.md#refine-within](../OUTPUT.md#refine-within)
- [CONFIG.md#security](../CONFIG.md#security)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`111__EXAMPLES__WITHIN.md` covers:

- a two-step session: a first query, then `within` its result; an expired id

## Acceptance criteria

- every example in `111__EXAMPLES__WITHIN.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
