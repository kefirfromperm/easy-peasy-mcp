# Examples for task 111

The examples use the fixture `funnel` of task 102: `src/a.rs` (`fn parse_query() {}`), `src/b.rs`
(`fn other() {}`), `docs/c.md`, `tools/d.py`, eleven `many/*.txt`.

## E111.1 Of those, which also have X?

Fixture: `funnel`

1. `find`

```json request
{"query": "lang = rust", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = rust

src/
a.rs  [#1]
b.rs  [#1]
```

2. `find` — only the files of `r1`

```json request
{"within": "r1", "query": "text has 'parse'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 text has 'parse'

src/
a.rs  [#1]  lines: 1  matches: 1
1:fn ⟦#1:parse⟧_query() {}
```

## E111.2 No walk

`dirs` and the walk options are ignored: the files of `r1` are searched even with `max_depth = 0`, which leaves
nothing to walk.

Fixture: `funnel`

1. `find`

```json request
{"query": "lang = rust", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = rust

src/
a.rs  [#1]
b.rs  [#1]
```

2. `find`

```json request
{"within": "r1", "dirs": ["tools"], "query": "text has 'fn'", "traversal": {"max_depth": 0}, "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 2  matches: 2  result: r2
#1 text has 'fn'

src/
a.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn⟧ parse_query() {}
b.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn⟧ other() {}
```

## E111.3 The paths of the result

`r1` was searched in `src`: the paths are written as in `r1`, and `path` sees them relative to `src`.

Fixture: `funnel`

1. `find`

```json request
{"dirs": ["src"], "query": "lang = rust", "output": {"sort": "path asc", "group_dirs": false}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = rust

src/a.rs  [#1]
src/b.rs  [#1]
```

2. `find`

```json request
{"within": "r1", "query": "path = 'a.rs'", "output": {"group_dirs": false}}
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 path = 'a.rs'

src/a.rs  [#1]
```

## E111.4 An unknown result

Fixture: `funnel`

```json request
{"within": "r7", "query": "text has 'parse'"}
```

```text error
error[result_expired]: the result r7 is unknown or no longer kept
  hint: the server keeps the last server.result_cache results
```

## E111.5 Not a result id

Fixture: `funnel`

```json request
{"within": "first", "query": "text has 'parse'"}
```

```text error
error[invalid_option]: 'within' must be a result id like 'r12'
```
