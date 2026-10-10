# Summary of task 11: `size` target

Server version: `0.11.0`.

## What was built

- `size` with `=`, `!=`, `<`, `<=`, `>`, `>=` and `in a..b`, both ends included (QUERY.md#comparison-operators).
- Sizes `integer [ "." integer ] [ B | K | M | G ]`, powers of 1024; units also in lower case and fractions not
  rounded (QUERY.md#sizes, clarified); the size of a directory or a symlink is the size the file system reports.
- `invalid_size` (`invalid size '10MB'`, hint `units: B, K, M, G`), `invalid_range` (`invalid range '10M..1M'`,
  the whole range marked); `operator_not_allowed` for text operators on `size`.
- The `find` description shows `size`.
- The fixture setup creates files of a given size: `file <path> <bytes>` (AGENTS.md).

## Architecture

- `query/value.rs`: `parse_size` and `bounds(op, from, to)`.
- `query/eval.rs`: `Bounds` (interval with inclusive / exclusive ends, negated for `!=`) and `Filter::Number` with
  the target `Size`; `FileValues.size`.
- `Query.needs_metadata`: the search reads `metadata()` only for queries with `size` or `empty`.

## Deviations from the plan

None.

## Tests

- examples E11.1–E11.13 (fixture `sizes`, created by the setup)
- unit: `parse_size` (units, lower case, fractions, invalid forms), `Bounds` per operator, size filters, validation
  errors

## Limitations and known gaps

None new.

## Try it

`{ "query": "size > 1M" }`
