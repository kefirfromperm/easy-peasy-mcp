# Task 91: `output.meta`

Status: in progress

Depends on: task 79, task 16

## Goal

File metadata in the list: type, size, date, language, kind.

## Scope

- the key `meta` in the `[output]` section of the config file, default `false`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.meta` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- text: `file 2.1K 2026-10-08 14:03 rust source` after the counts; JSON: `type`, `size`, `modified` (ISO 8601 with offset), `lang`, `kind`
- the score comes with task 93

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#fields](../OUTPUT.md#fields)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`91__EXAMPLES__OUTPUT_META.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `91__EXAMPLES__OUTPUT_META.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
