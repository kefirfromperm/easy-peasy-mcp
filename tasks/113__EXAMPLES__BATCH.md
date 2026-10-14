# Examples for task 113

Spec: `1`

A JSON response is compared as a JSON value. The examples use the fixture `funnel` of task 102: `src/a.rs`
(`fn parse_query() {}`), `src/b.rs` (`fn other() {}`), `docs/c.md`, `tools/d.py` (`def parse_query(): pass`),
eleven `many/*.txt`.

## E113.1 Several queries in one call

Every query has its section, its legend and its result id; the second one overrides the detail.

Fixture: `funnel`

```json request
{"queries": [{"label": "rust", "query": "lang = rust"}, {"label": "python", "query": "lang = python", "output": {"detail": "count"}}], "output": {"sort": "path asc"}}
```

```text response
== rust ==  files: 2  lines: 0  matches: 0  result: r1
#1 lang = rust

src/
a.rs  [#1]
b.rs  [#1]
== python ==  files: 1  lines: 0  matches: 0  result: r2
```

## E113.2 Without labels

The sections are numbered.

Fixture: `funnel`

```json request
{"queries": [{"query": "text has 'parse_query'"}, {"query": "name = 'c.md'", "dirs": ["docs"]}], "output": {"detail": "files", "sort": "path asc"}}
```

```text response
== 1 ==  files: 2  lines: 2  matches: 2  result: r1
#1 text has 'parse_query'

src/
a.rs  [#1]  lines: 1  matches: 1
tools/
d.py  [#1]  lines: 1  matches: 1
== 2 ==  files: 1  lines: 0  matches: 0  result: r2
#1 name = 'c.md'

docs/
c.md  [#1]
```

## E113.3 An invalid query

Its error is in its section, the other query runs.

Fixture: `funnel`

```json request
{"queries": [{"label": "ok", "query": "lang = python"}, {"label": "bad", "query": "name contains 'a'"}], "output": {"detail": "count"}}
```

```text response
== ok ==  files: 1  lines: 0  matches: 0  result: r1
== bad ==  error[unknown_operator]: unknown operator 'contains'
  name contains 'a'
       ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

## E113.4 All queries invalid

Fixture: `funnel`

```json request
{"queries": [{"query": "name contains 'a'"}, {"label": "two", "qery": "lang = rust"}]}
```

```text error
== 1 ==  error[unknown_operator]: unknown operator 'contains'
  name contains 'a'
       ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
== two ==  error[invalid_option]: unknown field 'qery'
  hint: did you mean 'query'? expected one of: label, query, dirs, timeout, traversal, output
```

## E113.5 In JSON

Fixture: `funnel`

```json request
{"queries": [{"label": "rust", "query": "lang = rust", "output": {"detail": "count"}}, {"query": "name contains 'a'"}], "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "queries": [
    { "label": "rust", "exists": true, "result": "r1", "counts": { "files": 2, "lines": 0, "matches": 0 } },
    { "label": "2", "error": { "code": "unknown_operator", "message": "unknown operator 'contains'",
      "query": "name contains 'a'", "position": { "col": 6, "len": 8 },
      "hint": "did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~" } }
  ]
}
```

## E113.6 `query` and `queries`

Fixture: `funnel`

```json request
{"query": "lang = rust", "queries": [{"query": "lang = python"}]}
```

```text error
error[invalid_option]: 'query' and 'queries' can't be used together
```

```json request
{"queries": []}
```

```text error
error[invalid_option]: 'queries' must be a non-empty list of objects
```

## E113.7 A wrong shared field

It fails the whole request.

Fixture: `funnel`

```json request
{"queries": [{"query": "lang = rust"}], "output": {"detail": "everything"}}
```

```text error
error[invalid_option]: 'output.detail' must be one of: exists, count, facets, files, lines
```
