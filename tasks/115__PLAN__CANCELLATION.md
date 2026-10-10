# Plan for task 115: cancellation

## Design

- `src/search/mod.rs`: `Progress` gets a cancellation flag (`cancel`, `is_cancelled`); `Walk::past_deadline` stops
  the walk and every search thread when it is set, the slow-read hook sleeps in short steps and stops too; the hint
  searches use the same options, so they stop as well.
- `src/mcp.rs`: a task waits for the `CancellationToken` of the request (`context.ct`, cancelled by rmcp on
  `notifications/cancelled`) and sets the flag; rmcp drops the response of a cancelled request and ignores
  cancellations of unknown or finished requests (TOOLS.md#cancellation).

## Tests

- examples E115.1–E115.2 (fixture `funnel` of task 102)
- `tests/search.rs`: a slow search stops soon after its flag is set

## Steps

1. failing tests
2. the flag, the stop, the MCP task

## Risks

- none
