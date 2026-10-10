# Task 103: Hints for empty results

Status: in progress

Depends on: task 102

## Goal

An empty result says how to fix the query: other case, other naming style, skipped files, similar names and paths.

## Scope

- variations of the condition where the count dropped to 0, evaluated on the files before it: `/i`, `/n`, without `/w`, `has` instead of `=`
- skipped files that could match: ignored (with the ignore setting), generated, too big, binary
- similar paths and names
- at most 20% of the timeout, 5 s without timeout
- `hints:` in text, `hints` in JSON

## Spec

- [OUTPUT.md#hints](../OUTPUT.md#hints)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`103__EXAMPLES__FUNNEL_HINTS_EMPTY.md` covers:

- each hint kind of the spec table

## Acceptance criteria

- every example in `103__EXAMPLES__FUNNEL_HINTS_EMPTY.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
