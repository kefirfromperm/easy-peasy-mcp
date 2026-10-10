# Plan for task 36: `update_config` tool

## Design

- `src/config.rs`:
  - the walk over the sections and keys of a TOML document, shared by the config file check and `update_config`:
    `entries(doc)` → `(offset, Entry)` in the order of the text, `Entry` = an unknown section, a section written as a
    value, or a key of a section (`section.key`, a language for `languages`)
  - `update_config(arguments) -> Result<String, Report>`: the arguments (`set` a string, `reset` a list of strings,
    no other field: `invalid_option`); the `reset` keys, then the `set` keys in the order of the TOML: `read_only`
    (`server.allowed_roots`, `server.allow_update` — they are read-only whether implemented or not), `unknown_key`
    (with `did you mean` or the sections), `invalid_toml` (the line, the column in characters and the parser's
    message), a key of the spec which isn't implemented yet: `not_implemented`. With no key implemented, a valid call
    changes nothing: `# nothing changed`. The session configuration, the atomic apply and the result with `# was ...`
    comments come with the first key.
- `src/error.rs`: the codes `invalid_toml`, `read_only`, `unknown_key`.
- `src/engine.rs`, `src/mcp.rs`, `src/description.rs`: the tool `update_config` with the description, schema and
  annotations of TOOLS.md#update_config (`readOnlyHint: false`, `destructiveHint: false`, `idempotentHint: true`,
  `openWorldHint: false`).

## Tests

- examples E36.1–E36.12 (fixture `update-config`)
- unit: the argument errors, each error of `set` and `reset`, the column of `invalid_toml`, `not_implemented` for a
  key of the spec, the order (reset before set, the first key of the TOML)
- end to end: `tools/list` lists `update_config` with its annotations

## Steps

1. failing tests
2. entries walk, update_config, tool definition

## Risks

- `update_disabled` needs `server.allow_update` (task 42)
