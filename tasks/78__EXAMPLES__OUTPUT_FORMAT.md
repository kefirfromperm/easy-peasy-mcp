# Examples for task 78

Examples with a command line are startup examples (AGENTS.md); `Slow reads` is the test hook of AGENTS.md. A JSON
response is compared as a JSON value: the blocks are indented for reading, the server writes the document without
whitespace.

## Fixtures

### json-out

```text tree
src/config.rs              2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/json-out/`, `src/config.rs`:

```text
fn load() {
let raw = read(path)?;
let cfg = parse(&raw).unwrap();
}
```

`src/main.rs` is `fn main() {}`. Times are UTC.

## E78.1 Text by default

Fixture: `json-out`

1. `find`

```json request
{"query": "name = 'config.rs' and text has 'unwrap('"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap('

src/
config.rs  [#1 #2]  lines: 1  matches: 1
3:let cfg = parse(&raw).⟦#2:unwrap(⟧);
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
```

## E78.2 JSON in a request

The fragments have their columns in characters, the end exclusive (`highlight = auto` is `columns` in JSON).

Fixture: `json-out`

```json request
{"query": "name = 'config.rs' and text has 'unwrap('", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "name = 'config.rs'", "#2": "text has 'unwrap('" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        {
          "lines": [
            { "n": 3, "text": "let cfg = parse(&raw).unwrap();",
              "fragments": [ { "col": [23, 30], "text": "unwrap(", "ids": ["#2"] } ] }
          ]
        }
      ]
    }
  ]
}
```

## E78.3 Files without content, paths in full

Fixture: `json-out`

```json request
{"query": "type = dir or name ~ '*.rs'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 3, "lines": 0, "matches": 0 },
  "legend": { "#1": "type = dir", "#2": "name ~ '*.rs'" },
  "files": [
    { "path": "src/", "conditions": ["#1"] },
    { "path": "src/config.rs", "conditions": ["#2"] },
    { "path": "src/main.rs", "conditions": ["#2"] }
  ]
}
```

## E78.4 A multiline match

The fragment is in its first line, `col` ends at the end of that line, `to` is its end.

Fixture: `json-out`

1. `find`

```json request
{"query": "text =~ `unwrap\\(\\);\\n\\}` /m", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 1 },
  "legend": { "#1": "text =~ `unwrap\\(\\);\\n\\}` /m" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 2,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        {
          "lines": [
            { "n": 3, "text": "let cfg = parse(&raw).unwrap();",
              "fragments": [ { "col": [23, 32], "text": "unwrap();\n}", "ids": ["#1"], "to": { "n": 4, "col": 2 } } ] },
            { "n": 4, "text": "}" }
          ]
        }
      ]
    }
  ]
}
```

2. `find`

The same in text.

```json request
{"query": "text =~ `unwrap\\(\\);\\n\\}` /m"}
```

```text response
files: 1  lines: 2  matches: 1  result: r2
#1 text =~ `unwrap\(\);\n\}` /m

src/
config.rs  [#1]  lines: 2  matches: 1
3:let cfg = parse(&raw).⟦#1:unwrap();
4:}⟧
```

## E78.5 Escape levels

Fixture: `levels`

