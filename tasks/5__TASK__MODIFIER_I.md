# Task 5: `/i` modifier on `name` and `path`

Status: not implemented

Depends on: task 4

## Goal

Case-insensitive name and path conditions on every platform: `name = 'readme.md' /i`.

## Scope

- the modifier syntax after a string value: `/x`, several in any order, with or without spaces between them
- `/i` on `name` and `path` with every text operator: Unicode simple case folding
- other modifiers (`/w`, `/m`, `/c`, `/e`, `/n`) fail with `not_implemented` until their tasks; a modifier not allowed for the target / operator fails with `modifier_not_allowed`, the hint as in the spec
- the legend writes the condition with its modifiers as written in the query

## Spec

- [QUERY.md#modifiers](../QUERY.md#modifiers)
- [QUERY.md#errors](../QUERY.md#errors)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`5__EXAMPLES__MODIFIER_I.md` covers:

- `=`, `has`, `~` with `/i`; non-ASCII letters (`Ä`/`ä`)
- an unknown modifier error

## Acceptance criteria

- every example in `5__EXAMPLES__MODIFIER_I.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
