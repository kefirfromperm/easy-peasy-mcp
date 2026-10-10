# Summary of task 7: Condition labels `@name`

Server version: `0.7.0`.

## What was built

- `@label` after a condition and its modifiers (QUERY.md#labels-and-condition-ids): the label is the condition id in
  the legend and in the condition lists; only conditions without a label are numbered, without gaps.
- `duplicate_label` at the second use of a label, hint `labels must be unique`.
- `@` which doesn't start a label is `unexpected_token` with the hint `a label starts with a letter or '_', e.g.
  @name`.
- The `find` description shows labels.

## Architecture

The ids, the legend and the filter used labels since task 1; this task removes labels from the subset check and adds
`duplicate_labels` to the validation.

## Deviations from the plan

None.

## Tests

- examples E7.1–E7.6 (fixture `names`)
- unit: duplicate labels inside groups, the `@` hint, labelled and numbered ids

## Limitations and known gaps

- labels of `line( ... )` and `near( ... )` come with tasks 26 and 28

## Try it

`{ "query": "name ~ '*.rs' @rust or name ~ '*.go' @go" }`
