# Summary of task 6: Logical operators `not`, `and`, `xor`, `or` and brackets

Server version: `0.6.0`.

## What was built

- `not`, `and`, `xor`, `or` with the precedence of the spec and brackets (QUERY.md#basics); keywords in any case,
  free whitespace and line breaks; chained `xor` is true for an odd number of true parts.
- Condition ids `#1`, `#2`, ... in the order of appearance; the legend lists every condition, also those under `not`.
- The condition list of a file (OUTPUT.md#file-conditions): a true `and` — all parts, a true `or` / `xor` — only
  the true parts, nothing under `not`; a file matched only through `not` has no list (spec clarified).
- Grammar errors for a missing operand, a missing `)`, two conditions without an operator, `not` at the end.
- The `find` description shows the logical operators.

## Architecture

- `query/eval.rs`: `Filter` is a tree `And` / `Or` / `Xor` / `Not` / `Text` (brackets disappear) with `eval` and
  `contributors`; `Text` keeps its order of appearance to sort the ids. `FileValues` holds the name and path.
- `query/subset.rs` walks the syntax tree and reports the first unsupported node by position: targets, operators,
  modifiers, labels, `line( ... )`, `near( ... )`, comments.

## Deviations from the plan

None.

## Tests

- examples E6.1–E6.12 (fixture `names`); the files of an `or` / `xor` example have as many contributing
  conditions as each other, or the order by contributions is the path order too
- unit: evaluation and contributions (`and`, `or`, chained `xor`, `not`, `not not`, nested branches), the subset
  check of nested expressions

## Limitations and known gaps

- three-valued logic comes with the first unknown value (tasks 13 and 20)
- the same output gaps as before: no funnel, no truncation, order by path

## Try it

`{ "query": "name ~ '*.rs' and not path ~ '**/tests/**'" }`
