# Summary of task 115: cancellation

Server version: `0.115.0`.

## What was built

- `notifications/cancelled` stops a running `find` (TOOLS.md#cancellation): the walk, every search thread, the
  slow-read test hook and the hint searches stop as soon as possible; no response is sent for the cancelled request
  (rmcp drops it); a cancellation of an unknown or finished request is ignored, and the next requests are answered
  as usual.

## Architecture

- `src/search/mod.rs`: `Progress::cancel` / `is_cancelled`; `Walk::past_deadline` also stops on the cancellation,
  `slow_read` sleeps in steps of 50 ms. `Progress` is exported for tests and embedding.
- `src/mcp.rs`: a task waits for the request's `CancellationToken` and cancels the `Progress` of the call.

## Deviations from the plan

None.

## Tests

- examples E115.1–E115.2 (fixture `funnel` of task 102)
- `tests/search.rs`: `a_cancelled_search_stops_soon`

## Limitations and known gaps

None.

## Try it

Start a slow `find`, then send `{"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 7}}`.
