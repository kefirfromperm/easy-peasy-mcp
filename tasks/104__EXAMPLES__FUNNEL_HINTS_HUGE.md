# Examples for task 104

A JSON response is compared as a JSON value. The examples list one or two files (`max_files`) of eleven, so the
result is huge: more than 10 × `max_files` files.

## Fixtures

### huge

```text tree
docs/d1.md                 2026-10-01 10:00
src/b1.rs                  2026-10-01 10:00
src/b2.rs                  2026-10-01 10:00
tests/t1.rs                2026-10-01 10:00
tests/t2.rs                2026-10-01 10:00
vendor/lib/a1.rs           2026-10-01 10:00
vendor/lib/a2.rs           2026-10-01 10:00
vendor/lib/a3.rs           2026-10-01 10:00
vendor/lib/a4.rs           2026-10-01 10:00
vendor/lib/a5.rs           2026-10-01 10:00
vendor/lib/a6.rs           2026-10-01 10:00
```

In `tests/fixtures/huge/`: the `.rs` files are `let e = error;`, `docs/d1.md` is `an error here`. `tests/` holds
tests. Times are UTC.

## E104.1 Where the matches are and how to narrow the query

Six of the eleven files are in `vendor/`, three are not source, and `error` is searched without `/w`.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"sort": "path asc", "max_files": 1}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiZTI2ZmFmMWMiLCJmIjoxLCJ0IjpbMTEsMTEsMTFdfQ
#1 text has 'error'
funnel:
  text has 'error'                      11
hints:
  dir: vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
  lang: rust 10 · markdown 1
  narrow with: not path ~ 'vendor/**', kind = source, /w

docs/
d1.md  [#1]  lines: 1  matches: 1
1:an ⟦#1:error⟧ here
```

## E104.2 In JSON

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"sort": "path asc", "max_files": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "funnel": [ { "step": "text has 'error'", "files": 11 } ],
  "hints": [
    "dir: vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1",
    "lang: rust 10 · markdown 1",
    "narrow with: not path ~ 'vendor/**', kind = source, /w"
  ],
  "counts": { "files": 11, "lines": 11, "matches": 11 },
  "truncated": true,
  "truncated_by": ["max_files"],
  "next_cursor": "eyJ2IjoxLCJoIjoiMTAxZDRiMTUiLCJmIjoxLCJ0IjpbMTEsMTEsMTFdfQ",
  "legend": { "#1": "text has 'error'" },
  "files": [
    { "path": "docs/d1.md", "lines": 1, "matches": 1, "conditions": ["#1"],
      "blocks": [ { "lines": [ { "n": 1, "text": "an error here",
        "fragments": [ { "col": [4, 9], "text": "error", "ids": ["#1"] } ] } ] } ] }
  ]
}
```

## E104.3 Only the suggestions which narrow the query

With `/w` no `/w` is suggested.

Fixture: `huge`

```json request
{"query": "text has 'error' /w", "output": {"sort": "path asc", "max_files": 1, "detail": "files"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMjJkNzM2NjQiLCJmIjoxLCJ0IjpbMTEsMTEsMTFdfQ
#1 text has 'error' /w
funnel:
  text has 'error' /w                   11
hints:
  dir: vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
  lang: rust 10 · markdown 1
  narrow with: not path ~ 'vendor/**', kind = source

docs/
d1.md  [#1]  lines: 1  matches: 1
```

## E104.4 Not for a result which is not huge

With two files listed, eleven files are not more than 10 × `max_files`: the funnel has no hints.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"sort": "path asc", "max_files": 2, "detail": "files", "funnel": "always"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiZTdkMTY1OGMiLCJmIjoyLCJ0IjpbMTEsMTEsMTFdfQ
#1 text has 'error'
funnel:
  text has 'error'                      11

docs/
d1.md  [#1]  lines: 1  matches: 1
src/
b1.rs  [#1]  lines: 1  matches: 1
```

## E104.5 Not without the funnel

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"sort": "path asc", "max_files": 1, "detail": "files", "funnel": "never"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiYjliOTEyMjAiLCJmIjoxLCJ0IjpbMTEsMTEsMTFdfQ
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
```
