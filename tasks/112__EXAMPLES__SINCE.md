# Examples for task 112

A JSON response is compared as a JSON value. The examples use the fixture `funnel` of task 102: `src/a.rs`
(`fn parse_query() {}`), `src/b.rs` (`fn other() {}`), `docs/c.md`, `tools/d.py` (`def parse_query(): pass`). The
files don't change between the requests: the differences come from the queries.

## E112.1 Added, changed, removed

`src/a.rs` matches as before, `src/b.rs` matches now with a matched line, `tools/d.py` is new, `docs/c.md` doesn't
match any more.

Fixture: `funnel`

1. `find`

```json request
{"query": "name ~ '*.rs' or name = 'c.md'"}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 name = 'c.md'

src/
a.rs  [#1]
b.rs  [#1]
docs/
c.md  [#2]
```

2. `find` — the differences to `r1`

```json request
{"since": "r1", "query": "name = 'a.rs' or text has 'other' or text has 'def'", "output": {"sort": "path asc"}}
```

```text response
files: 3  lines: 2  matches: 2  result: r2  since: r1  unchanged: 1
#1 name = 'a.rs'
#2 text has 'other'
#3 text has 'def'

~ src/b.rs  [#2]  lines: 1  matches: 1
1:fn ⟦#2:other⟧() {}
+ tools/d.py  [#3]  lines: 1  matches: 1
1:⟦#3:def⟧ parse_query(): pass
- docs/c.md
```

## E112.2 In JSON

Fixture: `funnel`

1. `find`

```json request
{"query": "name ~ '*.rs' or name = 'c.md'"}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 name = 'c.md'

src/
a.rs  [#1]
b.rs  [#1]
docs/
c.md  [#2]
```

2. `find`

```json request
{"since": "r1", "query": "name = 'a.rs' or text has 'other'", "output": {"format": "json", "detail": "files"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "since": "r1",
  "unchanged": 1,
  "counts": { "files": 2, "lines": 1, "matches": 1 },
  "legend": { "#1": "name = 'a.rs'", "#2": "text has 'other'" },
  "files": [
    { "path": "src/b.rs", "change": "changed", "lines": 1, "matches": 1, "conditions": ["#2"] },
    { "path": "docs/c.md", "change": "removed" }
  ]
}
```

## E112.3 Nothing changed

Fixture: `funnel`

1. `find`

```json request
{"query": "lang = rust"}
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
{"since": "r1", "query": "lang = rust"}
```

```text response
files: 2  lines: 0  matches: 0  result: r2  since: r1  unchanged: 2
#1 lang = rust
```

## E112.4 An unknown result

Fixture: `funnel`

```json request
{"since": "r3", "query": "lang = rust"}
```

```text error
error[result_expired]: the result r3 is unknown or no longer kept
  hint: the server keeps the last server.result_cache results
```

## E112.5 Not a result id

Fixture: `funnel`

```json request
{"since": 1, "query": "lang = rust"}
```

```text error
error[invalid_option]: 'since' must be a result id like 'r12'
```
