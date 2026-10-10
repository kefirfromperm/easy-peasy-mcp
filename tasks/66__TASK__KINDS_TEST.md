# Task 66: `kinds.test`

Status: implemented, see [66__SUMMARY__KINDS_TEST.md](66__SUMMARY__KINDS_TEST.md)

Depends on: task 63

## Goal

Change which paths are tests.

## Scope

- the key `test` in the `[kinds]` section of the config file, default the list of the full example in CONFIG.md
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `kinds.test` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- a configured list replaces the built-in one

## Spec

- [CONFIG.md#kinds](../CONFIG.md#kinds)
- [QUERY.md#kind](../QUERY.md#kind)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`66__EXAMPLES__KINDS_TEST.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `66__EXAMPLES__KINDS_TEST.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
