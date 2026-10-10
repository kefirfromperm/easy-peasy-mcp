# Task 81: `output.before`

Status: in progress

Depends on: task 80

## Goal

Lines before matches like `grep -B`.

## Scope

- the key `before` in the `[output]` section of the config file, default equal to `context`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.before` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- not set in the config file means equal to `context`; `get_config` writes it as a comment

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`81__EXAMPLES__OUTPUT_BEFORE.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `81__EXAMPLES__OUTPUT_BEFORE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
