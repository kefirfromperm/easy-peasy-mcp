# Examples for task 108

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `huge` of task 104: eleven files with `error` — `docs/d1.md`, two in `src/`, two in
`tests/`, six in `vendor/lib/`; with `group_repeated = 0` and `sort = "path asc"` their lines are listed in path
order. A token is estimated as four characters (`chars_per_token` comes with task 109): the lines make 712
characters (178 tokens), the list of the files 524 (131 tokens).

## E108.1 Not set by default

Fixture: `huge`

1. `find`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
1:an ⟦#1:error⟧ here
src/
b1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
b2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
tests/
t1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
t2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
vendor/lib/
a1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a3.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a4.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a5.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a6.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
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

## E108.2 A budget the response fits in

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc", "budget": 178}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
1:an ⟦#1:error⟧ here
src/
b1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
b2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
tests/
t1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
t2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
vendor/lib/
a1.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a2.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a3.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a4.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a5.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
a6.rs  [#1]  lines: 1  matches: 1
1:let e = ⟦#1:error⟧;
```

## E108.3 The files without lines

The lines don't fit in 140 tokens, all the files do.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc", "budget": 140}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  detail: files (budget 140)  truncated: yes (budget)
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
src/
b1.rs  [#1]  lines: 1  matches: 1
b2.rs  [#1]  lines: 1  matches: 1
tests/
t1.rs  [#1]  lines: 1  matches: 1
t2.rs  [#1]  lines: 1  matches: 1
vendor/lib/
a1.rs  [#1]  lines: 1  matches: 1
a2.rs  [#1]  lines: 1  matches: 1
a3.rs  [#1]  lines: 1  matches: 1
a4.rs  [#1]  lines: 1  matches: 1
a5.rs  [#1]  lines: 1  matches: 1
a6.rs  [#1]  lines: 1  matches: 1
```

## E108.4 The facets

In 100 tokens neither the lines nor ten files fit.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc", "budget": 100}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  detail: facets (budget 100)  truncated: yes (budget)
#1 text has 'error'
dir:        vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
lang:       rust 10 · markdown 1
kind:       source 8 · test 2 · doc 1
condition:  #1 11
```

## E108.5 In JSON

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"format": "json", "budget": 100}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "detail_used": "facets",
  "facets": {
    "dir": { "vendor/lib/": 6, "src/": 2, "tests/": 2, "docs/": 1 },
    "lang": { "rust": 10, "markdown": 1 },
    "kind": { "source": 8, "test": 2, "doc": 1 },
    "condition": { "#1": 11 }
  },
  "counts": { "files": 11, "lines": 11, "matches": 11 },
  "truncated": true,
  "truncated_by": ["budget"],
  "legend": { "#1": "text has 'error'" }
}
```

## E108.6 Never a higher detail

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"detail": "count", "budget": 100}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
```

## E108.7 For the session

Fixture: `huge`

1. `update_config`

```json request update_config
{"set": "output.budget = 140"}
```

```text response
[output]
budget = 140    # was (not set)
```

2. `find`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  detail: files (budget 140)  truncated: yes (budget)
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
src/
b1.rs  [#1]  lines: 1  matches: 1
b2.rs  [#1]  lines: 1  matches: 1
tests/
t1.rs  [#1]  lines: 1  matches: 1
t2.rs  [#1]  lines: 1  matches: 1
vendor/lib/
a1.rs  [#1]  lines: 1  matches: 1
a2.rs  [#1]  lines: 1  matches: 1
a3.rs  [#1]  lines: 1  matches: 1
a4.rs  [#1]  lines: 1  matches: 1
a5.rs  [#1]  lines: 1  matches: 1
a6.rs  [#1]  lines: 1  matches: 1
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
budget = 140    # update
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

## E108.8 An invalid value in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"budget": 99}}
```

```text error
error[invalid_option]: 'output.budget' must be a whole number of at least 100
```

## E108.9 An invalid value for `update_config`

Fixture: `huge`

```json request update_config
{"set": "output.budget = 'small'"}
```

```text error
error[invalid_value]: invalid value for 'output.budget'
  hint: a whole number of at least 100 is expected
```

## E108.10 From the config file

Fixture: `huge`

```toml conf/o.toml
[output]
budget = 140
group_repeated = 0
sort = "path asc"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'error'"}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  detail: files (budget 140)  truncated: yes (budget)
#1 text has 'error'

docs/
d1.md  [#1]  lines: 1  matches: 1
src/
b1.rs  [#1]  lines: 1  matches: 1
b2.rs  [#1]  lines: 1  matches: 1
tests/
t1.rs  [#1]  lines: 1  matches: 1
t2.rs  [#1]  lines: 1  matches: 1
vendor/lib/
a1.rs  [#1]  lines: 1  matches: 1
a2.rs  [#1]  lines: 1  matches: 1
a3.rs  [#1]  lines: 1  matches: 1
a4.rs  [#1]  lines: 1  matches: 1
a5.rs  [#1]  lines: 1  matches: 1
a6.rs  [#1]  lines: 1  matches: 1
```

## E108.11 An invalid value in the config file

Fixture: `huge`

```toml conf/o.toml
[output]
budget = 50
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.budget': a whole number of at least 100 is expected
```
