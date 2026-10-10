# Task 27: `len` target

Status: implemented, see [27__SUMMARY__LEN_TARGET.md](27__SUMMARY__LEN_TARGET.md)

Depends on: task 26

## Goal

Find lines by length: `name ~ '*.py' and not line(len > 120)`.

## Scope

- `len` inside `line( ... )` (and `near( ... )` after task 28): line length in Unicode characters without the line break, a tab is one character
- comparison operators and `in a..b` with non-negative integers
- `line_target_outside` outside a group
- `len` never produces fragments; a group of only `len` gives whole-line fragments

## Spec

- [QUERY.md#targets](../QUERY.md#targets)
- [QUERY.md#numbers](../QUERY.md#numbers)
- [QUERY.md#line-scope](../QUERY.md#line-scope)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`27__EXAMPLES__LEN_TARGET.md` covers:

- long lines (under the 300 character cut), empty lines with `len = 0`, tabs and non-ASCII
- `len > 100` outside a group

## Acceptance criteria

- every example in `27__EXAMPLES__LEN_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
