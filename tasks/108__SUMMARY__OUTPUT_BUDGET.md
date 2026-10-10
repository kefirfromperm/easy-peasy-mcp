# Summary of task 108: `output.budget`

Server version: `0.108.0`.

## What was built

- `output.budget` (CONFIG.md#output, OUTPUT.md#budget), not set by default, at least 100: from the config file, shown
  by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.budget`; an invalid value is `a whole number of at least 100 is expected` / `must be a whole number of at
  least 100`.
- The response is rendered level by level until its estimate (characters / 4, rounded up; `chars_per_token` comes
  with task 109) is at most the budget: as requested, without context lines, lines cut to 120 characters, `files`
  with as many files as fit (at least 10 or all; a cut list has the next cursor), `facets`, `count`. The budget never
  raises the detail and the `max_*` limits still apply.
- `detail: files (budget 2000)` after the result id and JSON `detail_used` when the detail was lowered;
  `truncated_by` names `budget` whenever a level after the first is used.

## Architecture

- `src/output/budget.rs` (new): `fit` — the levels, the number of files searched in halves.
- `src/output/mod.rs`: `Detail::keyword`, `BudgetUse`, `Response.budget`, `Limits.budget_files`
  (`files_limit`); `src/output/json.rs`: `detail_used`.
- `src/config/keys.rs`: `Type::OptAtLeast`, `Key::Budget`; `src/engine.rs`: the facets (and the metadata) are
  computed in advance with a budget, `CHARS_PER_TOKEN`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–107 show the key and OUTPUT.md got the estimate and how the levels are
  reported (separate commits).

## Tests

- examples E108.1–E108.11 (fixture `huge` of task 104)
- unit: `output::budget::tests::the_first_level_which_fits`, `request::tests::output_limits`

## Limitations and known gaps

- Every level renders the whole response again.

## Try it

`{ "query": "lang = rust and text has 'unwrap(' /c", "output": { "budget": 1500 } }`
