# Examples for task 94

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `sorting` of task 92: `a.txt` (one `TODO`), `b.txt` (three), `c.txt` and `d/e.txt` (two
each), and set `sort = "path asc"`. The cursor of the next page comes with task 101.

## E94.1 100 files by default

Fixture: `sorting`

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
repeated_locations = 3
sort = "relevance"
path_style = "relative"
omit = []
```

## E94.2 The first files

The counts are the real totals.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
```

## E94.3 With lines

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_files": 1}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMzVmMWRhMjYiLCJmIjoxLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
1:TODO
```

## E94.4 In JSON

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 1, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 4, "lines": 4, "matches": 8 },
  "truncated": true,
  "truncated_by": ["max_files"],
  "next_cursor": "eyJ2IjoxLCJoIjoiZjEzNDRmMGQiLCJmIjoxLCJ0IjpbNCw0LDhdfQ",
  "legend": { "#1": "text has 'TODO'" },
  "files": [ { "path": "a.txt", "lines": 1, "matches": 1, "conditions": ["#1"] } ]
}
```

## E94.5 Not hit

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 4}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

Without a list of files nothing is cut:

```json request
{"query": "text has 'TODO'", "output": {"detail": "count", "max_files": 1}}
```

```text response
files: 4  lines: 4  matches: 8  result: r2
```

## E94.6 Sorted before the limit

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "size desc", "max_files": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMTA5ZDA2NWYiLCJmIjoyLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
```

## E94.7 For the session

Fixture: `sorting`

1. `update_config`

```json request update_config
{"set": "output.max_files = 3"}
```

```text response
[output]
max_files = 3    # was 100
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path desc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiZDM0MjM0NmIiLCJmIjozLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

d/
e.txt  [#1]  lines: 1  matches: 2
./
c.txt  [#1]  lines: 1  matches: 2
b.txt  [#1]  lines: 1  matches: 3
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
max_files = 3    # update
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
```

## E94.8 An invalid value in a request

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"max_files": 0}}
```

```text error
error[invalid_option]: 'output.max_files' must be a whole number of at least 1
```

## E94.9 An invalid value for `update_config`

Fixture: `sorting`

```json request update_config
{"set": "output.max_files = -1"}
```

```text error
error[invalid_value]: invalid value for 'output.max_files'
  hint: a whole number of at least 1 is expected
```

## E94.10 From the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_files = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiODg1OWJjNWQiLCJmIjoxLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
```

## E94.11 An invalid value in the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
max_files = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_files': a whole number of at least 1 is expected
```
