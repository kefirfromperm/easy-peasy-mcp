# Plan for task 37: `--log-level`

## Design

- `src/main.rs`: `--log-level error|warn|info|debug` (default `warn`), once; any other value or a missing value prints
  the usage (exit 2). The `tracing` subscriber writes to stderr with this level, so stdout stays protocol-only.
- Log lines (not a contract): `info` at start — the server version, the start directory and the config file if any;
  `debug` for every tool call — the tool and its arguments, and when it ends the time it took and whether it is an
  error.
- `src/mcp.rs`: the `debug` lines of the tool calls.

## Tests

- startup examples E37.1–E37.4 (fixture `log-level`)
- end to end (`tests/mcp_stdio.rs`): at `debug` stdout has only JSON-RPC messages and stderr has the start line and
  the call lines; at the default level stderr is empty

## Steps

1. failing examples and end-to-end test
2. command line, subscriber, log lines

## Risks

- none
