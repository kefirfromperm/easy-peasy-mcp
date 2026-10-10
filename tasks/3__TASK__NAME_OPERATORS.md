# Task 3: `name` with `has`, `starts`, `ends`

Status: not implemented

Depends on: task 1

## Goal

Find files by a part of the name: `name has 'config'`, `name starts 'test_'`, `name ends '.test.ts'`.

## Scope

- operators `has`, `starts`, `ends` on `name`, case sensitivity by the platform (`path_case = native`)
- Unicode NFC comparison, as for `=`
- `unknown_operator` lists the full spec operators, as in task 1

## Spec

- [QUERY.md#text-operators](../QUERY.md#text-operators)
- [QUERY.md#targets](../QUERY.md#targets)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`3__EXAMPLES__NAME_OPERATORS.md` covers:

- each operator, a match at the start / middle / end of the name, a value with an escape
- a value which matches a directory name only — directories are not returned

## Acceptance criteria

- every example in `3__EXAMPLES__NAME_OPERATORS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
