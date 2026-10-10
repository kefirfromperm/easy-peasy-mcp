# Summary of task 5: `/i` modifier on `name` and `path`

Server version: `0.5.0`.

## What was built

- Modifiers after a string value (QUERY.md#modifiers): `/x` in any case, several in any order, with or without
  spaces; the legend keeps them as written.
- `/i` on `name` and `path` with `=`, `has`, `starts`, `ends`, `~`: case-insensitive with Unicode simple case
  folding (`Ä` = `ä`) on every platform.
- `modifier_not_allowed` for every modifier of every condition, with the scope (inside `line( ... )` /
  `near( ... )` or not): the message `modifier '/x' is not allowed here`, the position of the modifier and the hint
  of its rule (see the plan). `/w`, `/m`, `/c`, `/e`, `/n` where allowed are `not_implemented` until their tasks.
- An unknown modifier is `unexpected_token` with the list of modifiers.
- The `find` description shows `/i`.

## Architecture

`query::validate` walks the tree with the scope and checks wildcards and modifiers for all conditions; errors are
ordered by position, and on the same position a real error wins over `not_implemented`. The compiled matcher is
case-insensitive with `/i` or an insensitive `path_case`.

## Deviations from the plan

None.

## Tests

- examples E5.1–E5.7 (fixture `names`)
- unit: `/i` matchers, `modifier_not_allowed` for each modifier (with the validation alone where the target is not
  implemented yet), allowed modifiers reach the subset check, the legend with modifiers, the subset accepts `/i`

## Limitations and known gaps

- the same as task 2: no funnel for empty results, no truncation, order by path

## Try it

`{ "query": "name = 'readme.md' /i" }`
