# Task 106: `output.facet_depth`

Status: implemented, see [106__SUMMARY__OUTPUT_FACET_DEPTH.md](106__SUMMARY__OUTPUT_FACET_DEPTH.md)

Depends on: task 105

## Goal

Directory depth of the `dir` facet.

## Scope

- the key `facet_depth` in the `[output]` section of the config file, default `2`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.facet_depth` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- files higher up are counted in their own directory

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#facets](../OUTPUT.md#facets)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`106__EXAMPLES__OUTPUT_FACET_DEPTH.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `106__EXAMPLES__OUTPUT_FACET_DEPTH.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
