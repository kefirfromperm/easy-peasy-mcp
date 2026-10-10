# Plan for task 116: concurrent requests

## Design

- `src/engine.rs`: `Slots` — the threads of `server.threads` (0: the number of CPU cores) shared by all running
  searches: a search takes the free ones (at least one, waiting while none is free, giving up on its deadline or
  cancellation) for its walk and gives them back when its response is written (`Lease`).
- the snapshot of the configuration at the start of a `find` and the one-by-one `update_config` exist already
  (`Engine::settings`, the write lock); result ids come from the mutex of the cache.
- `tests/common/mod.rs`: `json receive` takes any message no earlier block took (AGENTS.md, in its own commit), so
  concurrent responses can come in any order.

## Tests

- example E116.1 (fixture `funnel` of task 102)
- unit: the slots (taking the free ones, waiting, giving back)

## Steps

1. failing tests
2. the slots

## Risks

- a search waits for the threads of the others: with `server.threads = 1` the searches run one after another
