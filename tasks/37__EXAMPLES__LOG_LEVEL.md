# Examples for task 37

Startup examples (AGENTS.md). What the server logs is not a contract, so the examples show the responses on stdout
and the usage errors; an end-to-end test checks the log lines on stderr.

## Fixtures

### log-level

```text tree
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/log-level/`. Times are UTC.

## E37.1 Debug logs go to stderr

stdout carries only the protocol: the response is the same at every level.

Fixture: `log-level`

```text command
easy-peasy-mcp --log-level debug
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

## E37.2 Only errors

Fixture: `log-level`

```text command
easy-peasy-mcp --log-level error
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

## E37.3 An unknown level

Fixture: `log-level`

```text command
easy-peasy-mcp --log-level verbose
```

```text stderr exit 2
usage: easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--log-level <level>]
               [--version]
```

## E37.4 A level is missing

Fixture: `log-level`

```text command
easy-peasy-mcp --log-level
```

```text stderr exit 2
usage: easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--log-level <level>]
               [--version]
```
