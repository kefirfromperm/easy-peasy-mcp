# Task 113: Several queries in one call: `queries`

Status: not implemented

Depends on: task 110, task 108

## Goal

One round trip for several questions: `"queries": [{ "label": "defs", "query": ... }, ...]`.

## Scope

- `queries` instead of `query` (both — `invalid_option`); items with `query`, optional `label`, and overrides of `dirs`, `timeout`, `traversal`, `output`
- other request fields shared
- parallel queries; the same `dirs` and `traversal` share one walk
- `budget` and `max_output` split equally, unused parts go to the next queries; own values are kept
- result ids, legends, funnels per query; `== label ==` section headers (the number without label)
- an invalid query gets its error in its section; `isError` only if all fail

## Spec

- [OUTPUT.md#batch](../OUTPUT.md#batch)
- [TOOLS.md#errors](../TOOLS.md#errors)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`113__EXAMPLES__BATCH.md` covers:

- the spec batch example, a batch with an invalid query, a per-query override

## Acceptance criteria

- every example in `113__EXAMPLES__BATCH.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
