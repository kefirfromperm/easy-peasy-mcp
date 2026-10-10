# Examples for task 83

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### blank

```text tree
a.rs                       2026-10-01 10:00
```

File contents are in `tests/fixtures/blank/`: `a.rs` has `fn a() {`, an empty line, `let x = foo.unwrap();`, a
line of three spaces and `}`. Times are UTC.

## E83.1 Blank context lines are skipped by default

The line numbers show the gaps.

Fixture: `blank`

1. `find`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 2}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap()'

./
a.rs  [#1]  lines: 1  matches: 1
1-fn a() {
3:let x = foo.⟦#1:unwrap()⟧;
5-}
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
```

## E83.2 Blank context lines kept in a request

Fixture: `blank`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 2, "blank_context": true}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap()'

./
a.rs  [#1]  lines: 1  matches: 1
1-fn a() {
2-
3:let x = foo.⟦#1:unwrap()⟧;
4-   
5-}
```

## E83.3 In JSON

Fixture: `blank`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 2, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "text has 'unwrap()'" },
  "files": [
    {
      "path": "a.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 1, "text": "fn a() {" },
          { "n": 3, "text": "let x = foo.unwrap();",
            "fragments": [ { "col": [13, 21], "text": "unwrap()", "ids": ["#1"] } ] },
          { "n": 5, "text": "}" }
        ] }
      ]
    }
  ]
}
```

## E83.4 Blank context lines for the session

Fixture: `blank`

1. `update_config`

```json request update_config
{"set": "output.blank_context = true"}
```

```text response
[output]
blank_context = true    # was false
```

2. `find`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 2}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap()'

./
a.rs  [#1]  lines: 1  matches: 1
1-fn a() {
2-
3:let x = foo.⟦#1:unwrap()⟧;
4-   
5-}
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
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = true    # update
group_dirs = true    # default
group_repeated = 3    # default
```

## E83.5 An invalid value in a request

Fixture: `blank`

```json request
{"output": {"blank_context": "yes"}}
```

```text error
error[invalid_option]: 'output.blank_context' must be true or false
```

## E83.6 An invalid value for `update_config`

Fixture: `blank`

```json request update_config
{"set": "output.blank_context = 1"}
```

```text error
error[invalid_value]: invalid value for 'output.blank_context'
  hint: true or false is expected
```

## E83.7 From the config file

Fixture: `blank`

```toml conf/o.toml
[output]
blank_context = true
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
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = true    # file: conf/o.toml
group_dirs = true    # default
group_repeated = 3    # default
```

2. `find`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 2}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap()'

./
a.rs  [#1]  lines: 1  matches: 1
1-fn a() {
2-
3:let x = foo.⟦#1:unwrap()⟧;
4-   
5-}
```

## E83.8 An invalid value in the config file

Fixture: `blank`

```toml conf/o.toml
[output]
blank_context = "no"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.blank_context': true or false is expected
```
