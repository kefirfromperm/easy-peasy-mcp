# Summary of task 26: Line scope `line( ... )`

Server version: `0.26.0`.

## What was built

- `line( ... )` (QUERY.md#line-scope): the expression must be true for one line; `not` inside negates per line,
  `not line(...)` means no line matches; several groups are independent; `=`, `has`, `starts`, `ends`, `=~` work on
  the line.
- The group gets an id or a label; the file lists the group, the fragments carry the ids of the group and of the
  condition (OUTPUT.md#fragments); a matched line without a fragment of a contributing text condition is one
  whole-line fragment with the group id; the legend writes the group and its inner conditions.
- `file_target_inside` (`'name' inside line( ... )`), `nested_line` (`nested line( ... )`; `near( ... )` inside a
  group with its own message).
- The `find` description shows `line(...)`.

## Architecture

`Filter::Line { inner }`, `eval_line` and `contributing_in_line` for the inner tree; `search::line_fragments`;
`Node.keyword` keeps the span of the first token for errors about groups.

## Deviations from the plan

None.

## Tests

- examples E26.1–E26.8 (fixture `content`)
- unit: per-line `and` / `or` / `not`, `not line`, anchors on lines, the validation errors

## Limitations and known gaps

- `len` (task 27), `near( ... )` (task 28)

## Try it

`{ "query": "line(text has 'Mutex' and text has 'async')" }`
