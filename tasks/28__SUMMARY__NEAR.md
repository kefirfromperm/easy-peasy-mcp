# Summary of task 28: Proximity `near( ... )`

Server version: `0.28.0`.

## What was built

- `near(A, B, N [, order] [, report])` (QUERY.md#proximity): A and B are line scope expressions, `N = 0` is the same
  line; options in any order and case.
- `ordered`: the B line after the A line, on the same line a B match starting at or after the end of an A match
  (spec clarified); `any` by default.
- `report`: `all` (default), `closest` (the nearest B per A line, the earlier on a tie), `first` (the first A line
  with its nearest B line, spec clarified); it changes only the reported lines, fragments and counts.
- Fragments carry the inner condition id and the `near` id; a reported line without a text fragment is one
  whole-line fragment; the file lists `near`; a label after `near( ... )` works.
- Errors: `invalid_near_option`, `conflicting_options`; `near` inside `line` and groups inside `near` are
  `nested_line`; `file_target_inside` names `near( ... )`.
- The `find` description shows `near`.

## Architecture

`Filter::Near` with `Report`; `near_pairs` finds the A and B lines, the pairs (sorted B lines, binary search) and
filters them by the report; `match_spans` gives the matches of a line expression for the same-line order;
`search::line_fragments` serves `line( ... )` and both sides of `near`. The validation has a `Scope`
(`File`, `Line`, `Near`).

## Deviations from the plan

None.

## Tests

- examples E28.1–E28.14 (fixture `near` with the spec examples)
- unit: any / ordered, the same line, `N = 0`, `len` inside `near`, the option errors, file targets inside

## Limitations and known gaps

None new.

## Try it

`{ "query": "near(text has '.lock()', text has '.await', 3)" }`
