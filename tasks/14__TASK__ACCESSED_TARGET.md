# Task 14: `accessed` target

Status: in progress

Depends on: task 13

## Goal

Find files by last access date: `accessed < -30d`.

## Scope

- `accessed` with the same operators and values as `modified`
- unknown and `date_unsupported` as for `created`; reading files during the search must not be what makes them match (atime is read before the content)

## Spec

- [QUERY.md#targets](../QUERY.md#targets)
- [CONFIG.md#platforms](../CONFIG.md#platforms)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`14__EXAMPLES__ACCESSED_TARGET.md` covers:

- atimes set by the test setup

## Acceptance criteria

- every example in `14__EXAMPLES__ACCESSED_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
