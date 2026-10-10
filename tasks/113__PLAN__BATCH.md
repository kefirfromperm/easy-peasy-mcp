# Plan for task 113: batch `queries`

## Design

- `src/batch.rs` (new): a request with `queries` — `query` with it is `invalid_option`, it must be a non-empty list
  of objects; the shared fields are checked first (an error fails the request); every item (a string `query`, an
  optional `label`, `dirs`, `timeout`, `traversal`, `output`) becomes the arguments of one `find`: the shared fields
  with the item's `dirs` and `timeout` instead and its `traversal` and `output` merged key by key; a wrong item is
  an error of its section (OUTPUT.md#batch, clarified in its own commit).
- the budget and `max_output` of the whole response are split: an item with its own value keeps it, the rest is
  divided equally among the others in their order, what a query leaves goes to the next ones.
- the response: the text sections `== label ==  <first line>` (the number without a label), in JSON
  `{ "version": 1, "queries": [...] }` with every item's response (without `version`, `label` first) or its error;
  `isError` only when all queries fail.
- `src/engine.rs`: `find` hands a request with `queries` to the batch; `src/description.rs`: the schema.

## Tests

- examples E113.1–E113.7 (fixture `funnel` of task 102)
- unit: the arguments of an item (merging), the split of the budget

## Steps

1. failing tests
2. the batch, the sections, the split

## Risks

- the queries run one after another (the split needs what the earlier ones used) and every query walks on its own
