# Task 4: `path` target

Status: implemented, see [4__SUMMARY__PATH_TARGET.md](4__SUMMARY__PATH_TARGET.md)

Depends on: task 3

## Goal

Find files by their path relative to the working directory: `path ~ 'src/**/*.rs'`, `path starts 'docs/'`.

## Scope

- `path` with `=`, `has`, `starts`, `ends`, `~`
- the path is relative to the working directory, without it, `/` as the separator in queries
- wildcards on paths: `*` and `?` don't cross `/`, `**` crosses directories, `**/x/**` also matches the top level directory `x`
- NFC comparison and platform case sensitivity as for `name`

## Spec

- [QUERY.md#targets](../QUERY.md#targets)
- [QUERY.md#wildcards](../QUERY.md#wildcards)

## Out of scope

- `\` as a separator in queries on Windows — task 120

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`4__EXAMPLES__PATH_TARGET.md` covers:

- `path ~` with `*`, `**`, `?`, `**/tests/**`; `path =`, `has`, `starts`, `ends`
- files at several depths, grouped under directory headers

## Acceptance criteria

- every example in `4__EXAMPLES__PATH_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
