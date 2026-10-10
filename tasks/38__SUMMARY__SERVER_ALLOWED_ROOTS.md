# Summary of task 38: `server.allowed_roots`

Server version: `0.38.0`.

## What was built

- `server.allowed_roots` (CONFIG.md#server), the first configuration key: from the config file (a list of strings,
  otherwise `invalid value for 'server.allowed_roots': a list of paths is expected` at start) or `--root <dir>`
  (repeatable, replaces the list of the file); default `[]` — the start directory.
- Paths: `~` expanded (`HOME`, `USERPROFILE` on Windows), relative paths against the start directory, canonicalized
  at start; a root which doesn't exist stops the server: `the allowed root '<root>' doesn't exist`, after
  `<file>:<line>: ` for a root of the config file (spec clarified first).
- `get_config` (spec clarified first): the read-only line `# read-only: server.allowed_roots`, `[server]` with the
  canonical roots in effect as TOML literal strings, and with `sources` the source comment (`default`, `file:
  <path>`, `cli`) four spaces after the longest line of the section.
- `update_config` rejects the key with `read_only` (since task 36).
- `find`: the working directory (the start directory until task 39) must be inside a root, otherwise
  `error[dir_not_allowed]: the working directory '.' is outside of the allowed roots`, hint `get_config shows the
  allowed roots`.

## Architecture

- `src/config.rs`: `Source`, `FileValues` (the implemented keys of a config file with their lines), `ConfigFile`,
  `Settings` (the effective configuration: canonical roots and their source) with `defaults`, `resolve` and
  `is_allowed`; `check` returns the file values; `get_config` renders the settings (`push_lines`, `toml_path`);
  `StartError.path` is optional for command line errors.
- `src/main.rs`: `--root`; `start` locates and loads the config file, resolves the settings, then serves.
- `src/engine.rs`: `EngineOptions.settings`; the working directory check in `find`.
- harness: the fixture root is canonical; `<fixture>` in every expected text (AGENTS.md); startup examples call any
  tool with `json request <tool>`.

## Deviations from the plan

- The `get_config` examples of tasks 35 and 36 were updated: they show the new key (a separate commit with the
  reason); every key task updates them.

## Tests

- examples E38.1–E38.13 (fixture `roots`; E38.6 and E38.8 on Linux only)
- unit: `config::tests::roots_are_resolved`, the file values and errors in `valid_files` / `errors_with_lines`, the
  rendering in `get_config_arguments`

## Limitations and known gaps

- Several working directories, `dirs` and `default_dirs` come with task 39; symlinks out of the roots with
  `follow_symlinks`.

## Try it

`easy-peasy-mcp --root ~/projects`, then `tools/call get_config { "sources": true }`.
