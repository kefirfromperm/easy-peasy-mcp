# Examples for task 120

Spec: `1`

The conventions of Windows (CONFIG.md#platforms). The examples are marked `` Platform: `windows` `` and run only
there; they are compared exactly, `\` included.

## Fixtures

### windows

```text tree
src/main.rs     2026-10-01 10:00
src/lib.rs      2026-10-01 10:00
data/x.txt      2026-10-01 10:00
```

```text setup
junction vendor -> src
```

File contents are in `tests/fixtures/windows/`. `vendor` is a junction to `src`.

## E120.1 Paths are written with `\`

The directory headers and the full paths of `group_dirs = false`.

Fixture: `windows`

Platform: `windows`

1. Grouped:

```json request
{ "query": "name ~ '*.rs'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src\
lib.rs  [#1]
main.rs  [#1]
```

2. Full paths:

```json request
{ "query": "name ~ '*.rs'", "output": { "sort": "path asc", "group_dirs": false } }
```

```text response
files: 2  lines: 0  matches: 0  result: r2
#1 name ~ '*.rs'

src\lib.rs  [#1]
src\main.rs  [#1]
```

## E120.2 `\` is a separator in query paths

A raw string keeps the `\`; `/` works too.

Fixture: `windows`

Platform: `windows`

```json request
{ "query": "path ~ `src\\*.rs` and not path = 'src/lib.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ `src\*.rs`
#2 path = 'src/lib.rs'

src\
main.rs  [#1]
```

## E120.3 Absolute paths have no `\\?\` prefix

Fixture: `windows`

Platform: `windows`

```json request
{ "query": "name = 'main.rs'", "output": { "path_style": "absolute" } }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

<fixture>\src\
main.rs  [#1]
```

## E120.4 The hidden attribute hides an entry

A file with the hidden attribute is skipped like a name starting with `.`; `hidden = true` includes both.

Fixture: `macos`

Platform: `windows`

1. By default:

```json request
{ "query": "name ~ '*.txt'", "output": { "sort": "path asc" } }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

.\
a.txt  [#1]
```

2. With `hidden = true`:

```json request
{ "query": "name ~ '*.txt'", "traversal": { "hidden": true }, "output": { "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r2
#1 name ~ '*.txt'

.\
.dot.txt  [#1]
a.txt  [#1]
flagged.txt  [#1]
```

## E120.5 A junction is a link

Fixture: `windows`

Platform: `windows`

```json request
{ "query": "type = link" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = link

.\
vendor  [#1]
```

## E120.6 A file reached twice is returned once

With `follow_symlinks` the files of `src` are reached through the junction `vendor` too; the volume serial number and
the file index identify them, so they are returned once, under the first path.

Fixture: `windows`

Platform: `windows`

```json request
{ "query": "name ~ '*.rs'", "traversal": { "follow_symlinks": true }, "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src\
lib.rs  [#1]
main.rs  [#1]
```

## E120.7 The config directory is `%APPDATA%\easy-peasy-mcp`

Fixture: `windows`

Platform: `windows`

```toml appdata/easy-peasy-mcp/config.toml
[server]
timeut = "10s"
```

```text env
APPDATA=<fixture>\appdata
```

```text command
easy-peasy-mcp
```

```text stderr exit 1
easy-peasy-mcp: <fixture>\appdata\easy-peasy-mcp\config.toml:2: unknown key 'server.timeut', did you mean 'server.timeout'?
```

## E120.8 `~` is `%USERPROFILE%`, paths in single quotes

A TOML string in single quotes keeps `\` as it is.

Fixture: `ignore-paths`

Platform: `windows`

```text env
USERPROFILE=<fixture>
```

```toml conf/p.toml
[traversal]
ignore_paths = ['~\conf\personal.ignore']
```

```text command
easy-peasy-mcp --config conf\p.toml
```

```json request
{"query": "name ~ '*.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

.\
keep.log  [#1]
```
