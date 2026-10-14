# Examples for task 96

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `sorting` of task 92: `a.txt` (`TODO`), `b.txt` (`TODO TODO TODO`), `c.txt` and `d/e.txt`
(`TODO TODO`), and set `sort = "path asc"`. The cursor of the next page comes with task 101.

## E96.1 1000 matches by default

Fixture: `sorting`

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
groups = []
only_match = false
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

## E96.2 The first matches

The next fragments of a line are not marked, the next lines and the lines of the next files are not listed; the
files are, with their counts.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiNWE2MTI2OTUiLCJmIjoxLCJtIjoxLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
1:TODO
b.txt  [#1]  lines: 1  matches: 3
1:⟦#1:TODO⟧ TODO TODO
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

## E96.3 In JSON

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches": 2, "detail": "lines", "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 4, "lines": 4, "matches": 8 },
  "truncated": true,
  "truncated_by": ["max_matches"],
  "next_cursor": "eyJ2IjoxLCJoIjoiZmUwZGI2MGEiLCJmIjoxLCJtIjoxLCJ0IjpbNCw0LDhdfQ",
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "a.txt", "lines": 1, "matches": 1, "conditions": ["#1"],
      "blocks": [ { "lines": [ { "n": 1, "text": "TODO",
        "fragments": [ { "col": [1, 5], "text": "TODO", "ids": ["#1"] } ] } ] } ] },
    { "path": "b.txt", "lines": 1, "matches": 3, "conditions": ["#1"],
      "blocks": [ { "lines": [ { "n": 1, "text": "TODO TODO TODO",
        "fragments": [ { "col": [1, 5], "text": "TODO", "ids": ["#1"] } ] } ] } ] },
    { "path": "c.txt", "lines": 1, "matches": 2, "conditions": ["#1"] },
    { "path": "d/e.txt", "lines": 1, "matches": 2, "conditions": ["#1"] }
  ]
}
```

## E96.4 With `max_lines`

Both limits are named.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches": 3, "max_lines": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_lines, max_matches)  next cursor: eyJ2IjoxLCJoIjoiZDA5OWYyNWMiLCJmIjoxLCJtIjoyLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
1:TODO
b.txt  [#1]  lines: 1  matches: 3
1:⟦#1:TODO⟧ ⟦#1:TODO⟧ TODO
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

## E96.5 Not hit

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches": 8, "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

a.txt  [#1]  lines: 1  matches: 1
1:TODO
b.txt  [#1]  lines: 1  matches: 3
1:TODO TODO TODO
c.txt  [#1]  lines: 1  matches: 2
1:TODO TODO
d/e.txt  [#1]  lines: 1  matches: 2
1:TODO TODO
```

## E96.6 For the session

Fixture: `sorting`

1. `update_config`

```json request update_config
{"set": "output.max_matches = 1"}
```

```text response
[output]
max_matches = 1    # was 1000
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path desc", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiOWMxMGRjZTgiLCJmIjowLCJtIjoxLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

d/e.txt  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ TODO
c.txt  [#1]  lines: 1  matches: 2
b.txt  [#1]  lines: 1  matches: 3
a.txt  [#1]  lines: 1  matches: 1
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
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
groups = []    # default
only_match = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1    # update
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

## E96.7 An invalid value in a request

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"max_matches": 0}}
```

```text error
error[invalid_option]: 'output.max_matches' must be a whole number of at least 1
```

## E96.8 An invalid value for `update_config`

Fixture: `sorting`

```json request update_config
{"set": "output.max_matches = 1.5"}
```

```text error
error[invalid_value]: invalid value for 'output.max_matches'
  hint: a whole number of at least 1 is expected
```

## E96.9 From the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_matches = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiOWFmNTY0YjQiLCJmIjoxLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

a.txt  [#1]  lines: 1  matches: 1
1:TODO
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/e.txt  [#1]  lines: 1  matches: 2
```

## E96.10 An invalid value in the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_matches = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_matches': a whole number of at least 1 is expected
```
