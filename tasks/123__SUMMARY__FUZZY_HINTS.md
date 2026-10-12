# Summary of task 123: hints for an empty fuzzy result

Server version: `0.122.0`.

## What was built

When `name ~~` or `path ~~` finds nothing, the hints of an empty result ([OUTPUT.md#hints](../OUTPUT.md#hints)) now
say how to fix the query in one step:

- **`path ~~` instead of `name ~~`** — a new variation, evaluated and reported like the others (with the steps before
  it, the number of files, at most three paths): `path ~~ 'qryparser'  1 file: src/query/parser.rs`
- **below the fuzzy threshold** — the files which pass the steps before and which the condition matches with a score
  below `query.fuzzy_min_score`, best first (equal scores in path order), at most three, the names (`name`) or paths
  (`path`) with the scores rounded down to two decimals:
  `name ~~ 'rsr': below query_options.fuzzy_min_score 0.6: parse.rs 0.55, parser.rs 0.54`
- the hints are in a stated order: the variations and the fuzzy scores of each step, `skipped:`, the similar names
  and directories; in JSON they are lines of `hints` as before

The spec change came first: OUTPUT.md#hints (two rows of the table, the variation, the new hint, the order) and a
sentence in QUERY.md#--fuzzy. The `find` tool description doesn't list the hints, so it is unchanged.

## Architecture

- `src/query/mod.rs`: `FunnelStep::variations` is a `Vec<Variation>` (`text`, `target`): a variation knows what its
  hint shows of the files, since `path ~~` shows paths where its `name ~~` step would show names. The unused
  `FunnelStep::target` is gone. `FunnelStep::fuzzy` is the target of a step which is one `~~` condition.
- `src/hints.rs`: `below()` searches the condition alone, parsed with `fuzzy_min_score = 0`, with `rank` on, so the
  `fuzzy` relevance signal of every file is the score of this condition; with steps before, their files are searched
  too and intersected by path and member. The pure `below_hint()` sorts, cuts and formats. Both searches run within
  the deadline of the hints.

## Deviations from the plan

None.

## Tests

- examples E123.1–E123.8 in `tasks/123__EXAMPLES__FUZZY_HINTS.md`, fixture `tests/fixtures/fuzzy-hints/`: the `path
  ~~` variation, below the threshold for `name` and `path`, at most three with equal scores, the steps before, no
  hint, `or` branches, JSON
- unit tests: `the_fuzzy_steps` and the updated `the_variations_of_a_step` (`src/query/mod.rs`),
  `below_the_threshold_best_first`, `scores_rounded_down`, `nothing_below` (`src/hints.rs`)
- the examples of all earlier tasks pass unchanged; no earlier example had an empty `~~` result with hints

## Limitations

- `~~` is not allowed inside `line( ... )` / `near( ... )`, so there are no hints there
- a `~~` step inside brackets or a group with other conditions gets no fuzzy hints, like the other variations

## How to try it

```json
{ "query": "name ~~ 'qryparser'" }
```

on this repository answers with the hint `path ~~ 'qryparser'  1 file: src/query/parser.rs`.
