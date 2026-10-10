# Task 116: Concurrent requests

Status: in progress

Depends on: task 41, task 36

## Goal

Several `find` calls run at the same time without interfering.

## Scope

- parallel `find` calls share the thread pool of `server.threads`
- each `find` takes a configuration snapshot at start; `update_config` during a search doesn't affect it
- `update_config` calls are applied one after another
- result ids stay unique

## Spec

- [TOOLS.md#concurrency](../TOOLS.md#concurrency)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`116__EXAMPLES__CONCURRENCY.md` covers:

- end-to-end: two overlapping searches and an `update_config` between them

## Acceptance criteria

- every example in `116__EXAMPLES__CONCURRENCY.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
