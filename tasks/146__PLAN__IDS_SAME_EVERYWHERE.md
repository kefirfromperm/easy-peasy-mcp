# Plan for task 146: The ids which are the same everywhere are left out

## Design

- `output/omit.rs`: a `Differ { conditions, match_ids }` of the result, made by `Differ::of(&[Found])`: whether two
  files have different sets of conditions, whether two fragments (content and path fragments) have different sets of
  ids. `Omit::effective` takes it, ANDed with `many_conditions` by the caller, and `matches_shown` (`lines` detail, or
  path fragments in `files` detail) instead of the bare flag:
  - `conditions` left out unless `differ.conditions`
  - `match_ids` left out unless `differ.match_ids` (with `output.groups`: unless more than one reference, as before)
  - `legend` left out unless `conditions` stays or `match_ids` stays and `matches_shown`
- `engine.rs` (`respond`): `Differ::of(&files)` on all files of the result, before the page, `since` and the limits
  cut them; `many_conditions` still gates both

Rejected: deciding over the listed files only — the pages of a cursor would disagree; comparing escape levels —
the rule of one counted condition already drops them.

## Tests

- unit: `omit` — `Differ::of` over files with equal and different conditions and fragment ids (an id set in another
  order is the same), the rules of `effective` with the new arguments
- examples: E146.1–E146.7; earlier spec 2 examples which change (E143.2, E143.5, maybe others) are fixed with the
  reason in their commit

## Steps

1. red: the unit tests; the example tests already fail on E146.*
2. green: `Differ`, `effective`, the engine
3. fix the earlier spec 2 examples the rule changes
4. summary, status `implemented`, version 0.143.0

## Risks

- `Found` of a removed file of `since` has no conditions: `Differ` runs before `shown_files`, on the new result only
- the budget can lower `lines` to `files` after the decision: the legend may stay without matches, as the spec allows
