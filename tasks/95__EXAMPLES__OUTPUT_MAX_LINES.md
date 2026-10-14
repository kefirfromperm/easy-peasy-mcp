# Examples for task 95

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `detail` of task 79: `src/a.rs` (`// TODO one`, `// TODO four`) and `src/b.rs`
(`// TODO two`, `// TODO three`), and set `sort = "path asc"`. The cursor of the next page comes with task 101.

## E95.1 500 lines by default

Fixture: `detail`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
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
sort = "relevance"
path_style = "relative"
omit = []
```

## E95.2 The first lines

The lines of the next files are not listed; the files are, with their counts.

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 3}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiMGQ1OTAxOTAiLCJmIjoxLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
```

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 1}}
```

```text response
files: 2  lines: 4  matches: 4  result: r2  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiYTA1MmU1NTIiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
b.rs  [#1]  lines: 2  matches: 2
```

## E95.3 In JSON

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 2, "lines": 4, "matches": 4 },
  "truncated": true,
  "truncated_by": ["max_lines"],
  "next_cursor": "eyJ2IjoxLCJoIjoiNjZjNGJiMDEiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ",
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "src/a.rs", "lines": 2, "matches": 2, "conditions": ["#1"],
      "blocks": [ { "lines": [ { "n": 1, "text": "// TODO one",
        "fragments": [ { "col": [4, 8], "text": "TODO", "ids": ["#1"] } ] } ] } ] },
    { "path": "src/b.rs", "lines": 2, "matches": 2, "conditions": ["#1"] }
  ]
}
```

## E95.4 Not hit, and not in other detail levels

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 4}}
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

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 1, "detail": "files"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r2
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
b.rs  [#1]  lines: 2  matches: 2
```

## E95.5 For the session

Fixture: `detail`

1. `update_config`

```json request update_config
{"set": "output.max_lines = 2"}
```

```text response
[output]
max_lines = 2    # was 500
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path desc"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiYzQ0ZjA3MGIiLCJmIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
2:// ⟦#1:TODO⟧ three
a.rs  [#1]  lines: 2  matches: 2
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
count_by = "total"    # default
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
max_lines = 2    # update
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
omit = []    # default
```

## E95.6 An invalid value in a request

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"max_lines": 0}}
```

```text error
error[invalid_option]: 'output.max_lines' must be a whole number of at least 1
```

## E95.7 An invalid value for `update_config`

Fixture: `detail`

```json request update_config
{"set": "output.max_lines = 'all'"}
```

```text error
error[invalid_value]: invalid value for 'output.max_lines'
  hint: a whole number of at least 1 is expected
```

## E95.8 From the config file

Fixture: `detail`

```toml conf/o.toml
[output]
max_lines = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiMTBjM2VhN2QiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
b.rs  [#1]  lines: 2  matches: 2
```

## E95.9 An invalid value in the config file

Fixture: `detail`

```toml conf/o.toml
[output]
max_lines = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_lines': a whole number of at least 1 is expected
```
