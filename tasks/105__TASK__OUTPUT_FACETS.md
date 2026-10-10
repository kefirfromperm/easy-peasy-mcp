# Task 105: `output.facets`

Status: not implemented

Depends on: task 79, task 16, task 17

## Goal

A cheap map of where the matches are: `"detail": "facets"`.

## Scope

- the key `facets` in the `[output]` section of the config file, default `["dir", "lang", "kind", "condition"]`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.facets` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `detail = facets`: totals, legend and facets
- facets `dir`, `ext`, `lang`, `kind`, `git`, `condition`; values sorted by count
- the `git` facet reads the git status even without a `git` condition
- `facets` in JSON

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#facets](../OUTPUT.md#facets)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`105__EXAMPLES__OUTPUT_FACETS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `105__EXAMPLES__OUTPUT_FACETS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
