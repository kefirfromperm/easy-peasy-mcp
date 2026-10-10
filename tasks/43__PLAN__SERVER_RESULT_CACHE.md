# Plan for task 43: `server.result_cache`

## Design

- `src/config.rs`: `server.result_cache` (a whole number, `0` — off) in the file values, `Settings` with its source
  and as a `Mutable` key; `get_config` shows it.
- `src/engine.rs`: `ResultCache` — the last N results: the request (its arguments), all matched files and a
  fingerprint of their matched lines; a new result gets the next id and evicts the oldest beyond N. With `0` nothing
  is kept and the response has no id; ids count the kept results. The cached results are read by `within` and
  `since` (tasks 111–112).
- `src/output/mod.rs`: `Response.result_id` is optional.

## Tests

- examples E43.1–E43.5 (fixture `result-cache`)
- unit: the eviction and the ids of `ResultCache`

## Steps

1. failing tests
2. key, cache, optional id

## Risks

- the cache holds the matched files of the last results in memory: paths and a hash per file, small
