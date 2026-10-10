# Summary of task 35: `get_config` tool

Server version: `0.35.0`.

## What was built

- The tool `get_config` (TOOLS.md#get_config, CONFIG.md#get_config): listed after `find` with the description, the
  input schema and the annotations (`readOnlyHint: true`, `openWorldHint: false`) of the spec.
- The result: TOML with the first line `# easy-peasy-mcp <version>, spec 1, supported specs: 1`. No configuration
  key is implemented yet, so that is all; the read-only line, the sections in the order of the spec, the values (or
  `# key =  (not set)`) and the source comments of `sources: true` appear with the key tasks.
- The arguments: `section` (one of the sections of the spec), `sources` (a boolean); otherwise `invalid_option`: an
  unknown field or section with `did you mean` and the expected names, a wrong type.
- The server `instructions` and the first lines of the `find` description have the `get_config` sentence of the spec;
  the first line of the `find` description now names content search too.

## Architecture

- `src/config.rs`: `get_config(arguments)`, `unknown_name`, `GET_CONFIG_FIELDS`.
- `src/engine.rs`: `Engine::TOOLS` = `find`, `get_config`; `call` dispatches.
- `src/mcp.rs`: the tool definition; `src/description.rs`: `GET_CONFIG_DESCRIPTION`, `get_config_schema`,
  `INSTRUCTIONS`.
- Examples: `<version>` in a response stands for the server version (AGENTS.md), replaced by both example runners.

## Deviations from the plan

None.

## Tests

- examples E35.1–E35.6 (fixture `get-config`)
- unit: `config::tests::get_config_arguments`
- end to end: `tests/mcp_stdio.rs` checks the tool list, the `get_config` annotations, schema and description, and
  the instructions

## Limitations and known gaps

- Every key task adds its key to the result (with its source), starting with `update_config` (task 36) and the
  server keys (tasks 38–42).

## Try it

`tools/call get_config {}`
