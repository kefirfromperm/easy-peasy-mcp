# Task 89: `output.path_fragments`

Status: implemented, see [89__SUMMARY__OUTPUT_PATH_FRAGMENTS.md](89__SUMMARY__OUTPUT_PATH_FRAGMENTS.md)

Depends on: task 78

## Goal

Show which part of the path a `path` / `name` condition matched.

## Scope

- the key `path_fragments` in the `[output]` section of the config file, default `false`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.path_fragments` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `path_fragments` per file in JSON; the text form is decided in the plan and written into the spec if missing
- path fragments are not counted as matches

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#fields](../OUTPUT.md#fields)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`89__EXAMPLES__OUTPUT_PATH_FRAGMENTS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `89__EXAMPLES__OUTPUT_PATH_FRAGMENTS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