```json request
{"query": "text has `\"role\": \"admin\"` /e(backslash, 0..1)", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has `\"role\": \"admin\"` /e(backslash, 0..1)" },
  "files": [
    {
      "path": "nested.json",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        {
          "lines": [
            { "n": 1, "text": "{\"role\": \"admin\"}",
              "fragments": [ { "col": [2, 17], "text": "\"role\": \"admin\"", "ids": ["#1"], "escape": 0 } ] },
            { "n": 2, "text": "\"{\\\"role\\\": \\\"admin\\\"}\"",
              "fragments": [ { "col": [3, 22], "text": "\\\"role\\\": \\\"admin\\\"", "ids": ["#1"], "escape": 1 } ] }
          ]
        }
      ]
    }
  ]
}
```

## E78.6 The encoding of a file and warnings

Fixture: `encodings`

```json request
{"query": "name ~ '{utf16le.txt,invalid.txt}' and text has 'hello'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 2, "lines": 2, "matches": 2 },
  "legend": { "#1": "name ~ '{utf16le.txt,invalid.txt}'", "#2": "text has 'hello'" },
  "warnings": [ { "code": "decode_errors", "count": 1, "paths": ["enc/invalid.txt"] } ],
  "files": [
    {
      "path": "enc/invalid.txt",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        { "lines": [ { "n": 1, "text": "hello from invalid � bytes",
                       "fragments": [ { "col": [1, 6], "text": "hello", "ids": ["#2"] } ] } ] }
      ]
    },
    {
      "path": "enc/utf16le.txt",
      "encoding": "utf-16le",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        { "lines": [ { "n": 1, "text": "hello from utf-16le",
                       "fragments": [ { "col": [1, 6], "text": "hello", "ids": ["#2"] } ] } ] }
      ]
    }
  ]
}
```

## E78.7 The timeout

Fixture: `timeout`

Slow reads: `2s`

```json request
{"timeout": "200ms", "query": "type = dir or text has 'x'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "counts_exact": false,
  "truncated": true,
  "truncated_by": ["timeout"],
  "legend": { "#1": "type = dir", "#2": "text has 'x'" },
  "warnings": [ { "code": "timeout" } ],
  "files": [ { "path": "sub/", "conditions": ["#1"] } ]
}
```

## E78.8 An error in JSON

`position.col` is 1-based, in characters from the start of the query.

Fixture: `json-out`

```json request
{"query": "name ~ '*.rs' and text contains 'x'", "output": {"format": "json"}}
```

```json error
{
  "error": {
    "code": "unknown_operator",
    "message": "unknown operator 'contains'",
    "query": "name ~ '*.rs' and text contains 'x'",
    "position": { "col": 24, "len": 8 },
    "hint": "did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~"
  }
}
```

## E78.9 JSON for the session

Request errors are JSON too.

Fixture: `json-out`

1. `update_config`

```json request update_config
{"set": "output.format = \"json\""}
```

```text response
[output]
format = "json"    # was "text"
```

2. `find`

```json request
{"query": "name = 'main.rs'"}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "legend": { "#1": "name = 'main.rs'" },
  "files": [ { "path": "src/main.rs", "conditions": ["#1"] } ]
}
```

3. `find`

```json request
{"dirs": ["lib"], "query": "name = 'main.rs'"}
```

```json error
{ "error": { "code": "dir_not_found", "message": "the working directory 'lib' doesn't exist" } }
```

4. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "json"    # update
```

## E78.10 Text in a request when the session is JSON

Fixture: `json-out`

1. `update_config`

```json request update_config
{"set": "output.format = \"json\""}
```

```text response
[output]
format = "json"    # was "text"
```

2. `find`

```json request
{"query": "name = 'main.rs'", "output": {"format": "text"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E78.11 An invalid value in a request

The format is unknown, so the error is text.

Fixture: `json-out`

```json request
{"output": {"format": "xml"}}
```

```text error
error[invalid_option]: 'output.format' must be one of: text, json
```

## E78.12 An invalid value for `update_config`

Fixture: `json-out`

```json request update_config
{"set": "output.format = \"xml\""}
```

```text error
error[invalid_value]: invalid value for 'output.format'
  hint: expected one of: text, json
```

## E78.13 From the config file

Fixture: `json-out`

```toml conf/o.toml
[output]
format = "json"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

1. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "json"    # file: conf/o.toml
```

2. `find`

```json request
{"query": "name = 'main.rs'"}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "legend": { "#1": "name = 'main.rs'" },
  "files": [ { "path": "src/main.rs", "conditions": ["#1"] } ]
}
```

## E78.14 An invalid value in the config file

Fixture: `json-out`

```toml conf/o.toml
[output]
format = "yaml"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.format': expected one of: text, json
```
