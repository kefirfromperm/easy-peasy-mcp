# Summary of task 34: Config file

Server version: `0.34.0`.

## What was built

- The config file lookup (CONFIG.md#config-file): `--config <path>`, a non-empty `$EASY_PEASY_CONFIG`, the platform
  file (Linux `$XDG_CONFIG_HOME/easy-peasy-mcp/config.toml` when the variable is absolute, else
  `$HOME/.config/...`; macOS `$HOME/Library/Application Support/...`; Windows `%APPDATA%\...`). The first one is used;
  a named file must exist, the platform file is optional. No project level file is ever read.
- The checks at start (spec clarified first): not TOML, an unknown section (with the list of sections), a section
  written as a value, an unknown key (with `did you mean`), a key of the spec which isn't implemented yet. The first
  error in the file stops the server: `easy-peasy-mcp: <path>:<line>: <message>`, exit code 1; a file which can't be
  read: `easy-peasy-mcp: <path>: the config file doesn't exist` / `can't be read`.
- The command line: `--config <path>`, `--version`; `--root`, `--timeout`, `--no-update`, `--log-level` are
  `the option '...' is not implemented yet` (exit 2); anything else prints the usage of the spec (exit 2).
- With no key implemented, a valid config file has only comments and empty sections. Every key task adds its key to
  the file, the command line, `get_config` and `update_config`.

## Architecture

- `src/config.rs`: `SECTIONS` (the keys of CONFIG.md#full-example), `locate(cli, env, os, exists)` — pure over the
  environment lookup, `load(path)`, `check(text)` over a `toml_edit::Document` with spans, `StartError`.
- `src/main.rs`: `parse_args` → `Command::{Version, Serve}` or `ArgError::{Usage, NotImplemented}`; the config file is
  checked before the runtime starts.
- New dependency `toml_edit` 0.25: the parser with spans; later the TOML of `get_config` / `update_config`.

## Test harness

- AGENTS.md: startup examples — `text command`, `text env`, `toml <path>` and `text stderr exit <code>` blocks,
  `<fixture>`, `Platform:`. `tests/startup.rs` runs them with the binary in the fixture directory, `HOME`,
  `XDG_CONFIG_HOME`, `APPDATA` pointing to an empty directory and without `EASY_PEASY_CONFIG`; a server which starts
  answers the `find` requests of the example over MCP. The MCP client moved to `tests/common` (`McpServer`,
  `server_command`); `tests/examples.rs` skips startup examples.

## Deviations from the plan

None. As planned, the merge of the value sources (defaults ← file ← command line ← `update_config` ← request) comes
with the first key; there is nothing to merge yet.

## Tests

- startup examples E34.1–E34.15 (fixture `config`; E34.13 on Linux only)
- unit: `config::tests` — the lookup order, the platform directories, valid files, every error with its line, a
  missing file

## Limitations and known gaps

- The keys: `get_config` (task 35), `update_config` (36), `--log-level` (37), `allowed_roots` and `--root` (38), and
  the key tasks after them.

## Try it

`easy-peasy-mcp --config ~/.config/easy-peasy-mcp/config.toml`; a file with `[output]\nmax_file = 10` stops the server
with `...:2: unknown key 'output.max_file', did you mean 'output.max_files'?`.
