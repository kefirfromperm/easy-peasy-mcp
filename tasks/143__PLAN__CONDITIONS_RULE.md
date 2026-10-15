# Plan for task 143: The legend rule counts the conditions an agent wrote

## Design

- `query/ast.rs`: `Expr::counted()` counts the conditions as OUTPUT.md#counted-conditions says — a condition one,
  `line( ... )` / `near( ... )` / a labelled bracket what is inside, at least one, `not` nothing, `and` / `or` /
  `xor` / brackets the sum; `Expr::has_label()` says whether the agent wrote a label anywhere
- `query/mod.rs`: `Query::many_conditions: bool`, set by `parse`: more than one counted condition or a label; `false`
  for the empty query
- `output/omit.rs`: `Omit::effective` takes this flag instead of the number of the legend entries; `match_ids` still
  follows the number of the references of `output.groups` when it is set
- `engine.rs`: passes `query.many_conditions`

Rejected: counting only the conditions which can tell files or matches apart (`or` branches, several text
conditions): exact, but an agent can't predict it from the query it wrote.

## Tests

- unit: `query` — the count and the flag of the queries of the spec section; `omit` — the rules with the flag
- examples: E143.1–E143.8; the examples of every earlier task stay green

## Steps

1. red: the unit tests, the example tests run from the examples file
2. green: `counted`, `has_label`, the flag, `effective`
3. summary, status `implemented`, version 0.140.0

## Risks

- earlier spec 2 examples with one labelled condition would change: none found (130, 135, 139, 141, 142 run against
  spec 2; the test run shows any)
