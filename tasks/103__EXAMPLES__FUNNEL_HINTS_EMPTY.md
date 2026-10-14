# Examples for task 103

Spec: `1`

A JSON response is compared as a JSON value. The hints of huge results come with task 104.

## Fixtures

### hints

```text tree
big.txt                    2026-10-01 10:00
config.yaml                2026-10-01 10:00
data.bin                   2026-10-01 10:00
gen.rs                     2026-10-01 10:00
src/parser/mod.rs          2026-10-01 10:00
src/query.rs               2026-10-01 10:00
vendor/v.rs                2026-10-01 10:00
```

In `tests/fixtures/hints/`: `src/parser/mod.rs` is `fn parse() { ParseQuery::new() }`, `src/query.rs`
`fn parse_query() {}`, `config.yaml` `key: value`; `parseQuery` is in `vendor/v.rs` (ignored by `.ignore`), in
`gen.rs` (generated: `// @generated` on its first line), in `big.txt` (1512 bytes) and in `data.bin` (binary). Times
are UTC.

## E103.1 Other case, other naming style, skipped files

The warnings of the search are written after the hints.

Fixture: `hints`

```json request
{"query": "text has 'parseQuery'", "traversal": {"max_file_size": "1K"}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  text has 'parseQuery'                  0
hints:
  text has 'parseQuery' /i               1 file: ParseQuery
  text has 'parseQuery' /n               2 files: ParseQuery, parse_query
  skipped: 1 ignored (traversal.ignore_files), 1 generated (traversal.skip_generated), 1 too big (traversal.max_file_size), 1 binary (traversal.binary)

warning: too_big (1): big.txt
warning: binary_skipped (1): data.bin
```

## E103.2 On the files of the steps before

The variations are evaluated on the Rust files; the skipped files which could match are Rust files too.

Fixture: `hints`

```json request
{"query": "lang = rust and text has 'parseQuery'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  lang = rust                            2
  and text has 'parseQuery'              0
hints:
  text has 'parseQuery' /i               1 file: ParseQuery
  text has 'parseQuery' /n               2 files: ParseQuery, parse_query
  skipped: 1 ignored (traversal.ignore_files), 1 generated (traversal.skip_generated)
```

## E103.3 Without `/w`

`Query` is in `src/parser/mod.rs` and in `big.txt`; `data.bin` is not searched.

Fixture: `hints`

```json request
{"query": "text has 'Query' /w"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  text has 'Query' /w                    0
hints:
  text has 'Query'                       2 files: Query

warning: binary_skipped (1): data.bin
```

## E103.4 `has` instead of `=`

Fixture: `hints`

```json request
{"query": "name = 'query'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name = 'query'                         0
hints:
  name has 'query'                       1 file: query.rs
```

## E103.5 Similar names

Fixture: `hints`

```json request
{"query": "name = 'config.yml'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name = 'config.yml'                    0
hints:
  name = 'config.yml': similar: config.yaml
```

## E103.6 A missing directory

Fixture: `hints`

```json request
{"query": "path starts 'src/parsers/'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  path starts 'src/parsers/'             0
hints:
  path starts 'src/parsers/': no such directory, similar: src/parser/
```

## E103.7 In JSON

Fixture: `hints`

```json request
{"query": "lang = rust and text has 'parseQuery'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": false,
  "result": "r1",
  "funnel": [
    { "step": "lang = rust", "files": 2 },
    { "step": "and text has 'parseQuery'", "files": 0 }
  ],
  "hints": [
    "text has 'parseQuery' /i  1 file: ParseQuery",
    "text has 'parseQuery' /n  2 files: ParseQuery, parse_query",
    "skipped: 1 ignored (traversal.ignore_files), 1 generated (traversal.skip_generated)"
  ],
  "counts": { "files": 0, "lines": 0, "matches": 0 }
}
```

## E103.8 Not without the funnel

Fixture: `hints`

```json request
{"query": "lang = rust and text has 'parseQuery'", "output": {"funnel": "never"}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
```

## E103.9 Only for an empty result

Fixture: `hints`

```json request
{"query": "text has 'ParseQuery'", "output": {"funnel": "always"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'ParseQuery'
funnel:
  text has 'ParseQuery'                  1

src/parser/
mod.rs  [#1]  lines: 1  matches: 1
1:fn parse() { ⟦#1:ParseQuery⟧::new() }

warning: binary_skipped (1): data.bin
```
