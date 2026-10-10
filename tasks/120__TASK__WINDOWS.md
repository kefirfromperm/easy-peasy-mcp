# Task 120: Windows specifics

Status: implemented, see [120__SUMMARY__WINDOWS.md](120__SUMMARY__WINDOWS.md)

Depends on: task 118, task 47, task 52, task 9

## Goal

The server follows Windows conventions.

## Scope

- output paths with `\`, the `\\?\` prefix never shown
- `\` accepted as a separator in query paths; wildcard escapes with brackets
- `path_case = native` is case-insensitive
- hidden attribute
- junctions are `link`
- duplicates by volume serial and file index
- `%APPDATA%` config directory, `%USERPROFILE%` for `~`
- TOML paths in single quotes

## Spec

- [CONFIG.md#platforms](../CONFIG.md#platforms)
- [QUERY.md#targets](../QUERY.md#targets)
- [OUTPUT.md#paths](../OUTPUT.md#paths)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`120__EXAMPLES__WINDOWS.md` covers:

- Windows-only examples marked as such

## Acceptance criteria

- every example in `120__EXAMPLES__WINDOWS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
