# Examples for task 34

Spec: `1`

The examples of this task are startup examples (AGENTS.md): they run the binary with a command line, environment and
config files.

## Fixtures

### config

```text tree
config.toml                2026-10-01 10:00
easy-peasy-mcp.toml        2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/config/`. `config.toml` and `easy-peasy-mcp.toml` in the working directory are
not valid TOML: they are never read unless named. Times are UTC.

## E34.1 No config file: the built-in defaults

A config file in the working directory is not a config file of the server.

Fixture: `config`

```text command
easy-peasy-mcp
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E34.2 A config file named by `--config`

Comments and an empty section are valid.

Fixture: `config`

```toml conf/empty.toml
# defaults only
[output]
```

```text command
easy-peasy-mcp --config conf/empty.toml
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E34.3 A named file must exist

Fixture: `config`

```text command
easy-peasy-mcp --config missing.toml
```

```text stderr exit 1
easy-peasy-mcp: missing.toml: the config file doesn't exist
```

## E34.4 Not TOML

The line of the error, not of the start of the table.

Fixture: `config`

```toml conf/bad.toml
# broken
[output
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:2: invalid TOML: unclosed table, expected `]`
```

## E34.5 An unknown section

Fixture: `config`

```toml conf/bad.toml
# colors
[colors]
error = "red"
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:2: unknown section 'colors', expected one of: server, traversal, output, escape, query, kinds, git, languages
```

## E34.6 An unknown key

Fixture: `config`

```toml conf/bad.toml
[output]
max_file = 10
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:2: unknown key 'output.max_file', did you mean 'output.max_files'?
```

## E34.8 A dotted key at the top level

Fixture: `config`

```toml conf/bad.toml

output.max_file = 10
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:2: unknown key 'output.max_file', did you mean 'output.max_files'?
```

## E34.9 A section written as a value

Fixture: `config`

```toml conf/bad.toml
output = 1
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:1: 'output' must be a section
```

## E34.11 `$EASY_PEASY_CONFIG`

Fixture: `config`

```toml env.toml
[git]
basse = "main"
```

```text env
EASY_PEASY_CONFIG=env.toml
```

```text command
easy-peasy-mcp
```

```text stderr exit 1
easy-peasy-mcp: env.toml:2: unknown key 'git.basse', did you mean 'git.base'?
```

## E34.12 `--config` comes before `$EASY_PEASY_CONFIG`

Fixture: `config`

```toml conf/empty.toml

```

```toml env.toml
[git]
base = "main"
```

```text env
EASY_PEASY_CONFIG=env.toml
```

```text command
easy-peasy-mcp --config conf/empty.toml
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E34.13 The platform config directory

Fixture: `config`

Platform: `linux`

```toml xdg/easy-peasy-mcp/config.toml
[server]
timeut = "10s"
```

```text env
XDG_CONFIG_HOME=<fixture>/xdg
```

```text command
easy-peasy-mcp
```

```text stderr exit 1
easy-peasy-mcp: <fixture>/xdg/easy-peasy-mcp/config.toml:2: unknown key 'server.timeut', did you mean 'server.timeout'?
```

## E34.15 `--config` without a path

Fixture: `config`

```text command
easy-peasy-mcp --config
```

```text stderr exit 2
usage: easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--allow-write]
               [--output-dir <dir>] [--log-level <level>] [--version]
```
