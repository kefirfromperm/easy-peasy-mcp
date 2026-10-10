# Task 7: Condition labels `@name`

Status: not implemented

Depends on: task 6

## Goal

Name conditions so the output says which one matched: `name ~ '*.rs' @rust or name ~ '*.go' @go`.

## Scope

- `@label` after a condition (after its modifiers); the label syntax of the grammar
- a labelled condition gets the label as its id, unlabelled conditions are numbered without gaps
- the legend lists labels as ids and the conditions without labels
- `duplicate_label` error

## Spec

- [QUERY.md#labels-and-condition-ids](../QUERY.md#labels-and-condition-ids)
- [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`7__EXAMPLES__LABELS.md` covers:

- labels with `or`, mixed labelled and numbered conditions
- a duplicate label, an invalid label

## Acceptance criteria

- every example in `7__EXAMPLES__LABELS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
