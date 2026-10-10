# Task 37: `--log-level`

Status: in progress

Depends on: task 34

## Goal

Logs for troubleshooting, without disturbing the protocol.

## Scope

- `--log-level error|warn|info|debug`, logs to stderr only, stdout stays protocol-only
- an invalid level: a usage error at start

## Spec

- [CONFIG.md#command-line](../CONFIG.md#command-line)
- [TOOLS.md#transport](../TOOLS.md#transport)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`37__EXAMPLES__LOG_LEVEL.md` covers:

- the level from the command line; an end-to-end test checks stdout has only JSON-RPC messages at `debug`

## Acceptance criteria

- every example in `37__EXAMPLES__LOG_LEVEL.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
