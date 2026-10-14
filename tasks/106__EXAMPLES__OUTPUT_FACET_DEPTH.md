# Examples for task 106

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `huge` of task 104:
eleven files with `error` — six in `vendor/lib/`, two in `src/`, two in `tests/`, `docs/d1.md`.

## E106.1 Two levels by default

Fixture: `huge`

1. `find`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"
detail = "lines"
count_by = "total"
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
repeated_locations = 3
sort = "relevance"
path_style = "relative"
omit = []
show = []
```

## E106.2 One level in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"], "facet_depth": 1}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/ 6 · src/ 2 · tests/ 2 · docs/ 1
```

## E106.3 Also the hints of a huge result

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"sort": "path asc", "max_files": 1, "detail": "files", "facet_depth": 1}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDVhMmY2NWUiLCJmIjoxLCJ0IjpbMTEsMTEsMTFdfQ
#1 text has 'error'
funnel:
  text has 'error'                      11
hints:
  dir: vendor/ 6 · src/ 2 · tests/ 2 · docs/ 1
  lang: rust 10 · markdown 1
  narrow with: not path ~ 'vendor/**', kind = source, /w

docs/
d1.md  [#1]  lines: 1  matches: 1
```

## E106.4 For the session

Fixture: `huge`

1. `update_config`

```json request update_config
{"set": "output.facet_depth = 1"}
```

```text response
[output]
facet_depth = 1    # was 2
```

2. `find`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/ 6 · src/ 2 · tests/ 2 · docs/ 1
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
count_by = "total"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 1    # update
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
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
repeated_locations = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
show = []    # default
```

## E106.5 An invalid value in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"facet_depth": 0}}
```

```text error
error[invalid_option]: 'output.facet_depth' must be a whole number of at least 1
```

## E106.6 An invalid value for `update_config`

Fixture: `huge`

```json request update_config
{"set": "output.facet_depth = 'deep'"}
```

```text error
error[invalid_value]: invalid value for 'output.facet_depth'
  hint: a whole number of at least 1 is expected
```

## E106.7 From the config file

Fixture: `huge`

```toml conf/o.toml
[output]
facet_depth = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/ 6 · src/ 2 · tests/ 2 · docs/ 1
```

## E106.8 An invalid value in the config file

Fixture: `huge`

```toml conf/o.toml
[output]
facet_depth = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.facet_depth': a whole number of at least 1 is expected
```
