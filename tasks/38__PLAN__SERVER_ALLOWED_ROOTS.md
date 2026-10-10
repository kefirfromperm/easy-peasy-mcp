# Plan for task 38: `server.allowed_roots`

## Design

- `src/config.rs`:
  - `Source` (`default`, `file: <path>`, `cli`) and `Settings` — the effective configuration of the session, an
    immutable value; for now `allowed_roots` (canonical) and its source.
  - the config file check returns the values of the implemented keys (`IMPLEMENTED`): `server.allowed_roots` must be a
    list of strings (`invalid value for 'server.allowed_roots': a list of paths is expected`), with its line.
  - `Settings::resolve(file, cli_roots, start_dir, home)`: the command line list replaces the file list; `~` expanded,
    relative paths against the start directory, canonicalized; an empty list is the start directory; a root which
    doesn't exist is a `StartError` (with the file and the line of the key when it comes from the file).
  - `get_config` renders the settings: the read-only line, `[server]` with `allowed_roots` as TOML literal strings,
    the source comments aligned four spaces after the longest line of the section.
- `src/engine.rs`: `EngineOptions.settings`; `find` checks that the canonical working directory (the start
  directory, until `dirs` and `default_dirs`) is inside a root: `dir_not_allowed`.
- `src/main.rs`: `--root <dir>`, repeatable; the settings are resolved before the server starts.
- `src/error.rs`: `dir_not_allowed`.
- harness: `FixtureDir.root` is canonical; `<fixture>` in every expected text; startup examples call other tools
  with `json request <tool>`.

## Tests

- examples E38.1–E38.13 (fixture `roots`; E38.6 and E38.8 on Linux only)
- unit: the root resolution (`~`, relative, canonical, missing), the file values and their errors, the rendering
  with and without sources, the literal strings

## Steps

1. harness and failing tests
2. settings, file values, command line, get_config, the working directory check

## Risks

- canonical paths differ from the given ones (macOS `/var` → `/private/var`): the harness compares with the
  canonical fixture path
