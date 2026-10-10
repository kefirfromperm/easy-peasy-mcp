# Task 24: `/m` multiline regex

Status: not implemented

Depends on: task 22

## Goal

Match across lines: ``text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m``.

## Scope

- `/m` with `=~` on `text` in file scope: the regex runs on the whole content; `\n` matches a line break, `^` / `$` match at line starts and ends, `.` doesn't match `\n` unless `(?s)`
- multiline fragments: `to` in JSON, the marker opens on the first line and closes on the last
- every line of a multiline match is a matched line
- `modifier_not_allowed` for `/m` elsewhere, also inside `line( ... )` once it exists

## Spec

- [QUERY.md#regex](../QUERY.md#regex)
- [QUERY.md#modifiers](../QUERY.md#modifiers)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)
- [OUTPUT.md#highlight](../OUTPUT.md#highlight)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`24__EXAMPLES__MODIFIER_M.md` covers:

- the spec example of a test attribute and the function after it
- `(?s)`, an error with `name`

## Acceptance criteria

- every example in `24__EXAMPLES__MODIFIER_M.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
