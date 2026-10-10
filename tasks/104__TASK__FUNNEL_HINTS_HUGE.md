# Task 104: Hints for huge results

Status: implemented, see [104__SUMMARY__FUNNEL_HINTS_HUGE.md](104__SUMMARY__FUNNEL_HINTS_HUGE.md)

Depends on: task 103, task 15

## Goal

A huge result says where the matches are and how to narrow the query.

## Scope

- the top values of the `dir` and `lang` facets; the counting is shared with the facets of task 105
- `narrow with:` suggestions

## Spec

- [OUTPUT.md#hints](../OUTPUT.md#hints)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`104__EXAMPLES__FUNNEL_HINTS_HUGE.md` covers:

- a generated fixture with more than 10 × `max_files` matches (with a small `max_files`)

## Acceptance criteria

- every example in `104__EXAMPLES__FUNNEL_HINTS_HUGE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
