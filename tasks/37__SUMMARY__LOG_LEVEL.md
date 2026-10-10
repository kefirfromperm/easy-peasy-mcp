# Summary of task 37: `--log-level`

Server version: `0.37.0`.

## What was built

- `--log-level error|warn|info|debug` (CONFIG.md#command-line), `warn` by default (spec clarified first); an unknown
  or missing level prints the usage with exit code 2.
- Logs go to stderr only (TOOLS.md#transport); stdout carries only JSON-RPC messages at every level.
- Log lines (not a contract): `info` — `easy-peasy-mcp <version> started in <dir>, config file: <path|none>`, plus the
  lines of `rmcp`; `debug` — `tools/call <tool> <arguments>` and `tools/call <tool> done in <ms> ms, error: <bool>`.

## Architecture

- `src/main.rs`: `parse_args` reads `--log-level` into `Command::Serve`; the `tracing` subscriber is set up with it
  before the config file is checked; `run` logs the start.
- `src/mcp.rs`: the `debug` lines around a tool call.
- `tests/common`: `McpServer::from_child` for a server whose stderr a test reads.

## Deviations from the plan

None.

## Tests

- startup examples E37.1–E37.4 (fixture `log-level`)
- end to end: `tests/mcp_stdio.rs::logs_go_to_stderr` — at `debug` the start and call lines are on stderr and every
  stdout line is JSON-RPC; at the default level stderr is empty

## Limitations and known gaps

None.

## Try it

`easy-peasy-mcp --log-level debug`
