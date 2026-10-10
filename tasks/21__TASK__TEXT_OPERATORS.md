# Task 21: `text` with `=`, `starts`, `ends`

Status: implemented, see [21__SUMMARY__TEXT_OPERATORS.md](21__SUMMARY__TEXT_OPERATORS.md)

Depends on: task 19

## Goal

Match the whole content, its start or its end: `text starts '#!/bin/bash'`.

## Scope

- `=`, `starts`, `ends` on `text` in file scope work on the whole content
- the fragment is the matched part, which may span lines
- `operator_not_allowed` for `~` and `~~` on `text` with the spec hint

## Spec

- [QUERY.md#text-operators](../QUERY.md#text-operators)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`21__EXAMPLES__TEXT_OPERATORS.md` covers:

- a shebang with `starts`, a trailing newline with `ends`, `=` on a small file
- `text ~ '*foo*'` error

## Acceptance criteria

- every example in `21__EXAMPLES__TEXT_OPERATORS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
- if the spec doesn't define how a fragment spanning lines is shown without `/m` (`to`), the spec is clarified first
