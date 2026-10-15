# Summary of task 146: The ids which are the same everywhere are left out

Server version: `0.143.0`.

## What was built

- The spec 2 rule of the legend, the conditions of the files and the ids of the matches
  ([OUTPUT.md#the-same-everywhere](../OUTPUT.md#the-same-everywhere)) now also asks whether they tell anything apart:
  - `conditions` are written when two files of the result have different conditions
  - `match_ids` when two matches (content and path fragments) have different sets of ids; escape levels are not
    compared; with `output.groups` the ids still follow the number of its references
  - the `legend` when the conditions differ, or the ids of the matches differ (with `groups`: are written) and the
    response shows matches (`lines` detail, path fragments in `files` detail); in `facets` detail when the conditions
    differ. A part brought back by `show` doesn't bring the legend
  - the decision is made over all files of the result, so the pages of a cursor, `max_files` and the budget don't
    change it; with `since` over the new result
- `name ~ '*.rs' and text has 'x'` answers without a legend, `[#1 #2]` and `⟦#2:`; `name = 'a' or text has 'x'`
  keeps the conditions of the files and drops the ids of the matches; `line(text has 'a' or text has 'b')` keeps the
  ids of the matches and drops `[#1]`. The counted-conditions rule of task 143 still applies first. Spec 1 is unchanged.
- Spec: two commits on the branch — the rule and the subsection `The same everywhere`, then the legend following
  what differs rather than `show`. A change of a spec 2 rule, allowed since spec 2 has not been used outside this
  repository (TOOLS.md#versioning).

## Architecture

- `output::omit::Differ { conditions, match_ids }`: `Differ::of(&[Found])` compares the condition sets of the files
  and the id sets of all fragments (`HashSet<&CondId>`, so the order doesn't matter); `Differ::and(many)` applies the
  counted-conditions rule.
- `Omit::effective` takes the `Differ` and `matches_shown` instead of the `many_conditions` flag.
- `engine.rs` (`respond`) computes `Differ::of(&files)` on the whole result, before the `since` differences and the
  limits, and `matches_shown` from the detail and `path_fragments`.

## Deviations from the task and the plan

- The plan said the legend is written when one of the two parts "is written". The unit tests showed two cases where
  this read wrong: `show = ["conditions"]` would bring the legend along (the agent asked for one part), and with
  `groups` of two references and one counted condition the legend would come back (task 143 left it out). The legend
  now follows what differs, not `omit`/`show`; the spec says so (second spec commit).

## Tests

- examples E146.1–E146.7 (fixture `omit` of task 134): `and` in text and JSON and with a line group, `or` with
  different files, a line group with different matches in `lines` and `files` detail, a cursor whose first page alone
  has the same ids, the `condition` facet, `show` of each part, spec 1
- unit `output::omit::tests::what_differs_in_a_result`, `the_parts_which_tell_nothing_apart`; the earlier `omit`
  tests adapted to the new arguments
- earlier spec 2 examples changed by the rule, fixed in their own commit: E130.6, E130.7, E135.7, E141.10, E143.2,
  E143.5, E144.4

## Limitations and known gaps

- when the budget lowers `lines` to `files`, the decision made for `lines` stays: a legend can be written without
  any match, as the spec allows.

## Try it

`{"query": "name ~ '*__TASK__*' and line(text = 'Status: not implemented')", "dirs": ["tasks"]}` — the files without
`[#1 #2]`, no legend.
