# Plan for task 123: hints for an empty fuzzy result

## Design

Two new hints of an empty result (OUTPUT.md#hints), both on the step of a `name ~~` / `path ~~` condition.

### `path ~~` instead of `name ~~` — `src/query/mod.rs`

- `variations()` adds `path ~~ '...'` for a `name ~~` condition: the target token rewritten with `rewritten()`
- a variation knows its own target, since the examples of `path ~~` are paths, not names: `FunnelStep::variations`
  becomes `Vec<Variation>`, `Variation { text, target }`; `hints::variation_hint` takes the target of the variation
  instead of the target of the step
- evaluated and reported by the existing code: with the steps before it, the number of files, at most three paths

### Below the fuzzy threshold — `src/hints.rs`

- `FunnelStep::fuzzy: Option<Target>`: the target of a step which is one `~~` condition
- `below(cx, before, step)`: parses the condition alone with `fuzzy_min_score = 0` and searches it with `rank`, so
  every found file has `signals.fuzzy`, the score of this one condition; with steps before, also searches them and
  keeps the files found by both (by path and member); keeps the scores below `cx.parse.fuzzy_min_score`
- a pure `below_hint(condition, threshold, Vec<(shown, score)>) -> Option<Hint>`: path order kept for equal scores
  (stable sort, best first), at most three, scores rounded down to two decimals, the threshold with `{}` (`1` for
  `1.0`, as `get_config`); `None` without files
- `collect()` runs it right after the variations of each step, so the order is the one of the spec: variations and
  fuzzy scores of each step, `skipped:`, similar names and directories

The searches run within the deadline of the hints like the others (`OutOfTime`). No new modules or dependencies.

## Rejected

- scoring with `Fuzzy::score` on the name or path directly: the hint would have to rebuild the haystack the search
  uses (working directory prefix, NFC, members of archives); `rank` gives the same score the search compares
- one search of `<steps before> and <condition>`: the `fuzzy` signal is the best of all `~~` conditions of the
  query, so a `~~` step before would mix in its score
- rounding the score as `meta` does: `0.497` would be shown as `0.50`, not below `0.5`

## Tests

- examples E123.1–E123.8 (`tests/examples.rs`)
- unit tests in `src/query/mod.rs`: `name ~~` has the `path ~~` variation with target `path`, `path ~~` has none;
  `fuzzy` is set for a `~~` step only
- unit tests in `src/hints.rs`: `below_hint` orders best first and keeps the path order of equal scores, cuts at
  three, rounds down, writes `1` for a threshold of `1.0`, gives nothing without files
- the examples of every earlier task, in particular 103 (hints) and 62 (`fuzzy_min_score`), still pass

## Steps

1. unit tests, failing (with the examples of the previous commit)
2. the `path ~~` variation with its own target
3. the hint below the threshold
4. summary, status `implemented`, minor version bump `0.121.1` → `0.122.0`

## Risks

- an earlier example with an empty `~~` result gains hints: then it is checked against the spec and fixed in its
  examples file, with the reason in the commit message
- two more searches per `~~` step cost time; they are metadata searches and stop at the deadline of the hints
