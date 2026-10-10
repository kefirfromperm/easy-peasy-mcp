# Plan for task 11: `size` target

## Design

- `query/eval.rs`: `Bounds { min, max, negated }` — an interval with inclusive or exclusive ends, possibly negated,
  shared with the date targets of tasks 12–14; `Filter::Number { id, order, target, bounds }` with the target `Size`;
  `FileValues.size`.
- `query/value.rs` (new): `parse_size` — `integer [ "." integer ] [ B | K | M | G ]`, units in any case, powers of
  1024, the result in bytes as `f64` (fractions are not rounded); `bounds(op, value)` builds the interval:
  `=` `[v, v]`, `!=` negated, `<` / `<=` / `>` / `>=` open or closed ends, `in a..b` `[a, b]`.
- validation: `invalid_size` (message `invalid size '10MB'`, hint `units: B, K, M, G`) for each value;
  `invalid_range` (message `invalid range '10M..1M'`, the whole range marked, hint `the start must not be greater
  than the end`). `operator_not_allowed` for text operators is in the parser since task 1.
- `query/subset.rs`: `size` with the comparison operators and `in`.
- `Query.needs_metadata`: the search reads `metadata()` only for queries with `size` (or `empty`).
- the example runner: `file <path> <bytes>` in the setup block (done with the examples).
- `description.rs`: `size > 1M`, `size in 10K..1M`.

## Tests

- examples E11.1–E11.13 (fixture `sizes` from the setup)
- unit: `parse_size` (units, lower case, fractions, invalid forms), `Bounds` for every operator, validation errors

## Steps

1. failing unit tests
2. value parsing, bounds, filter, validation, subset, search
3. description

## Risks

None known.
