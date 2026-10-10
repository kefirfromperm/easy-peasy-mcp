# Task 99: `output.max_output_per_file`

Status: implemented, see [99__SUMMARY__OUTPUT_MAX_OUTPUT_PER_FILE.md](99__SUMMARY__OUTPUT_MAX_OUTPUT_PER_FILE.md)

Depends on: task 97

## Goal

Limit the characters of one file's lines and context.

## Scope

- the key `max_output_per_file` in the `[output]` section of the config file, default `4000`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.max_output_per_file` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the file gets `(cut)`, the next file is processed
- minimum 100

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`99__EXAMPLES__OUTPUT_MAX_OUTPUT_PER_FILE.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `99__EXAMPLES__OUTPUT_MAX_OUTPUT_PER_FILE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
