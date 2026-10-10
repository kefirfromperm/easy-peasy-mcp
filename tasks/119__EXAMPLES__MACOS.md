# Examples for task 119

The conventions of macOS (CONFIG.md#platforms). The examples marked `` Platform: `macos` `` run only there; the others
hold on every platform.

## Fixtures

### macos

```text tree
README.md       2026-10-01 10:00
a.txt           2026-10-01 10:00
flagged.txt     2026-10-01 10:00
.dot.txt        2026-10-01 10:00
```

```text setup
hide flagged.txt
file café.md
```

File contents are in `tests/fixtures/macos/`. `flagged.txt` has the `hidden` flag on macOS (`chflags hidden`), the
hidden attribute on Windows. `café.md` is created with a decomposed name (NFD): `e` and the combining acute accent
U+0301, the form older macOS file systems and many macOS programs write.

## E119.1 `path_case = native` is case-insensitive on macOS

Fixture: `macos`

Platform: `macos`

1. Without a `path_case`:

```json request
{ "query": "name = 'readme.md'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'readme.md'

./
README.md  [#1]
```

2. With `path_case = "sensitive"`:

```json request
{ "query": "name = 'readme.md'", "traversal": { "path_case": "sensitive" }, "output": { "detail": "count", "funnel": "never" } }
```

```text response
files: 0  lines: 0  matches: 0  result: r2
```

## E119.2 The `hidden` flag hides an entry

A file with the `hidden` flag is skipped like a name starting with `.`; `hidden = true` includes both.

Fixture: `macos`

Platform: `macos`

1. By default:

```json request
{ "query": "name ~ '*.txt'", "output": { "sort": "path asc" } }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

./
a.txt  [#1]
```

2. With `hidden = true`:

```json request
{ "query": "name ~ '*.txt'", "traversal": { "hidden": true }, "output": { "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r2
#1 name ~ '*.txt'

./
.dot.txt  [#1]
a.txt  [#1]
flagged.txt  [#1]
```

## E119.3 A decomposed name matches a composed query

The name `café.md` on disk is decomposed (NFD), the query composed (NFC): names are compared in NFC form on every
platform.

Fixture: `macos`

```json request
{ "query": "name = 'café.md'", "output": { "detail": "count" } }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
```

## E119.4 The macOS config directory

The config file is looked up in `~/Library/Application Support/easy-peasy-mcp/config.toml`.

Fixture: `macos`

Platform: `macos`

```toml Library/Application Support/easy-peasy-mcp/config.toml
[server]
timeut = "10s"
```

```text env
HOME=<fixture>
```

```text command
easy-peasy-mcp
```

```text stderr exit 1
easy-peasy-mcp: <fixture>/Library/Application Support/easy-peasy-mcp/config.toml:2: unknown key 'server.timeut', did you mean 'server.timeout'?
```

## E119.5 The creation date is known on macOS

Every file was created when the fixture was set up, so all of them were created after 2000.

Fixture: `macos`

Platform: `macos`

```json request
{ "query": "created > 2000-01-01", "output": { "detail": "count" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
```
