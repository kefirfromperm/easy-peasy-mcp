# Plan for task 42: `server.allow_update`

## Design

- `src/config.rs`: `server.allow_update` (a boolean, read-only) in the file values and `Settings` with its source;
  `--no-update` in `CommandLine`; `get_config` lists it in the read-only line and in `[server]`.
- `src/engine.rs`: the tools of the session (`Engine::tools`): `update_config` only with `allow_update`; a call
  made anyway fails with `update_disabled`.
- `src/mcp.rs`: `tools/list` lists the tools of the engine; a call of `update_config` reaches the engine even when
  it is not listed.
- `src/main.rs`: `--no-update`.
- `src/error.rs`: `update_disabled`.

## Tests

- examples E42.1–E42.5 (fixture `allow-update`)
- end to end: with `--no-update` `tools/list` has `find` and `get_config` only
- unit: the file value

## Steps

1. failing tests
2. key, command line, tools of the session

## Risks

- none
