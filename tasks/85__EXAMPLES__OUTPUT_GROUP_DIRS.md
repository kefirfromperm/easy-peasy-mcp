# Examples for task 85

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `detail` of task 79:
`src/a.rs` and `src/b.rs` with two lines with `TODO` each.

## E85.1 Directory groups by default

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
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

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

## E85.2 Full paths in a request

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"group_dirs": false}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
src/b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
2:// ⟦#1:TODO⟧ three
```

## E85.3 Full paths for the session

Fixture: `detail`

1. `update_config`

```json request update_config
{"set": "output.group_dirs = false"}
```

```text response
[output]
group_dirs = false    # was true
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
src/b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
2:// ⟦#1:TODO⟧ three
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
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = false    # update
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
```

## E85.4 An invalid value in a request

Fixture: `detail`

```json request
{"output": {"group_dirs": "no"}}
```

```text error
error[invalid_option]: 'output.group_dirs' must be true or false
```

## E85.5 An invalid value for `update_config`

Fixture: `detail`

```json request update_config
{"set": "output.group_dirs = []"}
```

```text error
error[invalid_value]: invalid value for 'output.group_dirs'
  hint: true or false is expected
```

## E85.6 From the config file

Fixture: `detail`

```toml conf/o.toml
[output]
group_dirs = false
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
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
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
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = false    # file: conf/o.toml
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
src/b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
2:// ⟦#1:TODO⟧ three
```

## E85.7 An invalid value in the config file

Fixture: `detail`

```toml conf/o.toml
[output]
group_dirs = "no"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.group_dirs': true or false is expected
```
