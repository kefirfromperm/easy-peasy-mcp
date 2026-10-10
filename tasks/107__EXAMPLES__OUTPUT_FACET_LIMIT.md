# Examples for task 107

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `huge` of task 104: eleven files with `error` — six in `vendor/lib/`, two in `src/`, two in
`tests/`, `docs/d1.md`.

## E107.1 8 values by default

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
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

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
```

## E107.2 Fewer values in a request

The two values over the limit have three files.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir", "kind"], "facet_limit": 2}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/lib/ 6 · src/ 2 · … 2 more (3)
kind:       source 8 · test 2 · … 1 more (1)
```

## E107.3 In JSON

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"], "facet_limit": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "facets": { "dir": { "vendor/lib/": 6, "… 3 more": 5 } },
  "counts": { "files": 11, "lines": 11, "matches": 11 },
  "legend": { "#1": "text has 'error'" }
}
```

## E107.4 For the session

Fixture: `huge`

1. `update_config`

```json request update_config
{"set": "output.facet_limit = 3"}
```

```text response
[output]
facet_limit = 3    # was 8
```

2. `find`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["dir"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/lib/ 6 · src/ 2 · tests/ 2 · … 1 more (1)
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
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 2    # default
facet_limit = 3    # update
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
sort = "relevance"    # default
path_style = "relative"    # default
```

## E107.5 An invalid value in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"facet_limit": 0}}
```

```text error
error[invalid_option]: 'output.facet_limit' must be a whole number of at least 1
```

## E107.6 An invalid value for `update_config`

Fixture: `huge`

```json request update_config
{"set": "output.facet_limit = -8"}
```

```text error
error[invalid_value]: invalid value for 'output.facet_limit'
  hint: a whole number of at least 1 is expected
```

## E107.7 From the config file

Fixture: `huge`

```toml conf/o.toml
[output]
facet_limit = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["lang"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
lang:       rust 10 · … 1 more (1)
```

## E107.8 An invalid value in the config file

Fixture: `huge`

```toml conf/o.toml
[output]
facet_limit = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.facet_limit': a whole number of at least 1 is expected
```
