# Examples for task 79

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### detail

```text tree
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
```

File contents are in `tests/fixtures/detail/`: every file has two lines with `TODO`. Times are UTC.

## E79.1 Lines by default

Fixture: `detail`

1. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
2:// ⟦#1:TODO⟧ three
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

## E79.2 Does anything match?

Fixture: `detail`

1. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "exists"}}
```

```text response
exists: yes
```

2. `find`

```json request
{"query": "text has 'FIXME'", "output": {"detail": "exists"}}
```

```text response
exists: no
```

## E79.3 How many?

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"detail": "count"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
```

## E79.4 Which files

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
b.rs  [#1]  lines: 2  matches: 2
```

## E79.5 The levels in JSON

Fixture: `detail`

1. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "exists", "format": "json"}}
```

```json response
{ "version": 1, "exists": true }
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "count", "format": "json"}}
```

```json response
{ "version": 1, "exists": true, "result": "r1", "counts": { "files": 2, "lines": 4, "matches": 4 } }
```

3. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": { "files": 2, "lines": 4, "matches": 4 },
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "src/a.rs", "lines": 2, "matches": 2, "conditions": ["#1"] },
    { "path": "src/b.rs", "lines": 2, "matches": 2, "conditions": ["#1"] }
  ]
}
```

## E79.6 A detail for the session

Fixture: `detail`

1. `update_config`

```json request update_config
{"set": "output.detail = \"count\""}
```

```text response
[output]
detail = "count"    # was "lines"
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
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
detail = "count"    # update
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
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E79.7 An invalid value in a request

Fixture: `detail`

```json request
{"output": {"detail": "all"}}
```

```text error
error[invalid_option]: 'output.detail' must be one of: exists, count, facets, files, lines
```

## E79.8 An invalid value for `update_config`

Fixture: `detail`

```json request update_config
{"set": "output.detail = \"all\""}
```

```text error
error[invalid_value]: invalid value for 'output.detail'
  hint: expected one of: exists, count, facets, files, lines
```

## E79.9 From the config file

Fixture: `detail`

```toml conf/o.toml
[output]
detail = "files"
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
detail = "files"    # file: conf/o.toml
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
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
b.rs  [#1]  lines: 2  matches: 2
```

## E79.10 An invalid value in the config file

Fixture: `detail`

```toml conf/o.toml
[output]
detail = "everything"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.detail': expected one of: exists, count, facets, files, lines
```
