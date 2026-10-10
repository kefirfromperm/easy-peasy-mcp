# Summary of task 116: concurrent requests

Server version: `0.116.0`.

## What was built

- parallel `find` calls share the threads of `server.threads` (0: the number of CPU cores, TOOLS.md#concurrency): a
  search takes the free threads (at least one, waiting while none is free) and gives them back when its response is
  written; a search which waits gives up on its deadline (`timeout_ms`) or cancellation
- each `find` reads a snapshot of the configuration at its start; `update_config` calls are applied one after
  another; result ids stay unique and increase in the order the results are kept
- a cancelled `find` keeps no result: its result id is not used up

## Architecture

- `src/engine.rs`: `Slots` (the count of busy threads under a mutex and a condition variable) and `Lease` (the
  threads a search took, given back on drop); `Run` holds the lease until the response is written.
- `src/error.rs`: `ErrorCode::Cancelled` — the error of a cancelled call, which rmcp never sends.
- `tests/common/mod.rs`: `json receive` takes any message no earlier `receive` took, so concurrent responses can come
  in any order (AGENTS.md).

## Deviations from the plan

- a cancelled call returns `Cancelled` before keeping its result: with shared threads the next request waits for the
  cancelled one, which would otherwise take the next result id (E115.1).

## Tests

- example E116.1 (fixture `funnel` of task 102)
- unit: `engine::tests::searches_share_the_threads`, `tests/startup.rs`: `exchange_blocks_match_messages`

## Limitations and known gaps

- with `server.threads = 1` the searches run one after another.

## Try it

Send two `find` calls without waiting for the first response, with an `update_config` between them: both are
answered, each with the configuration it started with.
