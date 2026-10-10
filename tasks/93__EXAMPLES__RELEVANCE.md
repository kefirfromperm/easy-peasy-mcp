# Examples for task 93

A JSON response is compared as a JSON value. Every example searches one directory of the fixture whose files differ
in one signal only, so the order doesn't depend on the weights; the path order is the opposite.

## Fixtures

### relevance

```text tree
conditions/a.txt           2026-10-01 10:00
conditions/b.txt           2026-10-01 10:00
density/a.rs               2026-10-01 10:00
density/b.rs               2026-10-01 10:00
depth/deep/x/a.rs          2026-10-01 10:00
depth/z.rs                 2026-10-01 10:00
equal/a.rs                 2026-10-01 10:00
equal/b.rs                 2026-10-01 10:00
fuzzy/a_config_loader_old.rs 2026-10-01 10:00
fuzzy/config_loader.rs     2026-10-01 10:00
kind/a_test.rs             2026-10-01 10:00
kind/b.rs                  2026-10-01 10:00
recency/a.rs               2026-09-24 10:00
recency/b.rs               2026-10-08 10:00
words/a.rs                 2026-10-01 10:00
words/parse_query.rs       2026-10-01 10:00
```

In `tests/fixtures/relevance/`: the files with `// TODO` have only this line, except `density/a.rs`, which has nine
other lines before it; `words/*.rs` are `fn parse_query() {}`; `conditions/a.txt` is `alpha`, `conditions/b.txt`
`alpha beta`; `fuzzy/*.rs` are `x`. Times are UTC.

## E93.1 A better fuzzy match first

Fixture: `relevance`

```json request
{"query": "name ~~ 'cfgldr'", "dirs": ["fuzzy"]}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'

fuzzy/
config_loader.rs  [#1]
a_config_loader_old.rs  [#1]
```

## E93.2 More matches per line first

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["density"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

density/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
a.rs  [#1]  lines: 1  matches: 1
10:// ⟦#1:TODO⟧
```

## E93.3 Shallower paths first

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["depth"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

depth/
z.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
depth/deep/x/
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
```

## E93.4 Source before tests

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["kind"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

kind/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
a_test.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
```

## E93.5 Recently modified first

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["recency"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

recency/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
```

## E93.6 The words of the query in the file name

Fixture: `relevance`

```json request
{"query": "text has 'parse query' /n", "dirs": ["words"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'parse query' /n

words/
parse_query.rs  [#1]  lines: 1  matches: 1
1:fn ⟦#1:parse_query⟧() {}
a.rs  [#1]  lines: 1  matches: 1
1:fn ⟦#1:parse_query⟧() {}
```

## E93.7 More matched conditions first

Fixture: `relevance`

```json request
{"query": "text has 'alpha' or text has 'beta'", "dirs": ["conditions"]}
```

```text response
files: 2  lines: 2  matches: 3  result: r1
#1 text has 'alpha'
#2 text has 'beta'

conditions/
b.txt  [#1 #2]  lines: 1  matches: 2
1:⟦#1:alpha⟧ ⟦#2:beta⟧
a.txt  [#1]  lines: 1  matches: 1
1:⟦#1:alpha⟧
```

## E93.8 Equal scores in path order

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["equal"]}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

equal/
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧
```

## E93.9 The score with `meta`

The score is rounded to two decimals.

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["recency"], "output": {"detail": "files", "meta": true}}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

recency/
b.rs  [#1]  lines: 1  matches: 1  file 8B 2026-10-08 10:00 rust source 1.00
a.rs  [#1]  lines: 1  matches: 1  file 8B 2026-09-24 10:00 rust source 0.95
```

```json request
{"query": "text has 'TODO'", "dirs": ["recency"], "output": {"detail": "files", "meta": true, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": { "files": 2, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "recency/b.rs", "type": "file", "size": 8, "lang": "rust", "kind": "source",
      "modified": "2026-10-08T10:00:00+00:00", "score": 1.0, "lines": 1, "matches": 1, "conditions": ["#1"] },
    { "path": "recency/a.rs", "type": "file", "size": 8, "lang": "rust", "kind": "source",
      "modified": "2026-09-24T10:00:00+00:00", "score": 0.95, "lines": 1, "matches": 1, "conditions": ["#1"] }
  ]
}
```

## E93.10 No score with another order

Fixture: `relevance`

```json request
{"query": "text has 'TODO'", "dirs": ["recency"], "output": {"detail": "files", "meta": true, "sort": "path asc"}}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

recency/
a.rs  [#1]  lines: 1  matches: 1  file 8B 2026-09-24 10:00 rust source
b.rs  [#1]  lines: 1  matches: 1  file 8B 2026-10-08 10:00 rust source
```
