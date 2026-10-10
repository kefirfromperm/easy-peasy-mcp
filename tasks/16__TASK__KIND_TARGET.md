# Task 16: `kind` target

Status: in progress

Depends on: task 15

## Goal

Find files by purpose: `kind = test`, `kind in (source, test)`, `kind = generated`.

## Scope

- kinds `generated`, `test`, `doc`, `config`, `source`, `other`, the first matching in this order, by the built-in rules; `source` — a language with `source = true`
- a query with a `kind` condition skips nothing implicitly: `kind = generated` returns generated files
- `=`, `!=`, `in ( ... )`, `unknown_value`

## Spec

- [QUERY.md#kind](../QUERY.md#kind)
- [QUERY.md#generated-files](../QUERY.md#generated-files)
- [CONFIG.md#kinds](../CONFIG.md#kinds)

## Out of scope

- changing the rules: tasks 63–68

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`16__EXAMPLES__KIND_TARGET.md` covers:

- one file of every kind, a generated file found only with a `kind` condition
- a file matching several rules gets the first kind

## Acceptance criteria

- every example in `16__EXAMPLES__KIND_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
