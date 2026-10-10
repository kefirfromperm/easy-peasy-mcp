# Task 23: `/w` whole word modifier

Status: in progress

Depends on: task 22

## Goal

Match whole words only: `text has 'id' /w` matches `id`, not `valid`.

## Scope

- `/w` with `=`, `has`, `starts`, `ends`, `=~` on `text`: no letter, digit or `_` right before and after the match (Unicode letters and digits)
- `modifier_not_allowed` elsewhere

## Spec

- [QUERY.md#modifiers](../QUERY.md#modifiers)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`23__EXAMPLES__MODIFIER_W.md` covers:

- matches at line start / end, next to punctuation, next to `_` and digits, with `/i`, with `=~`

## Acceptance criteria

- every example in `23__EXAMPLES__MODIFIER_W.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
