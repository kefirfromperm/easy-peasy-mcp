# Plan for task 7: Condition labels `@name`

## Design

- The lexer and parser already read `@label` after a condition, `line( ... )` and `near( ... )`, and the ids and the
  legend already use labels (task 1); the legend leaves labels out.
- `query/subset.rs`: labels are no longer `not_implemented`.
- `query/mod.rs` validation: `duplicate_label`, message `duplicate label '@x'`, at the second use of the label, hint
  `labels must be unique`.
- `query/parser.rs`: an `@` which doesn't start a label (`@1x`) is `unexpected_token` with the hint
  `a label starts with a letter or '_', e.g. @name`.
- `description.rs`: `Labels: name ~ '*.rs' @rust — the output shows which condition matched.`

## Tests

- examples E7.1–E7.6
- unit: duplicate labels (also inside groups), the `@` hint, ids of mixed labelled and numbered conditions

## Steps

1. failing unit tests
2. validation, parser hint, subset
3. description

## Risks

None known.
