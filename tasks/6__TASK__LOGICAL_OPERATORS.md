# Task 6: Logical operators `not`, `and`, `xor`, `or` and brackets

Status: not implemented

Depends on: task 5

## Goal

Combine conditions: `name ~ '*.rs' and not path ~ '**/tests/**'`, `name = 'a' or name = 'b'`.

## Scope

- `not`, `and`, `xor`, `or` with the precedence of the spec, brackets, chained `xor` (odd number)
- keywords case-insensitive, free whitespace and line breaks
- condition ids `#1`, `#2`, ... in the order of appearance; the legend lists every condition
- the file conditions `[...]`: which conditions made the file match, by the contribution rules (true `and` — all parts, true `or` / `xor` — only true parts, nothing under `not`)
- the grammar errors `unexpected_token`, `unexpected_end` with the spec hints
- `not` over an unknown value is defined with task 13 (three-valued logic)

## Spec

- [QUERY.md#basics](../QUERY.md#basics)
- [QUERY.md#grammar](../QUERY.md#grammar)
- [OUTPUT.md#file-conditions](../OUTPUT.md#file-conditions)
- [QUERY.md#labels-and-condition-ids](../QUERY.md#labels-and-condition-ids)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`6__EXAMPLES__LOGICAL_OPERATORS.md` covers:

- each operator, precedence without brackets, brackets, `not not`, chained `xor`
- file conditions of `or` branches, of `and`, and nothing listed under `not`
- missing operand, missing `)`, two conditions without an operator

## Acceptance criteria

- every example in `6__EXAMPLES__LOGICAL_OPERATORS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
