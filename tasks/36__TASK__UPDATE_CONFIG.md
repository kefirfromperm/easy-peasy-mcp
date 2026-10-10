# Task 36: `update_config` tool

Status: not implemented

Depends on: task 35

## Goal

The agent tunes the session: `update_config { "set": "output.context = 2" }` changes the configuration in memory until the server stops.

## Scope

- the tool with its description, input schema and annotations of the spec
- `set`: a TOML fragment like the config file, dotted keys or sections; lists are replaced
- `reset`: keys back to file / command line / default, `["all"]`; reset is applied before set
- atomic: one invalid value changes nothing
- the result: TOML with the changed keys and the old values in comments (`# was ...`, `, reset`)
- errors `invalid_toml`, `read_only`, `unknown_key` (with `did you mean`), `invalid_value` (with the expected values)
- the config file is never written

## Spec

- [CONFIG.md#update_config](../CONFIG.md#update_config)
- [TOOLS.md#update_config](../TOOLS.md#update_config)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`36__EXAMPLES__UPDATE_CONFIG.md` covers:

- set and reset of the keys implemented so far, a session `update_config` → `get_config` → `find`
- each error

## Acceptance criteria

- every example in `36__EXAMPLES__UPDATE_CONFIG.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
