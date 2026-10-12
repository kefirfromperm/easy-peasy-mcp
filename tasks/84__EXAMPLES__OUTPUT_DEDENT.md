# Examples for task 84

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### dedent

```text tree
config.rs                  2026-10-01 10:00
tab.rs                     2026-10-01 10:00
```

File contents are in `tests/fixtures/dedent/`, `config.rs`:

```text
impl Config {
    fn load() {
        let cfg = parse(&raw).unwrap();
        let port = cfg.port.unwrap();
    }
}
```

`tab.rs` is `let a = 1;` after four spaces and a tab. Times are UTC.

## E84.1 The common indentation is removed by default

Fixture: `dedent`

1. `find`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap()'

./
config.rs  [#1 #2]  lines: 2  matches: 2
⇥8
3:let cfg = parse(&raw).⟦#2:unwrap()⟧;
4:let port = cfg.port.⟦#2:unwrap()⟧;
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[output]
format = "text"
detail = "lines"
# budget =  (not set)
chars_per_token = 4
funnel = "auto"
facets = ['dir', 'lang', 'kind', 'condition']
facet_depth = 2
facet_limit = 8
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
max_output = 40000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
omit = []
```

## E84.2 Of the whole block, context lines too

Fixture: `dedent`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'", "output": {"context": 1}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap()'

./
config.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2-fn load() {
3:    let cfg = parse(&raw).⟦#2:unwrap()⟧;
4:    let port = cfg.port.⟦#2:unwrap()⟧;
5-}
```

## E84.3 Without dedent in a request

Fixture: `dedent`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'", "output": {"dedent": false}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap()'

./
config.rs  [#1 #2]  lines: 2  matches: 2
3:        let cfg = parse(&raw).⟦#2:unwrap()⟧;
4:        let port = cfg.port.⟦#2:unwrap()⟧;
```

## E84.4 In JSON the block has `indent`, `col` refers to the original line

Fixture: `dedent`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "name = 'config.rs'", "#2": "text has 'unwrap()'" },
  "files": [
    {
      "path": "config.rs",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1", "#2"],
      "blocks": [
        {
          "indent": 8,
          "lines": [
            { "n": 3, "text": "let cfg = parse(&raw).unwrap();",
              "fragments": [ { "col": [31, 39], "text": "unwrap()", "ids": ["#2"] } ] },
            { "n": 4, "text": "let port = cfg.port.unwrap();",
              "fragments": [ { "col": [29, 37], "text": "unwrap()", "ids": ["#2"] } ] }
          ]
        }
      ]
    }
  ]
}
```

## E84.5 A match which starts in the indentation keeps it

Fixture: `dedent`

```json request
{"query": "name = 'tab.rs' and text =~ `^\\s+let`"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'tab.rs'
#2 text =~ `^\s+let`

./
tab.rs  [#1 #2]  lines: 1  matches: 1
1:⟦#2:    	let⟧ a = 1;
```

## E84.6 Without dedent for the session

Fixture: `dedent`

1. `update_config`

```json request update_config
{"set": "output.dedent = false"}
```

```text response
[output]
dedent = false    # was true
```

2. `find`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap()'

./
config.rs  [#1 #2]  lines: 2  matches: 2
3:        let cfg = parse(&raw).⟦#2:unwrap()⟧;
4:        let port = cfg.port.⟦#2:unwrap()⟧;
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
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
max_output = 40000    # default
dedent = false    # update
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
```

## E84.7 An invalid value in a request

Fixture: `dedent`

```json request
{"output": {"dedent": 4}}
```

```text error
error[invalid_option]: 'output.dedent' must be true or false
```

## E84.8 An invalid value for `update_config`

Fixture: `dedent`

```json request update_config
{"set": "output.dedent = \"yes\""}
```

```text error
error[invalid_value]: invalid value for 'output.dedent'
  hint: true or false is expected
```

## E84.9 From the config file

Fixture: `dedent`

```toml conf/o.toml
[output]
dedent = false
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
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
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
max_output = 40000    # default
dedent = false    # file: conf/o.toml
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
```

2. `find`

```json request
{"query": "name = 'config.rs' and text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap()'

./
config.rs  [#1 #2]  lines: 2  matches: 2
3:        let cfg = parse(&raw).⟦#2:unwrap()⟧;
4:        let port = cfg.port.⟦#2:unwrap()⟧;
```

## E84.10 An invalid value in the config file

Fixture: `dedent`

```toml conf/o.toml
[output]
dedent = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.dedent': true or false is expected
```
