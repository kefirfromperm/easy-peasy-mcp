# Summary of task 113: batch `queries`

Server version: `0.113.0`.

## What was built

- `queries` (OUTPUT.md#batch): several queries in one call. Every item has a string `query`, an optional `label` and
  may override `dirs` and `timeout` and add to `traversal` and `output` (merged key by key); the other request fields
  are shared. `query` with `queries` is `invalid_option`; a `queries` which is not a non-empty list of objects too.
- Every query has its section with its own result id, legend and funnel: in the text format `== label ==  ` before the
  first line of its response (its number without a label); in JSON `{ "version": 1, "queries": [...] }`, every item
  its response without `version` after its `label`, or its `error`.
- A wrong item or an invalid query is an error in its section, the others still run; a wrong shared field fails the
  request; `isError` only when all queries fail.
- The budget and `max_output` of the whole response are split: an item with its own value keeps it, the rest is
  divided equally among the others in their order, and what a query doesn't use goes to the next ones.

## Architecture

- `src/batch.rs` (new): `items`, `shared`, `label`, `item_arguments`, `Split`, `response`.
- `src/engine.rs`: `find` hands a request with `queries` to `batch`, which runs every item through `try_find`.
- `src/description.rs`: `queries` in the input schema.

## Deviations from the plan

- OUTPUT.md got the sections, the items and the JSON form of a batch (separate commit).

## Tests

- examples E113.1–E113.7 (fixture `funnel` of task 102)
- unit: `batch::tests::the_arguments_of_an_item`, `batch::tests::the_budget_is_split`

## Limitations and known gaps

- The queries run one after another and every query walks its directories on its own: the split of the budget needs
  what the earlier queries used; sharing one walk between queries with the same `dirs` and `traversal` is not done.

## Try it

`{ "queries": [{ "label": "defs", "query": "text =~ `fn\\s+load_config`" }, { "label": "calls", "query": "text has
'load_config('" }], "output": { "detail": "files" } }`
