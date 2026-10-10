# Task 102: `output.funnel`

Status: in progress

Depends on: task 94

## Goal

Explain empty and huge results: how many files are left after each condition.

## Scope

- the key `funnel` in the `[output]` section of the config file, default `"auto"`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.funnel` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `auto`: when empty or more than 10 × `max_files` files; `always`; `never` (also no hints)
- steps of an `and` query: metadata steps first, then content steps, each in query order; brackets and groups are one step; `or` branches with first-match counting; unknown doesn't pass
- `funnel:` after the legend in text, `funnel` in JSON

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#funnel](../OUTPUT.md#funnel)
- [OUTPUT.md#steps](../OUTPUT.md#steps)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`102__EXAMPLES__OUTPUT_FUNNEL.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `102__EXAMPLES__OUTPUT_FUNNEL.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
