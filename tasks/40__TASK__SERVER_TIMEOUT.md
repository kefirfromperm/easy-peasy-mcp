# Task 40: `server.timeout`

Status: not implemented

Depends on: task 39

## Goal

A search never runs longer than allowed; on timeout the partial result comes back.

## Scope

- the key `timeout` in the `[server]` section of the config file, default `"30s"`
- the command line option `--timeout <duration>`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `timeout` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- durations `ms`, `s`, `m`; `"0s"` — no limit; counted from the arrival of the request
- on expiry: partial results, `counts_exact: false`, `truncated: yes (timeout)`, a `timeout` warning, no cursor
- a timeout is not an error

## Spec

- [CONFIG.md#server](../CONFIG.md#server)
- [TOOLS.md#timeout](../TOOLS.md#timeout)
- [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`40__EXAMPLES__SERVER_TIMEOUT.md` covers:

- a timeout example needs a controllable slow search: the test server gets a hook to make reading slow; the example shows the truncated header and the warning

## Acceptance criteria

- every example in `40__EXAMPLES__SERVER_TIMEOUT.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
