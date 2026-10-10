# Task 35: `get_config` tool

Status: implemented, see [35__SUMMARY__GET_CONFIG.md](35__SUMMARY__GET_CONFIG.md)

Depends on: task 34

## Goal

The agent learns the effective configuration: `get_config` returns it as TOML.

## Scope

- the tool with its description, input schema and annotations of the spec
- the result TOML: the header comments `# easy-peasy-mcp 0.1.0, spec 1, supported specs: 1` and the read-only keys, sections and keys in the order of the spec, values which are not set as `# key =  (not set)` comments
- `section` filters to one section; `sources: true` adds `# default`, `# file: <path>`, `# cli`, `# update` comments
- lists only the keys implemented so far; every key task adds its key
- the server `instructions` get the `get_config` sentence of the spec

## Spec

- [CONFIG.md#get_config](../CONFIG.md#get_config)
- [TOOLS.md#get_config](../TOOLS.md#get_config)
- [TOOLS.md#transport](../TOOLS.md#transport)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches
- keys of later tasks are missing from the result

## Examples

`35__EXAMPLES__GET_CONFIG.md` covers:

- `get_config` with no arguments, with `section`, with `sources`, an invalid `section`

## Acceptance criteria

- every example in `35__EXAMPLES__GET_CONFIG.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
