# Task 123: Hints for an empty fuzzy result

Status: implemented, see [123__SUMMARY__FUZZY_HINTS.md](123__SUMMARY__FUZZY_HINTS.md)

Depends on: task 103, task 62

## Goal

When `name ~~` or `path ~~` finds nothing, the response tells the agent how to fix the query in one step. Today an
empty result of `~~` has a funnel but no hints, because the variations of
[OUTPUT.md#hints](../OUTPUT.md#hints) don't apply to `~~`.

Found by trying the server: `name ~~ 'qryparser'` returns nothing with the bare funnel, also with
`fuzzy_min_score = 0.2`, while `path ~~ 'qryparser'` finds `src/query/parser.rs` — the agent had the directory in mind,
but `name ~~` only sees the file name.

## Scope

A spec change first (its own commit), then the code:

- `name ~~ 'v'` with 0 files: the variation `path ~~ 'v'`, evaluated and reported like the other variations (the number
  of files, at most three paths)
- `name ~~` / `path ~~` with 0 files: the files whose score is above 0 but below `query.fuzzy_min_score`, best first,
  at most three, with the setting to change: `below query_options.fuzzy_min_score 0.5: parse.rs 0.41, ...`
- the hints in the text and the JSON format, as the other hints

## Out of scope

- hints for `~~` inside `line( ... )` / `near( ... )` (not allowed there)
- changing the fuzzy score

## Spec

- [OUTPUT.md#hints](../OUTPUT.md#hints)
- [QUERY.md#--fuzzy](../QUERY.md#--fuzzy)
- [CONFIG.md#query](../CONFIG.md#query)

## Examples

`123__EXAMPLES__FUZZY_HINTS.md` covers:

- `name ~~` with 0 files which `path ~~` would match
- `name ~~` / `path ~~` with matches only below the threshold
- `~~` with 0 files and no hint at all (no character sequence anywhere)
- the hints of a `~~` branch of `or`
- the JSON format

## Acceptance criteria

- the spec describes the hints before the code is written
- every example in `123__EXAMPLES__FUZZY_HINTS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description mentions the hint where it lists the hints, if it does
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
