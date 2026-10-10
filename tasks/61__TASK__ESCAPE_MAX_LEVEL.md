# Task 61: `escape.max_level`

Status: in progress

Depends on: task 30, task 36

## Goal

Choose the depth of `/e(style, any)`.

## Scope

- the key `max_level` in the `[escape]` section of the config file, default `3`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `escape.max_level` replaces it for one request
- the request object `escape` is added to the `find` input schema by this task, with this key only; the other keys of the spec are added by their tasks
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the upper level of `any` and the max level a query may ask for; `invalid_escape_level` names the configured value

## Spec

- [CONFIG.md#escape](../CONFIG.md#escape)
- [QUERY.md#levels](../QUERY.md#levels)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`61__EXAMPLES__ESCAPE_MAX_LEVEL.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `61__EXAMPLES__ESCAPE_MAX_LEVEL.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
