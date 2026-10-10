# Plan for task 34: Config file

## Design

- `src/config.rs` (new, library):
  - `locate(cli, env)`: the config file — `--config`, a non-empty `$EASY_PEASY_CONFIG` (both must exist), the
    platform file (Linux `$XDG_CONFIG_HOME` if absolute, else `$HOME/.config`; macOS
    `$HOME/Library/Application Support`; Windows `%APPDATA%`; optional). `env` is a lookup function, so the order is
    unit tested without touching the process environment. No other place is ever looked at.
  - `load(path)`: reads and parses the file with `toml_edit` (spans), then checks it: the sections and keys of the spec
    (`SECTIONS`, from CONFIG.md#full-example); a section which isn't one, a section written as a value, an unknown
    key (with `did you mean` from `query::did_you_mean`), a key of the spec which isn't implemented yet. The first
    error in file order wins. Lines are computed from the byte offsets of the spans.
  - `StartError { path, line, message }` displayed as `<path>:<line>: <message>`.
- `src/main.rs`: the command line `--config <path>`, `--version`; the other options of the spec (`--root`,
  `--timeout`, `--no-update`, `--log-level`) are `not implemented yet` (exit 2); anything else prints the usage (exit
  2). A start error is printed as `easy-peasy-mcp: <error>` with exit 1.
- The value sources (defaults ← file ← command line ← `update_config` ← request) need keys; with no key implemented
  yet there is nothing to merge, so the layering comes with the first key task — this task provides the file and the
  errors every key task plugs into.
- Dependency: `toml_edit` — the parser with spans (lines in errors), and later the TOML of `get_config` /
  `update_config`, which keeps comments and order.

## Tests

- startup examples E34.1–E34.15 in a new `tests/startup.rs`: the binary with its command line, environment and
  config files; the MCP client moves from `tests/mcp_stdio.rs` to `tests/common`; `tests/examples.rs` skips startup
  examples
- unit: `locate` (each source, the order, an empty variable, a missing named file, the platform paths), `load`
  (each error with its line, valid files: empty, comments, empty sections)

## Steps

1. harness and failing tests
2. config module, command line
3. description (nothing to describe yet: the tools don't change)

## Risks

- the platform paths can only be tested on their platform; the order and the path building are unit tested
