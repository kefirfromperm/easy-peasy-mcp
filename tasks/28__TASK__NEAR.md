# Task 28: Proximity `near( ... )`

Status: in progress

Depends on: task 26

## Goal

Find lines close to each other: `near(text has '.lock()', text has '.await', 3)`.

## Scope

- `near(A, B, N [, order] [, report])`, A and B are line scope expressions, `N = 0` the same line
- order `any` (default), `ordered` (B after A, on the same line the B match starts after the A match)
- report `all` (default), `closest` (nearest B per A, earlier on a tie), `first` (only the first pair)
- `report` doesn't change whether the file matches, only the reported lines, fragments and counts
- fragments carry the inner condition id and the `near` id; label after `near( ... )`
- errors `invalid_near_option`, `conflicting_options`; `near` can't be inside `line( ... )`

## Spec

- [QUERY.md#proximity](../QUERY.md#proximity)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)
- [QUERY.md#grammar](../QUERY.md#grammar)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`28__EXAMPLES__NEAR.md` covers:

- each spec example, each option, `N = 0`, ties for `closest`, the errors

## Acceptance criteria

- every example in `28__EXAMPLES__NEAR.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
