# Examples for task 80

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### context

```text tree
src/config.rs              2026-10-01 10:00
```

File contents are in `tests/fixtures/context/`, `src/config.rs`:

```text
fn load() {
let raw = read(path)?;
let cfg = parse(&raw).unwrap();
let port = cfg.port;
Ok(cfg)
}
fn save() {
let out = render();
write(out).unwrap();
}
```

Times are UTC.

## E80.1 No context by default

Fixture: `context`

1. `find`

```json request
{"query": "text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
--
9:write(out).⟦#1:unwrap()⟧;
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
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 100
max_lines = 500
max_matches = 1000
max_matches_per_file = 50
max_line_length = 300
max_output_per_file = 4000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E80.2 One line of context in a request

Context lines are `N-`, they are not counted; `--` separates the blocks.

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 1}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
--
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E80.3 Blocks whose context overlaps are merged

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 3}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
1-fn load() {
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
5-Ok(cfg)
6-}
7-fn save() {
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E80.4 Context in JSON

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has 'unwrap()'" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 2, "text": "let raw = read(path)?;" },
          { "n": 3, "text": "let cfg = parse(&raw).unwrap();",
            "fragments": [ { "col": [23, 31], "text": "unwrap()", "ids": ["#1"] } ] },
          { "n": 4, "text": "let port = cfg.port;" }
        ] },
        { "lines": [
          { "n": 8, "text": "let out = render();" },
          { "n": 9, "text": "write(out).unwrap();",
            "fragments": [ { "col": [12, 20], "text": "unwrap()", "ids": ["#1"] } ] },
          { "n": 10, "text": "}" }
        ] }
      ]
    }
  ]
}
```

## E80.5 Context for the session

Fixture: `context`

1. `update_config`

```json request update_config
{"set": "output.context = 1"}
```

```text response
[output]
context = 1    # was 0
```

2. `find`

```json request
{"query": "text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
--
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
10-}
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
meta = false    # default
context = 1    # update
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E80.6 An invalid value in a request

Fixture: `context`

```json request
{"output": {"context": -1}}
```

```text error
error[invalid_option]: 'output.context' must be a whole number
```

## E80.7 An invalid value for `update_config`

Fixture: `context`

```json request update_config
{"set": "output.context = \"2\""}
```

```text error
error[invalid_value]: invalid value for 'output.context'
  hint: a whole number is expected
```

## E80.8 From the config file

Fixture: `context`

```toml conf/o.toml
[output]
context = 1
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
meta = false    # default
context = 1    # file: conf/o.toml
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

2. `find`

```json request
{"query": "text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
--
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E80.9 An invalid value in the config file

Fixture: `context`

```toml conf/o.toml
[output]
context = 1.5
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.context': a whole number is expected
```
