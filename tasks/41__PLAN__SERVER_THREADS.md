# Plan for task 41: `server.threads`

## Design

- `src/config.rs`: `server.threads` (a whole number, `0` — the number of CPU cores) in the file values, `Settings`
  (with its source) and as a `Mutable` key; `get_config` shows it.
- `src/search/mod.rs`: `SearchOptions.threads` sets the threads of the parallel walker (`WalkBuilder::threads`,
  where `0` is the number of CPU cores); the file search runs in the walker threads.
- `src/engine.rs`: the threads of the settings snapshot.
- Results don't depend on the threads: the files are sorted after the walk, the dedup of hard links and of
  overlapping directories keeps the first path in path order.

## Tests

- examples E41.1–E41.5 (fixture `threads`)
- unit: the value check; the search with 1 and 4 threads gives the same files (`tests/search.rs`)

## Steps

1. failing tests
2. key, walker threads

## Risks

- none
