# Task 29: `/c` ignore matches in comments

Status: in progress

Depends on: task 26

## Goal

Search the code, not the comments: `text has 'unsafe' /c`.

## Scope

- `/c` on every operator on `text`; the comment syntax from the built-in language table (line comments, block comments, nested block comments, string delimiters)
- a comment marker inside a string literal is not a comment
- a match before a line comment on the same line counts
- files of an unknown language are searched as is

## Spec

- [QUERY.md#comments](../QUERY.md#comments)
- [CONFIG.md#comment-syntax](../CONFIG.md#comment-syntax)
- [CONFIG.md#built-in-languages](../CONFIG.md#built-in-languages)

## Out of scope

- changing the comment syntax — tasks 73–76

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`29__EXAMPLES__MODIFIER_C.md` covers:

- the spec Rust example, a nested block comment in Rust, a Python `#` comment, a string with `//`
- `not text has 'x' /c`

## Acceptance criteria

- every example in `29__EXAMPLES__MODIFIER_C.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
