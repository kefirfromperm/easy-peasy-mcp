# Plan for task 35: `get_config` tool

## Design

- `src/config.rs`: `get_config(arguments) -> Result<String, Report>` — the arguments checked like `find` arguments
  (`invalid_option`: an unknown field with `did you mean`, an unknown section with `did you mean` and the list, a
  wrong type); the result: the first line `# easy-peasy-mcp <version>, spec 1, supported specs: 1`, the read-only line
  and the sections with their implemented keys in the order of the spec (`section` filters, `sources` adds the source
  comments). No key is implemented yet: the key tasks add a key to a table of implemented keys with its value and
  source, and the result grows; the read-only line and a section appear with their first key.
- `src/engine.rs`: the tool `get_config`; `Engine::TOOLS` = `find`, `get_config`.
- `src/mcp.rs`: the tool definition (TOOLS.md#get_config: description, schema, annotations).
- `src/description.rs`: the server `instructions` and the first lines of the `find` description get the
  `get_config` sentence of the spec; the `get_config` description and schema.
- `tests/examples.rs`, `tests/startup.rs`: `<version>` in a response is the server version.

## Tests

- examples E35.1–E35.6 (fixture `get-config`)
- unit: the argument errors; the first line
- end to end (`tests/mcp_stdio.rs`): `tools/list` lists `find` and `get_config` with their annotations; the
  instructions

## Steps

1. failing tests
2. tool, definition, instructions

## Risks

- none: the result is small until the key tasks
