# Task 31: `/n` any naming style

Status: implemented, see [31__SUMMARY__MODIFIER_N.md](31__SUMMARY__MODIFIER_N.md)

Depends on: task 23

## Goal

Find an identifier in any naming style: `text has 'parse query' /n` matches `parseQuery`, `parse_query`, `PARSE_QUERY`.

## Scope

- word splitting of the value at spaces, `_`, `-`, `.` and case changes; uppercase runs are one word; digits stay with the preceding word
- matching camel, Pascal, snake, screaming snake, kebab, dot, words and flat styles, case-insensitive
- `/n/w` matches the whole identifier
- `/n` on `text`, `path`, `name` with `=`, `has`, `starts`, `ends`; not with `~`, `~~`, `=~`, not with `/e`

## Spec

- [QUERY.md#n--any-naming-style](../QUERY.md#n--any-naming-style)
- [QUERY.md#modifiers](../QUERY.md#modifiers)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`31__EXAMPLES__MODIFIER_N.md` covers:

- every style of the spec table, `HTTPServer`, `utf8Decode`, `/n/w`, `name has 'config loader' /n`

## Acceptance criteria

- every example in `31__EXAMPLES__MODIFIER_N.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
