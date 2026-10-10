# Task 42: `server.allow_update`

Status: implemented, see [42__SUMMARY__SERVER_ALLOW_UPDATE.md](42__SUMMARY__SERVER_ALLOW_UPDATE.md)

Depends on: task 36

## Goal

An operator can forbid the agent to change the configuration.

## Scope

- the key `allow_update` in the `[server]` section of the config file, default `true`
- the command line option `--no-update`
- read-only: shown by `get_config`, rejected by `update_config` with `read_only` and the spec hint
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- with `false`, `update_config` is not listed in `tools/list`; a call made anyway fails with `update_disabled`

## Spec

- [CONFIG.md#server](../CONFIG.md#server)
- [TOOLS.md#tools](../TOOLS.md#tools)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`42__EXAMPLES__SERVER_ALLOW_UPDATE.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `42__EXAMPLES__SERVER_ALLOW_UPDATE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
