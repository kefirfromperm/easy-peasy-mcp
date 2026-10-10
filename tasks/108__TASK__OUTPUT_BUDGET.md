# Task 108: `output.budget`

Status: in progress

Depends on: task 101, task 105, task 98

## Goal

Fit the response into N tokens without guessing limits: `"budget": 1500`.

## Scope

- the key `budget` in the `[output]` section of the config file, default not set
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.budget` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the levels: as requested, without context, lines cut to 120, `files` with a cursor if at least 10 files fit, `facets`, `count`
- never raises the detail; `max_*` limits still apply
- `detail: files (budget 2000)` in the header, `detail_used` in JSON
- minimum 100

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#budget](../OUTPUT.md#budget)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`108__EXAMPLES__OUTPUT_BUDGET.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `108__EXAMPLES__OUTPUT_BUDGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
