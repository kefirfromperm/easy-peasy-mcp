# Task 9: `type` target

Status: not implemented

Depends on: task 6

## Goal

Find directories and symlinks too: `type = dir and name = 'migrations'`.

## Scope

- `type` with `=`, `!=`, `in ( ... )` and the values `file`, `dir`, `link`, case-insensitive
- a query with a `type` condition gets no implicit `type = file`
- symlinks are `link` and not followed (`follow_symlinks` is task 46); on Windows also junctions (task 120)
- how directories and links are written in the output
- `unknown_value` for a wrong value, with a `did you mean` hint

## Spec

- [QUERY.md#files-and-directories](../QUERY.md#files-and-directories)
- [QUERY.md#value-operators](../QUERY.md#value-operators)
- [QUERY.md#targets](../QUERY.md#targets)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`9__EXAMPLES__TYPE_TARGET.md` covers:

- `type = dir`, `type = link`, `type in (file, link)`, `type != file`
- a symlink to a file and to a directory, created by the test setup

## Acceptance criteria

- every example in `9__EXAMPLES__TYPE_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
- if the spec doesn't say how a directory or a link is written in the text output, the spec is clarified first
