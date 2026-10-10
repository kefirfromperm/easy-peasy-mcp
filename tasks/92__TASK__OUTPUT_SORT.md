# Task 92: `output.sort`

Status: implemented, see [92__SUMMARY__OUTPUT_SORT.md](92__SUMMARY__OUTPUT_SORT.md)

Depends on: task 79

## Goal

Order files by path, modification time, size or matches: `"sort": "modified desc"`.

## Scope

- the key `sort` in the `[output]` section of the config file, default `"relevance"`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.sort` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `path`, `modified`, `size`, `matches` with `asc` / `desc`; equal values ordered by the full path
- files are sorted before the limits
- `relevance` until task 93: ordered by path

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#relevance](../OUTPUT.md#relevance)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`92__EXAMPLES__OUTPUT_SORT.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `92__EXAMPLES__OUTPUT_SORT.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
