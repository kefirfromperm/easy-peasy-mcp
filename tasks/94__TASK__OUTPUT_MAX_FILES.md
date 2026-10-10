# Task 94: `output.max_files`

Status: in progress

Depends on: task 92

## Goal

Limit the files in the response.

## Scope

- the key `max_files` in the `[output]` section of the config file, default `100`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.max_files` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `truncated: yes (max_files)` in the header, `truncated` / `truncated_by` in JSON, real totals
- the cursor comes with task 101

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`94__EXAMPLES__OUTPUT_MAX_FILES.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `94__EXAMPLES__OUTPUT_MAX_FILES.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
