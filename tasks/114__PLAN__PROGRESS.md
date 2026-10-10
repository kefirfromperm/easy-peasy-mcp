# Plan for task 114: progress notifications

## Design

- `src/engine.rs`: `Progress` — the scanned and matched entries of the running search (atomic counters) and when the
  request arrived; `Engine::call_with(tool, arguments, &Call)` passes it (and later the cancellation) to the search;
  `EngineOptions` stays as it is.
- `src/search/mod.rs`: `SearchOptions.progress`; every evaluated entry counts as scanned, every found one as
  matched.
- `src/mcp.rs`: with a `progressToken` in `_meta` a task sends `notifications/progress` every second while the call
  runs: `progress` the scanned files, no `total`, `message` `scanned N files, M matched, T.Ts`; the first one after
  one second, so a shorter search sends none (TOOLS.md#progress, clarified in its own commit).
- `src/main.rs`: `EASY_PEASY_TEST_SLOW_READS` (milliseconds) sets the slow reads of the binary, the test hook of the
  startup examples (AGENTS.md, in its own commit).
- `tests/startup.rs`, `tests/common/mod.rs`: the JSON-RPC exchange blocks `json send`, `json receive`, `json never`
  over a reader thread; `Slow reads` of a startup example.

## Tests

- examples E114.1–E114.3 (fixture `funnel` of task 102)
- unit: the progress message, the matching of the exchange blocks

## Steps

1. failing tests (the harness)
2. the counters, the notifications, the hook

## Risks

- the examples depend on timing only loosely: the search takes several seconds with slow reads and one thread
