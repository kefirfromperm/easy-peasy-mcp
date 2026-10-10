# Plan for task 112: `since`

## Design

- `src/request.rs`: `since` — a result id `rN` (`'since' must be a result id like 'r12'`), `Request.since`.
- `src/engine.rs`: with `since` the request runs as usual and its files are compared with the kept result (the same
  path and member): `+` added, `~` changed (another fingerprint of the matched lines), unchanged ones counted,
  `-` removed (in the kept result only, or deleted); the response lists the added and changed files in the order of
  the result, then the removed ones (OUTPUT.md#changes-since, clarified in its own commit); the current result is
  kept as usual; an unknown id is `result_expired`.
- `src/output/mod.rs`, `src/output/json.rs`: `Response.since` — the id, the unchanged files, the change of every
  listed file and the counts of the current result: the header `since: r12  unchanged: N`, the marks, full paths
  without directory groups, a removed file as its path only; JSON `since`, `unchanged`, `change`.
- `src/description.rs`: the schema.

## Tests

- examples E112.1–E112.5 (fixture `funnel` of task 102)
- unit: the request field, the comparison (added, changed, unchanged, removed)

## Steps

1. failing tests
2. the comparison, the output

## Risks

- none
