# Examples for task 97

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `sorting` of task 92: `a.txt` (`TODO`), `b.txt` (`TODO TODO TODO`), `c.txt` and `d/e.txt`
(`TODO TODO`), and set `sort = "path asc"`.

## E97.1 50 matches per file by default

Fixture: `sorting`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
detail = "lines"
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

## E97.2 The first matches of a file

The file is marked `(cut)`, the next files are listed as usual.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches_per_file": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches_per_file)
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
1:⟦#1:TODO⟧
b.txt  [#1]  lines: 1  matches: 3  (cut)
1:⟦#1:TODO⟧ ⟦#1:TODO⟧ TODO
c.txt  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ ⟦#1:TODO⟧
d/
e.txt  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ ⟦#1:TODO⟧
```

## E97.3 In JSON

Fixture: `sorting`

```json request
{"query": "text has 'TODO' and name = 'b.txt'", "output": {"max_matches_per_file": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 3 },
  "truncated": true,
  "truncated_by": ["max_matches_per_file"],
  "legend": { "#1": "text has 'TODO'", "#2": "name = 'b.txt'" },
  "files": [
    { "path": "b.txt", "truncated": true, "lines": 1, "matches": 3, "conditions": ["#1", "#2"],
      "blocks": [ { "lines": [ { "n": 1, "text": "TODO TODO TODO",
        "fragments": [ { "col": [1, 5], "text": "TODO", "ids": ["#1"] } ] } ] } ] }
  ]
}
```

## E97.4 Before the limits of the response

`max_matches` counts what is left of the files.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches_per_file": 1, "max_matches": 3, "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches, max_matches_per_file)  next cursor: eyJ2IjoxLCJoIjoiM2Y2Y2EzNzciLCJmIjozLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

a.txt  [#1]  lines: 1  matches: 1
1:⟦#1:TODO⟧
b.txt  [#1]  lines: 1  matches: 3  (cut)
1:⟦#1:TODO⟧ TODO TODO
c.txt  [#1]  lines: 1  matches: 2  (cut)
1:⟦#1:TODO⟧ TODO
d/e.txt  [#1]  lines: 1  matches: 2  (cut)
```

## E97.5 Only with lines

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches_per_file": 1, "detail": "files", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/e.txt  [#1]  lines: 1  matches: 2
```

## E97.6 For the session

Fixture: `sorting`

1. `update_config`

```json request update_config
{"set": "output.max_matches_per_file = 1"}
```

```text response
[output]
max_matches_per_file = 1    # was 50
```

2. `find`

```json request
{"query": "text has 'TODO' and name = 'c.txt'"}
```

```text response
files: 1  lines: 1  matches: 2  result: r1  truncated: yes (max_matches_per_file)
#1 text has 'TODO'
#2 name = 'c.txt'

./
c.txt  [#1 #2]  lines: 1  matches: 2  (cut)
1:⟦#1:TODO⟧ TODO
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
max_matches_per_file = 1    # update
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

## E97.7 An invalid value in a request

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"max_matches_per_file": 0}}
```

```text error
error[invalid_option]: 'output.max_matches_per_file' must be a whole number of at least 1
```

## E97.8 An invalid value for `update_config`

Fixture: `sorting`

```json request update_config
{"set": "output.max_matches_per_file = 0"}
```

```text error
error[invalid_value]: invalid value for 'output.max_matches_per_file'
  hint: a whole number of at least 1 is expected
```

## E97.9 From the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_matches_per_file = 2
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO' and name = 'b.txt'"}
```

```text response
files: 1  lines: 1  matches: 3  result: r1  truncated: yes (max_matches_per_file)
#1 text has 'TODO'
#2 name = 'b.txt'

./
b.txt  [#1 #2]  lines: 1  matches: 3  (cut)
1:⟦#1:TODO⟧ ⟦#1:TODO⟧ TODO
```

## E97.10 An invalid value in the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_matches_per_file = "50"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_matches_per_file': a whole number of at least 1 is expected
```
