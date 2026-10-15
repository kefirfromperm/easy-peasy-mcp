# Examples for task 135

The queries of a batch with the same walk share it (OUTPUT.md#batch): every section is the response of its query
alone, byte for byte. The examples are written for the current spec version 2. Examples with a command line are
startup examples (AGENTS.md).

## Fixtures

### shared-walk

```text tree
.hidden/h.rs       2026-10-01 10:00
docs/n.md          2026-10-01 10:00
src/a.rs           2026-10-01 10:00
src/b.rs           2026-10-01 10:00
```

```text setup
bytes data.bin 00ff00ff00
```

File contents are in `tests/fixtures/shared-walk/`: `src/a.rs` defines `fn parse_query()` (line 1) and calls it
(line 4), `src/b.rs` calls it (line 2), `docs/n.md` names it (line 3), the hidden `.hidden/h.rs` defines
`parse_query_hidden`; `data.bin` gives `decode_errors` to every query which reads it. The walk sees four
files: `data.bin`, `docs/n.md`, `src/a.rs`, `src/b.rs`.

## E135.1 Three queries, one walk

Fixture: `shared-walk`

```json request
{"queries": [{"label": "defs", "query": "text =~ `fn\\s+parse_query\\(`"}, {"label": "calls", "query": "text has 'parse_query();'", "output": {"detail": "files"}}, {"label": "docs", "query": "kind = doc and text has 'parse_query'", "output": {"detail": "count"}}], "output": {"sort": "path asc"}}
```

```text response
== defs ==  files: 1  lines: 1  matches: 1  result: r1

src/
a.rs  lines: 1  matches: 1
1:⟦fn parse_query(⟧) {}

warnings left out: decode_errors (1)
== calls ==  files: 2  lines: 2  matches: 2  result: r2

src/
a.rs  lines: 1  matches: 1
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
== docs ==  files: 1  lines: 1  matches: 1  result: r3
```

## E135.2 The same queries alone

The sections of E135.1 are the responses of its queries alone, with the same result ids.

Fixture: `shared-walk`

1. defs

```json request
{"query": "text =~ `fn\\s+parse_query\\(`", "output": {"sort": "path asc"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1

src/
a.rs  lines: 1  matches: 1
1:⟦fn parse_query(⟧) {}

warnings left out: decode_errors (1)
```

2. calls

```json request
{"query": "text has 'parse_query();'", "output": {"detail": "files", "sort": "path asc"}}
```

```text response
files: 2  lines: 2  matches: 2  result: r2

src/
a.rs  lines: 1  matches: 1
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
```

3. docs

```json request
{"query": "kind = doc and text has 'parse_query'", "output": {"detail": "count", "sort": "path asc"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r3
```

## E135.3 The warnings of the shared walk

Every query which reads `data.bin` has its warning; the query on names reads no file and has none.

Fixture: `shared-walk`

```json request
{"queries": [{"label": "text", "query": "text has 'parse_query'"}, {"label": "names", "query": "name ~ '*.rs'"}, {"label": "count", "query": "text has 'fn'", "output": {"detail": "count"}}], "output": {"detail": "files", "sort": "path asc", "show": ["warnings"]}}
```

```text response
== text ==  files: 3  lines: 4  matches: 4  result: r1

docs/
n.md  lines: 1  matches: 1
src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1

warning: decode_errors (1): data.bin
== names ==  files: 2  lines: 0  matches: 0  result: r2

src/
a.rs
b.rs
== count ==  files: 2  lines: 3  matches: 3  result: r3

warning: decode_errors (1): data.bin
```

## E135.4 Own `dirs` and own `traversal`: their own walks

Fixture: `shared-walk`

```json request
{"queries": [{"label": "all", "query": "text has 'parse_query'"}, {"label": "src", "query": "text has 'parse_query'", "dirs": ["src"]}, {"label": "hidden", "query": "text has 'parse_query'", "traversal": {"hidden": true}}], "output": {"detail": "files", "sort": "path asc"}}
```

```text response
== all ==  files: 3  lines: 4  matches: 4  result: r1

docs/
n.md  lines: 1  matches: 1
src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
== src ==  files: 2  lines: 3  matches: 3  result: r2

src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1
== hidden ==  files: 4  lines: 5  matches: 5  result: r3

.hidden/
h.rs  lines: 1  matches: 1
docs/
n.md  lines: 1  matches: 1
src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
```

## E135.5 Own `dirs` and `traversal` alone

Fixture: `shared-walk`

1. src

```json request
{"query": "text has 'parse_query'", "dirs": ["src"], "output": {"detail": "files", "sort": "path asc"}}
```

```text response
files: 2  lines: 3  matches: 3  result: r1

src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1
```

2. hidden

```json request
{"query": "text has 'parse_query'", "traversal": {"hidden": true}, "output": {"detail": "files", "sort": "path asc"}}
```

```text response
files: 4  lines: 5  matches: 5  result: r2

.hidden/
h.rs  lines: 1  matches: 1
docs/
n.md  lines: 1  matches: 1
src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
```

## E135.6 A timeout of its own

Every read takes 300ms: the query with a timeout of 100ms is cut, the other one in the same walk is not.

Fixture: `shared-walk`

Slow reads: `300ms`

```json request
{"queries": [{"label": "short", "query": "text has 'parse_query'", "timeout": "100ms"}, {"label": "long", "query": "text has 'parse_query'"}], "output": {"detail": "files", "sort": "path asc"}}
```

```text response
== short ==  files: 0  lines: 0  matches: 0  result: r1  truncated: yes (timeout)
funnel:
  text has 'parse_query'                 0

warnings left out: timeout
== long ==  files: 3  lines: 4  matches: 4  result: r2

docs/
n.md  lines: 1  matches: 1
src/
a.rs  lines: 2  matches: 2
b.rs  lines: 1  matches: 1

warnings left out: decode_errors (1)
```

## E135.7 `exists` and `lines` in one walk

The `exists` query stops at its first match, the walk goes on for the `lines` query.

Fixture: `shared-walk`

```json request
{"queries": [{"label": "any", "query": "name ~ '*.rs' and text has 'parse_query'", "output": {"detail": "exists"}}, {"label": "lines", "query": "name ~ '*.rs' and text has 'parse_query'"}], "output": {"sort": "path asc"}}
```

```text response
== any ==  exists: yes
== lines ==  files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 text has 'parse_query'

src/
a.rs  [#1 #2]  lines: 2  matches: 2
1:fn ⟦#2:parse_query⟧() {}
⇥4
4:⟦#2:parse_query⟧();
b.rs  [#1 #2]  lines: 1  matches: 1
⇥4
2:⟦#2:parse_query⟧();
```

## E135.8 The progress of a shared walk

Three queries over five files (the four of the fixture and `conf/o.toml`): the progress counts five scanned files,
not fifteen. One search thread and slow reads make the search take a
few seconds; the progress grows by at most two between two notifications, so a count of fifteen would pass 6 or 7.

Fixture: `shared-walk`

Slow reads: `700ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"queries": [{"query": "text has 'parse_query'"}, {"query": "text has 'fn'"}, {"query": "text has 'query'"}], "output": {"detail": "count"}}, "_meta": {"progressToken": "p9"}}}
```

```json receive
{"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progressToken": "p9", "progress": 5.0, "message": "scanned 5 files, * matched, *s"}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "== 1 ==  files: 3  lines: 4  matches: 4  result: r1\nwarnings left out: decode_errors (1)\n== 2 ==  files: 2  lines: 3  matches: 3  result: r2\nwarnings left out: decode_errors (1)\n== 3 ==  files: 3  lines: 4  matches: 5  result: r3\nwarnings left out: decode_errors (1)\n"}], "isError": false}}
```

```json never
{"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progressToken": "p9", "progress": 6.0}}
```

```json never
{"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progressToken": "p9", "progress": 7.0}}
```
