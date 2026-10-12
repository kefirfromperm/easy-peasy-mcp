# Examples for task 109

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `huge` of task 104
with `group_repeated = 0` and `sort = "path asc"`: the lines make 712 characters, the list of the files 524 (see the
examples of task 108).

## E109.1 4 characters per token by default

Fixture: `huge`

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

## E109.2 More characters per token

712 characters are 140 tokens of 5.1 characters: the lines fit in 140 tokens.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc", "budget": 140, "chars_per_token": 5.1}}
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

## E109.3 Fewer characters per token

The lines are 238 tokens of 3 characters, the list of the files 175.

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"group_repeated": 0, "sort": "path asc", "budget": 180, "chars_per_token": 3}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1  detail: files (budget 180)  truncated: yes (budget)
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

## E109.4 For the session

Fixture: `huge`

1. `update_config`

```json request update_config
{"set": "output.chars_per_token = 3.5"}
```

```text response
[output]
chars_per_token = 3.5    # was 4
```

2. `get_config`

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
chars_per_token = 3.5    # update
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
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
```

## E109.5 An invalid value in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"chars_per_token": 0}}
```

```text error
error[invalid_option]: 'output.chars_per_token' must be a positive number
```

## E109.6 An invalid value for `update_config`

Fixture: `huge`

```json request update_config
{"set": "output.chars_per_token = -1"}
```

```text error
error[invalid_value]: invalid value for 'output.chars_per_token'
  hint: a positive number is expected
```

## E109.7 From the config file

Fixture: `huge`

```toml conf/o.toml
[output]
chars_per_token = 5.1
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

## E109.8 An invalid value in the config file

Fixture: `huge`

```toml conf/o.toml
[output]
chars_per_token = "four"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.chars_per_token': a positive number is expected
```
