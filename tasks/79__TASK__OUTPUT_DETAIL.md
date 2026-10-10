# Task 79: `output.detail`

Status: implemented, see [79__SUMMARY__OUTPUT_DETAIL.md](79__SUMMARY__OUTPUT_DETAIL.md)

Depends on: task 78

## Goal

Ask cheap questions: `exists`, `count`, `files` before `lines`.

## Scope

- the key `detail` in the `[output]` section of the config file, default `"lines"`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.detail` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `exists`: `exists: yes|no`, stops at the first match
- `count`: totals only, with the result id
- `files`: totals, legend and the file list without lines
- `lines`: as before
- `facets` fails with `not_implemented` until task 105

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#detail-levels](../OUTPUT.md#detail-levels)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`79__EXAMPLES__OUTPUT_DETAIL.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `79__EXAMPLES__OUTPUT_DETAIL.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
