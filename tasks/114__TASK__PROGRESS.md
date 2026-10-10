# Task 114: Progress notifications

Status: implemented, see [114__SUMMARY__PROGRESS.md](114__SUMMARY__PROGRESS.md)

Depends on: task 40

## Goal

Long searches report progress to the client.

## Scope

- with a `progressToken` in `_meta`: `notifications/progress` about once a second, `progress` = scanned files, growing, no `total`, a message `scanned N files, M matched, Ts`
- no notifications for searches shorter than one second

## Spec

- [TOOLS.md#progress](../TOOLS.md#progress)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`114__EXAMPLES__PROGRESS.md` covers:

- end-to-end over stdio with the slow-read hook of task 40; JSON-RPC exchanges are the examples

## Acceptance criteria

- every example in `114__EXAMPLES__PROGRESS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
