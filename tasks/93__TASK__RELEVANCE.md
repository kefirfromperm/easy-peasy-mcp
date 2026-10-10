# Task 93: Relevance ranking

Status: in progress

Depends on: task 92, task 91, task 32

## Goal

`sort = relevance` (the default) puts the most likely files first.

## Scope

- the score 0–1 from the signals of the spec: fuzzy score, query words in the path, match density, matched conditions, kind, recency relative to the newest matched file, depth
- deterministic; equal scores ordered by the full path
- `score` with `meta`
- weights are internal and documented in the summary

## Spec

- [OUTPUT.md#relevance](../OUTPUT.md#relevance)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`93__EXAMPLES__RELEVANCE.md` covers:

- orders which follow from one dominant signal, so tuning the weights doesn't change them
- `meta` with scores rounded as the spec shows

## Acceptance criteria

- every example in `93__EXAMPLES__RELEVANCE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
