# Plan for task 28: Proximity `near( ... )`

## Design

- `query/eval.rs`: `Filter::Near { id, order, a, b, distance, ordered, report }` with `Report { All, Closest,
  First }`. For a known content: the lines where `a` / `b` are true (`eval_line`), their matches in the line (the
  contributing text conditions, or the whole line), and the valid pairs `(a, b)`: `|b - a| <= N`; with `ordered`
  `b > a`, or `b == a` and a `B` match starting at or after the end of an `A` match. The file matches if a pair
  exists; unknown / false for unread / absent content like `line( ... )`.
- `report` selects the reported pairs: `all` — every pair; `closest` — per `A` line the pair with the nearest `B`
  line, the earlier on a tie; `first` — the closest pair of the first `A` line. The reported `A` lines get the
  fragments of `a` with the ids of the inner condition and of `near`, the `B` lines those of `b`; a line without
  such a fragment is one whole-line fragment with the id of `near`.
- `query/mod.rs`: the options — `invalid_near_option` (`unknown option 'x'`, hint `options: any, ordered, all,
  closest, first`), `conflicting_options` (`conflicting options 'any' and 'ordered'`, at the second, hint `choose one
  of: any, ordered` / `all, closest, first`); validation scope `Near` for `a` and `b` (`file_target_inside` says
  `near( ... )`); `near` inside `line` is `nested_line` (task 26). `N` is a non-negative integer (the parser).
- `query/subset.rs`: `near( ... )` is supported.
- `description.rs`: `near(...)`.

## Tests

- examples E28.1–E28.14 (fixture `near`)
- unit: pairs for `any` / `ordered`, the same line, `N = 0`, the reports, ties, the option errors

## Steps

1. failing unit tests
2. filter, pairs, reports, fragments, validation, subset
3. description

## Risks

- many `A` and `B` lines: pairs are found with sorted line lists, `O((a + b) log b)` per file
