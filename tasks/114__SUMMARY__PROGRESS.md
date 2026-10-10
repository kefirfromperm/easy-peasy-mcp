# Summary of task 114: progress notifications

Server version: `0.114.0`.

## What was built

- A `tools/call` with a `progressToken` in `_meta` gets `notifications/progress` about once a second while it runs
  (TOOLS.md#progress): `progress` the scanned files (the entries whose conditions were evaluated, matched or not; it
  only grows), no `total`, `message` `scanned N files, M matched, T.Ts`. The first one comes after one second, so a
  shorter search sends none; a call without a token gets none.
- `EASY_PEASY_TEST_SLOW_READS` (milliseconds) sets the slow reads of the binary: the test hook of the startup
  examples.
- Startup examples can be JSON-RPC exchanges: `json send`, `json receive` (a subset of the message, `*` wildcards),
  `json never` (AGENTS.md).

## Architecture

- `src/search/mod.rs`: `Progress` (atomic counters), `SearchOptions.progress`, counted in `evaluate` and on every
  match.
- `src/engine.rs`: `Engine::call_with(tool, arguments, progress)`, threaded through `find_with`, `batch`,
  `try_find` and `run`.
- `src/mcp.rs`: `notify_progress` — a task with a one second interval which ends with the call.
- `src/main.rs`: the test hook; `tests/common/mod.rs`: `Exchange` (a reader thread, `receive`, `never`, `matches`);
  `tests/startup.rs`: the exchange blocks and `Slow reads`.

## Deviations from the plan

- AGENTS.md got the exchange blocks and the hook, TOOLS.md what the progress counts (separate commits).

## Tests

- examples E114.1–E114.3 (fixture `funnel` of task 102)
- unit: `exchange_blocks_match_messages` in `tests/startup.rs`

## Limitations and known gaps

None.

## Try it

`{"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has
'req-7f3a9c'"}, "_meta": {"progressToken": "p7"}}}` in a big tree.
