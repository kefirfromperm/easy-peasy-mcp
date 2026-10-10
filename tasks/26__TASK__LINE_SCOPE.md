# Task 26: Line scope `line( ... )`

Status: in progress

Depends on: task 22

## Goal

Several conditions on the same line: `line(text has 'serde' and text has 'tokio')`, and line negation like `grep -v`.

## Scope

- `line( ... )`: the expression must be true for one line; the file matches if at least one line does
- `not` inside negates per line, `not line(...)` means no line matches
- several groups are independent
- only `text` inside; `file_target_inside`, `nested_line` errors; `line_target_outside` arrives with `len` (task 27)
- the group gets an id and an optional label; its fragments carry the condition id and the group id
- a group without positive text conditions produces one whole-line fragment with the group id
- the legend writes the group with its inner conditions; inner conditions also get their own legend lines

## Spec

- [QUERY.md#line-scope](../QUERY.md#line-scope)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)
- [QUERY.md#labels-and-condition-ids](../QUERY.md#labels-and-condition-ids)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`26__EXAMPLES__LINE_SCOPE.md` covers:

- two conditions on one line vs in one file, `not` inside, `not line(...)`, two groups
- the spec `nginx.conf` example (after task 27 adds `len`), labels inside a group
- `line(name ~ '*.rs')`, `line(line(...))`

## Acceptance criteria

- every example in `26__EXAMPLES__LINE_SCOPE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
