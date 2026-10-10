# Task 13: `created` target

Status: in progress

Depends on: task 12

## Goal

Find files by creation date where the file system provides it: `created > -1d`.

## Scope

- `created` with the same operators and values as `modified`
- three-valued logic: a condition can be unknown; `not`, `and`, `or`, `xor` follow the table of the spec, a file is returned only if the whole query is true; content conditions use it from task 20
- without a creation date the condition is unknown and the response has a `date_unsupported` warning (the first warning: the `warning:` lines at the end of the text output)

## Spec

- [QUERY.md#targets](../QUERY.md#targets)
- [QUERY.md#files-whose-content-is-not-searched](../QUERY.md#files-whose-content-is-not-searched)
- [OUTPUT.md#warnings](../OUTPUT.md#warnings)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`13__EXAMPLES__CREATED_TARGET.md` covers:

- creation dates can't be set portably: examples use conditions which are true for any creation date of the fixture (e.g. `created > 2000-01-01`); the unknown case is a unit test

## Acceptance criteria

- every example in `13__EXAMPLES__CREATED_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
