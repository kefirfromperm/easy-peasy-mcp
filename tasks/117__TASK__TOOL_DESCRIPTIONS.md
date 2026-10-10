# Task 117: Final tool descriptions and instructions

Status: implemented, see [117__SUMMARY__TOOL_DESCRIPTIONS.md](117__SUMMARY__TOOL_DESCRIPTIONS.md)

Depends on: task 113, task 115, task 116

## Goal

The agent reads the complete cheat sheet of the spec.

## Scope

- `find`, `get_config`, `update_config` descriptions and input schemas exactly as in TOOLS.md
- the server `instructions` exactly as in TOOLS.md
- `tools/list` annotations as in the spec

## Spec

- [TOOLS.md#tools](../TOOLS.md#tools)
- [TOOLS.md#find](../TOOLS.md#find)
- [TOOLS.md#transport](../TOOLS.md#transport)

## Examples

`117__EXAMPLES__TOOL_DESCRIPTIONS.md` covers:

- `initialize` and `tools/list` responses compared with the spec text

## Acceptance criteria

- every example in `117__EXAMPLES__TOOL_DESCRIPTIONS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
