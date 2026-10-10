# Summary of task 42: `server.allow_update`

Server version: `0.42.0`.

## What was built

- `server.allow_update` (CONFIG.md#server), default `true`, read-only: from the config file (a boolean, otherwise
  `invalid value for 'server.allow_update': true or false is expected` at start, spec clarified first) or
  `--no-update`; `get_config` lists it in the read-only line and in `[server]` with its source; `update_config`
  rejects it with `read_only` (since task 36).
- With `false` `update_config` is not in `tools/list` (TOOLS.md#tools); a call made anyway fails with
  `error[update_disabled]: update_config is disabled`, hint `the configuration has server.allow_update = false`.
- Every command line option of the spec is implemented: the error for an option not implemented yet is gone.

## Architecture

- `src/config.rs`: `Settings.allow_update` with its source, `FileValues.allow_update`, `CommandLine.no_update`.
- `src/engine.rs`: `Engine::tools` — the tools of the session; `update_disabled` in `call`.
- `src/mcp.rs`: `tools/list` lists the tools of the engine.
- `src/main.rs`: `--no-update`; `ArgError::NotImplemented` removed.
- `src/error.rs`: `update_disabled`.

## Deviations from the plan

None.

## Tests

- examples E42.1–E42.5 (fixture `allow-update`)
- end to end: `tests/mcp_stdio.rs::no_update_hides_update_config`; the not-implemented key test is
  `keys_not_implemented_yet`
- unit: the file value in `roots_are_resolved`, the rendering in `get_config_arguments`

## Limitations and known gaps

None.

## Try it

`easy-peasy-mcp --no-update`
