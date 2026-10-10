# Plan for task 26: Line scope `line( ... )`

## Design

- `query/eval.rs`: `Filter::Line { id, order, inner }`. The inner expression is a filter tree of line scope
  conditions, evaluated per line by `eval_line(line)` (two-valued: the line is known); a `Content` leaf there runs its
  regex on the line, so `=` / `starts` / `ends` (anchored with `\A` / `\z`) and `has` work on the line, `=~` too.
  The group is true if one line makes the inner expression true; unknown while the content is not read or not
  searched; false without content.
- the contributions inside a line: all parts of a true `and`, true parts of `or` / `xor`, nothing under `not`.
- fragments of a group: for every line which matches it, the matches of the contributing text conditions in that
  line with the ids of the condition and of the group (in the order of appearance); a matched line without such a
  fragment is one whole-line fragment with the id of the group. The file lists the group, not the inner conditions.
- validation: inside a group only `text` (and `len`, task 27): other targets are `file_target_inside`
  (`'name' inside line( ... )`, hint `move 'name' out of line( ... )`); a group inside a group is `nested_line`
  (`nested line( ... )`, hint `line( ... ) can't be nested`), `near( ... )` inside a group too
  (`near( ... ) inside line( ... )`, hint `line( ... ) can't contain near( ... )`).
- `query/subset.rs`: `line( ... )` is supported.
- `description.rs`: `line(...)`.

## Tests

- examples E26.1–E26.8 (fixture `content`)
- unit: per-line evaluation of `and` / `or` / `not` inside, anchors on the line, `not line(...)`, contributions and
  whole-line fragments, the validation errors

## Steps

1. failing unit tests
2. filter, evaluation, fragments, validation, subset
3. description

## Risks

None known.
