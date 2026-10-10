# Summary of task 41: `server.threads`

Server version: `0.41.0`.

## What was built

- `server.threads` (CONFIG.md#server), default `0` — the number of CPU cores: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`. Invalid (spec clarified first): `invalid value
  for 'server.threads': a whole number is expected (0: the number of CPU cores)` at start, the same hint for
  `invalid_value`. A request can't set it (the `server` section).
- The walk and the file search run in the threads of the parallel walker (`WalkBuilder::threads`).
- The results don't depend on the threads: the files are sorted after the walk and the duplicates keep the first path
  in path order.

## Architecture

- `src/config.rs`: `Settings.threads` with its source, `FileValues.threads`, `Mutable::Threads`.
- `src/search/mod.rs`: `SearchOptions.threads`; `src/engine.rs`: the threads of the settings snapshot.

## Deviations from the plan

None.

## Tests

- examples E41.1–E41.5 (fixture `threads`): the same response with 1 thread and with the default
- integration: `tests/search.rs::results_dont_depend_on_the_threads` (200 files, 1, 4 and all threads)
- unit: the file value in `roots_are_resolved`, the rendering in `get_config_arguments`

## Limitations and known gaps

None.

## Try it

`update_config { "set": "server.threads = 2" }`
