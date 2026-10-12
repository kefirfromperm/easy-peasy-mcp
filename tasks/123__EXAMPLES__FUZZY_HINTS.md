# Examples for task 123

A JSON response is compared as a JSON value. The scores are those of the fuzzy matching of QUERY.md#--fuzzy, rounded
down to two decimals.

## Fixtures

### fuzzy-hints

```text tree
README.md                  2026-10-01 10:00
docs/parsing.md            2026-10-01 10:00
src/parse.rs               2026-10-01 10:00
src/query/lexer.rs         2026-10-01 10:00
src/query/parser.rs        2026-10-01 10:00
```

In `tests/fixtures/fuzzy-hints/`. The raw scores: `path ~~ 'qryparser'` matches `src/query/parser.rs` with 0.708;
`name ~~ 'rsr'` matches `parse.rs` with 0.554 and `parser.rs` with 0.549, `path ~~ 'rsr'` matches `src/parse.rs` with
0.538 and `src/query/parser.rs` with 0.527; `name ~~ 'e'` matches `lexer.rs` and `parse.rs` with 0.0825, `parser.rs`
and `README.md` with 0.0822.

## E123.1 `path ~~` instead of `name ~~`

The name `parser.rs` has no `q`: the agent had the directory in mind. `path ~~` finds the file.

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'qryparser'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~~ 'qryparser'                    0
hints:
  path ~~ 'qryparser'                    1 file: src/query/parser.rs
```

## E123.2 `name ~~` below the threshold

Two names match with a score below 0.6, best first; `path ~~ 'rsr'` scores lower and is no hint.

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'rsr'", "query_options": {"fuzzy_min_score": 0.6}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~~ 'rsr'                          0
hints:
  name ~~ 'rsr': below query_options.fuzzy_min_score 0.6: parse.rs 0.55, parser.rs 0.54
```

## E123.3 `path ~~` below the threshold

The paths of the files are shown.

Fixture: `fuzzy-hints`

```json request
{"query": "path ~~ 'rsr'", "query_options": {"fuzzy_min_score": 0.6}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  path ~~ 'rsr'                          0
hints:
  path ~~ 'rsr': below query_options.fuzzy_min_score 0.6: src/parse.rs 0.53, src/query/parser.rs 0.52
```

## E123.4 At most three, equal scores in path order

Four files match `name ~~ 'e'` with a score below the default 0.5. `lexer.rs` and `parse.rs` score the same
and come in path order, then `README.md` before `src/query/parser.rs`; the fourth is left out.

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'e'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~~ 'e'                            0
hints:
  name ~~ 'e': below query_options.fuzzy_min_score 0.5: parse.rs 0.08, lexer.rs 0.08, README.md 0.08
```

## E123.5 On the files of the steps before

Only `src/query/parser.rs` passes `path ~ 'src/query/**'` and matches `name ~~ 'rsr'`.

Fixture: `fuzzy-hints`

```json request
{"query": "path ~ 'src/query/**' and name ~~ 'rsr'", "query_options": {"fuzzy_min_score": 0.6}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  path ~ 'src/query/**'                  2
  and name ~~ 'rsr'                      0
hints:
  name ~~ 'rsr': below query_options.fuzzy_min_score 0.6: parser.rs 0.54
```

## E123.6 No hint

No file has the characters `zzz`: the funnel comes without hints.

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'zzz'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~~ 'zzz'                          0
```

## E123.7 The branches of `or`

Every branch gets its hints, in the order of the branches.

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'qryparser' or name ~~ 'rsr'", "query_options": {"fuzzy_min_score": 0.6}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~~ 'qryparser'                    0
  or name ~~ 'rsr'                       0
hints:
  path ~~ 'qryparser'                    1 file: src/query/parser.rs
  name ~~ 'rsr': below query_options.fuzzy_min_score 0.6: parse.rs 0.55, parser.rs 0.54
```

## E123.8 In JSON

Fixture: `fuzzy-hints`

```json request
{"query": "name ~~ 'qryparser' or name ~~ 'rsr'", "query_options": {"fuzzy_min_score": 0.6}, "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": false,
  "result": "r1",
  "funnel": [
    { "step": "name ~~ 'qryparser'", "files": 0 },
    { "step": "or name ~~ 'rsr'", "files": 0 }
  ],
  "hints": [
    "path ~~ 'qryparser'  1 file: src/query/parser.rs",
    "name ~~ 'rsr': below query_options.fuzzy_min_score 0.6: parse.rs 0.55, parser.rs 0.54"
  ],
  "counts": { "files": 0, "lines": 0, "matches": 0 }
}
```
