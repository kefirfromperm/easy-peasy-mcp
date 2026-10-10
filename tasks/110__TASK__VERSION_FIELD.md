# Task 110: Request field `version`

Status: in progress

Depends on: task 78

## Goal

An agent pins the spec version: `"version": 1`.

## Scope

- `version` in the `find` input schema, an integer ≥ 1
- `1` and a missing field use spec 1
- an unsupported version fails with `unsupported_version` and the supported list
- `"version": 1` in every JSON response; none in text

## Spec

- [TOOLS.md#versioning](../TOOLS.md#versioning)
- [OUTPUT.md#errors](../OUTPUT.md#errors)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`110__EXAMPLES__VERSION_FIELD.md` covers:

- `version: 1`, `version: 2`, JSON response with `version`

## Acceptance criteria

- every example in `110__EXAMPLES__VERSION_FIELD.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
