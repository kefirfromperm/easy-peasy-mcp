# Task 11: `size` target

Status: in progress

Depends on: task 6

## Goal

Find files by size: `size > 1M`, `size in 10K..1M`, `size = 0`.

## Scope

- comparison operators `=`, `!=`, `<`, `<=`, `>`, `>=` and `in a..b` (both ends included)
- size values: bytes with optional `B`, `K`, `M`, `G` (powers of 1024), fractions like `1.5M`
- errors `invalid_size`, `invalid_range`, `operator_not_allowed` for text operators on `size`

## Spec

- [QUERY.md#comparison-operators](../QUERY.md#comparison-operators)
- [QUERY.md#sizes](../QUERY.md#sizes)
- [QUERY.md#errors](../QUERY.md#errors)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`11__EXAMPLES__SIZE_TARGET.md` covers:

- every operator, every unit, a fraction, a range
- `size > 10MB`, `size in 10M..1M`

## Acceptance criteria

- every example in `11__EXAMPLES__SIZE_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
