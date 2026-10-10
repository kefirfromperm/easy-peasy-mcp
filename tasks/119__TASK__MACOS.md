# Task 119: macOS specifics

Status: not implemented

Depends on: task 118, task 47, task 52

## Goal

The server follows macOS conventions.

## Scope

- `path_case = native` is case-insensitive
- hidden: a name starting with `.` or the `hidden` flag
- decomposed (NFD) file names match composed queries (NFC)
- the macOS config directory
- `created` available

## Spec

- [CONFIG.md#platforms](../CONFIG.md#platforms)
- [QUERY.md#targets](../QUERY.md#targets)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`119__EXAMPLES__MACOS.md` covers:

- macOS-only examples marked as such

## Acceptance criteria

- every example in `119__EXAMPLES__MACOS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
