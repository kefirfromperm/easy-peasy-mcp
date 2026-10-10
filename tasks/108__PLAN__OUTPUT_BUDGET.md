# Plan for task 108: `output.budget`

## Design

- `src/config/keys.rs`: `Type::OptAtLeast(min)` — a whole number of at least `min` or not set; `Key::Budget` after
  `Detail`, `Type::OptAtLeast(100)`, not set by default.
- `src/output/budget.rs` (new): `fit(response, budget, chars_per_token, json)` renders the response level by level
  until the estimate (characters / `chars_per_token`, rounded up) is at most the budget: as requested, without
  context, lines cut to 120, `files` with as many files as fit (at least 10 or all, a cursor when cut), `facets`,
  `count`; never above the requested detail (OUTPUT.md#budget, clarified in its own commit).
- `src/output/mod.rs`, `src/output/json.rs`: `Response.budget` — the budget and whether the detail was lowered:
  `detail: files (budget N)` after the result id, JSON `detail_used`; `truncated_by` names `budget`; the file list
  cut by the budget (`Limits.budget_files`) gives the next cursor.
- `src/engine.rs`: with a budget the facets and the metadata they need are computed in advance; `chars_per_token`
  is 4 until task 109; `src/description.rs`.

## Tests

- examples E108.1–E108.11 (fixture `huge` of task 104)
- unit: the levels (context, cut lines, the most files which fit, count), the type, the request field

## Steps

1. failing tests
2. the type and key, the levels, the output

## Risks

- every level renders the whole response again; there are at most a few of them (the file count is searched in
  halves)
