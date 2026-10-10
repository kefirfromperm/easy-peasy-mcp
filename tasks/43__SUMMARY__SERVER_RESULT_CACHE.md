# Summary of task 43: `server.result_cache`

Server version: `0.43.0`.

## What was built

- `server.result_cache` (CONFIG.md#server, OUTPUT.md#result-ids), default `50`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`. Invalid (spec clarified first): `invalid value
  for 'server.result_cache': a whole number is expected (0: off)` at start, the same hint for `invalid_value`. A
  request can't set it (the `server` section).
- The last N results are kept: the arguments of the request, all matched files (paths as in the response) and a
  fingerprint of their matched lines; older ones are evicted.
- `0` turns the cache off: nothing is kept and the response has no `result: rN`. The ids count the kept results, so a
  response without an id doesn't use one (spec clarified first).

## Architecture

- `src/cache.rs`: `ResultCache::keep(capacity, arguments, files)`, `CachedResult`, `CachedFile`. The cached results
  are read by `within` and `since` (tasks 111–112); until then the fields are only written.
- `src/config.rs`: `Settings.result_cache` with its source, `FileValues.result_cache`, `Mutable::ResultCache`.
- `src/engine.rs`: the cache of the session; `fingerprint` hashes the matched lines of a file.
- `src/output/mod.rs`: `Response.result_id` is optional.

## Deviations from the plan

None.

## Tests

- examples E43.1–E43.5 (fixture `result-cache`)
- unit: `cache::tests::ids_and_eviction`, the file value and the rendering in `config::tests`

## Limitations and known gaps

- `within`, `since` and `result_expired` come with tasks 111–112.

## Try it

`update_config { "set": "server.result_cache = 0" }`
