# Task 115: Cancellation

Status: not implemented

Depends on: task 114

## Goal

The client stops a running search with `notifications/cancelled`.

## Scope

- the walk and all search threads stop as soon as possible
- no response for the cancelled request
- unknown or finished requests are ignored

## Spec

- [TOOLS.md#cancellation](../TOOLS.md#cancellation)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`115__EXAMPLES__CANCELLATION.md` covers:

- end-to-end over stdio: a slow search cancelled, then a normal request still answered

## Acceptance criteria

- every example in `115__EXAMPLES__CANCELLATION.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
