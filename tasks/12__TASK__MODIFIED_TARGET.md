# Task 12: `modified` target and dates

Status: in progress

Depends on: task 11

## Goal

Find recently changed files: `modified > -7d`, `modified = 2026-10-01`.

## Scope

- comparison operators and `in a..b` on `modified`
- absolute ISO 8601 dates and date-times, server local time unless an offset is given
- relative dates `-N` with `s`, `m`, `h`, `d`, `w`, relative to the moment the search starts
- a date without a time means the whole day, with the semantics of the table in the spec
- `invalid_date` error

## Spec

- [QUERY.md#dates](../QUERY.md#dates)
- [QUERY.md#comparison-operators](../QUERY.md#comparison-operators)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`12__EXAMPLES__MODIFIED_TARGET.md` covers:

- mtimes set by the test setup; examples use absolute dates or a fixed `now` given to the test server
- each whole-day row of the spec table, a range, an offset, a relative date

## Acceptance criteria

- every example in `12__EXAMPLES__MODIFIED_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
- tests can fix `now` and the local time zone of the server
