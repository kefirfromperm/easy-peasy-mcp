# Examples for task 87

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. Some
examples use the fixture `repeated` of task 86.

## Fixtures

### highlight

```text tree
src/config.rs              2026-10-01 10:00
```

`src/config.rs` (in `tests/fixtures/highlight/`):

```
fn load(raw: &str) -> Config {
    let cfg: Config = toml::from_str(raw).unwrap();
    cfg
}
```

## E87.1 Markers in text by default

Fixture: `highlight`

1. `find`

```json request
{"query": "text has 'Config' and text has 'unwrap('"}
```

```text response
files: 1  lines: 2  matches: 3  result: r1
#1 text has 'Config'
#2 text has 'unwrap('

src/
config.rs  [#1 #2]  lines: 2  matches: 3
1:fn load(raw: &str) -> ⟦#1:Config⟧ {
2:    let cfg: ⟦#1:Config⟧ = toml::from_str(raw).⟦#2:unwrap(⟧);
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
detail = "lines"
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
path_style = "relative"
```

## E87.2 Columns in JSON by default

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "text has 'unwrap('" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        { "indent": 4,
          "lines": [ { "n": 2, "text": "let cfg: Config = toml::from_str(raw).unwrap();",
            "fragments": [ { "col": [43, 50], "text": "unwrap(", "ids": ["#1"] } ] } ] }
      ]
    }
  ]
}
```

## E87.3 Columns in text

Every matched line is followed by the columns of its fragments.

Fixture: `highlight`

```json request
{"query": "text has 'Config' and text has 'unwrap('", "output": {"highlight": "columns"}}
```

```text response
files: 1  lines: 2  matches: 3  result: r1
#1 text has 'Config'
#2 text has 'unwrap('

src/
config.rs  [#1 #2]  lines: 2  matches: 3
1:fn load(raw: &str) -> Config {
  @ 23..29 #1
2:    let cfg: Config = toml::from_str(raw).unwrap();
  @ 14..20 #1  @ 43..50 #2
```

## E87.4 Columns of the original line after dedent

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"highlight": "columns", "context": 1}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
1-fn load(raw: &str) -> Config {
2:    let cfg: Config = toml::from_str(raw).unwrap();
  @ 43..50 #1
3-    cfg
```

The block with its context has no common indentation; without context it is dedented and the columns don't change (the second request of the session):

```json request
{"query": "text has 'unwrap('", "output": {"highlight": "columns"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).unwrap();
  @ 43..50 #1
```

## E87.5 Columns of a repeated line

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"highlight": "columns"}}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

repeated:
  ×3 use anyhow::Result;    src/a.rs:1, src/b.rs:1, src/c.rs:1
  @ 13..19 #1
src/
a.rs  [#1]  lines: 2  matches: 2
2:fn a() -> Result<()> { todo!() }
  @ 11..17 #1
```

## E87.6 No highlight in text

Fixture: `highlight`

```json request
{"query": "text has 'Config' and text has 'unwrap('", "output": {"highlight": "none"}}
```

```text response
files: 1  lines: 2  matches: 3  result: r1
#1 text has 'Config'
#2 text has 'unwrap('

src/
config.rs  [#1 #2]  lines: 2  matches: 3
1:fn load(raw: &str) -> Config {
2:    let cfg: Config = toml::from_str(raw).unwrap();
```

## E87.7 No highlight in JSON

Matched lines get `"match": true`, context lines don't.

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"format": "json", "highlight": "none", "after": 1}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "text has 'unwrap('" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        { "indent": 4,
          "lines": [
            { "n": 2, "text": "let cfg: Config = toml::from_str(raw).unwrap();", "match": true },
            { "n": 3, "text": "cfg" }
          ] }
      ]
    }
  ]
}
```

## E87.8 Markers in JSON

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"format": "json", "highlight": "markers"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "repeated": [
    { "count": 3, "text": "use anyhow::⟦#1:Result⟧;",
      "locations": ["src/a.rs:1", "src/b.rs:1", "src/c.rs:1"] }
  ],
  "counts": { "files": 3, "lines": 4, "matches": 4 },
  "legend": { "#1": "text has 'Result'" },
  "files": [
    {
      "path": "src/a.rs",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [ { "lines": [ { "n": 2, "text": "fn a() -> ⟦#1:Result⟧<()> { todo!() }" } ] } ]
    }
  ]
}
```

## E87.9 For the session

Fixture: `highlight`

1. `update_config`

```json request update_config
{"set": "output.highlight = \"columns\""}
```

```text response
[output]
highlight = "columns"    # was "auto"
```

2. `find`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).unwrap();
  @ 43..50 #1
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "columns"    # update
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
path_style = "relative"    # default
```

## E87.10 An invalid value in a request

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"highlight": "bold"}}
```

```text error
error[invalid_option]: 'output.highlight' must be one of: auto, none, columns, markers
```

## E87.11 An invalid value for `update_config`

Fixture: `highlight`

```json request update_config
{"set": "output.highlight = 1"}
```

```text error
error[invalid_value]: invalid value for 'output.highlight'
  hint: expected one of: auto, none, columns, markers
```

## E87.12 From the config file

Fixture: `highlight`

```toml conf/o.toml
[output]
highlight = "none"
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
format = "text"    # default
detail = "lines"    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "none"    # file: conf/o.toml
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
path_style = "relative"    # default
```

2. `find`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).unwrap();
```

## E87.13 An invalid value in the config file

Fixture: `highlight`

```toml conf/o.toml
[output]
highlight = "bold"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.highlight': expected one of: auto, none, columns, markers
```
