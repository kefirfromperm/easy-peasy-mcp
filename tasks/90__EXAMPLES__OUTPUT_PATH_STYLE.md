# Examples for task 90

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `path-frags` of task 89: `docs/guide.md`, `src/config.rs`, `src/config_loader.rs`.
`<fixture>` is the canonical path of the fixture directory, the working directory of the server.

## E90.1 Relative paths by default

Fixture: `path-frags`

1. `find`

```json request
{"query": "name has 'config'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'config'

src/
config.rs  [#1]
config_loader.rs  [#1]
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
funnel = "auto"
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

## E90.2 Absolute paths in a request

The directory headers are absolute too.

Fixture: `path-frags`

```json request
{"query": "name has 'config' or name = 'guide.md'", "output": {"path_style": "absolute"}}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name has 'config'
#2 name = 'guide.md'

<fixture>/src/
config.rs  [#1]
config_loader.rs  [#1]
<fixture>/docs/
guide.md  [#2]
```

## E90.3 In JSON

Fixture: `path-frags`

```json request
{"query": "name has 'loader'", "output": {"path_style": "absolute", "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "legend": { "#1": "name has 'loader'" },
  "files": [ { "path": "<fixture>/src/config_loader.rs", "conditions": ["#1"] } ]
}
```

## E90.4 From another working directory

The path doesn't depend on how the working directory is written.

Fixture: `path-frags`

```json request
{"query": "name has 'loader'", "dirs": ["src/../src"], "output": {"path_style": "absolute", "group_dirs": false}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'loader'

<fixture>/src/config_loader.rs  [#1]
```

A file directly in the working directory is under the header of the directory:

```json request
{"query": "name has 'loader'", "dirs": ["src"], "output": {"path_style": "absolute"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name has 'loader'

<fixture>/src/
config_loader.rs  [#1]
```

## E90.5 For the session

Fixture: `path-frags`

1. `update_config`

```json request update_config
{"set": "output.path_style = \"absolute\""}
```

```text response
[output]
path_style = "absolute"    # was "relative"
```

2. `find`

```json request
{"query": "name has 'loader'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'loader'

<fixture>/src/
config_loader.rs  [#1]
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
path_style = "absolute"    # update
```

## E90.6 An invalid value in a request

Fixture: `path-frags`

```json request
{"query": "name has 'loader'", "output": {"path_style": "full"}}
```

```text error
error[invalid_option]: 'output.path_style' must be one of: relative, absolute
```

## E90.7 An invalid value for `update_config`

Fixture: `path-frags`

```json request update_config
{"set": "output.path_style = \"full\""}
```

```text error
error[invalid_value]: invalid value for 'output.path_style'
  hint: expected one of: relative, absolute
```

## E90.8 From the config file

Fixture: `path-frags`

```toml conf/o.toml
[output]
path_style = "absolute"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "name has 'loader'", "output": {"group_dirs": false}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'loader'

<fixture>/src/config_loader.rs  [#1]
```

## E90.9 An invalid value in the config file

Fixture: `path-frags`

```toml conf/o.toml
[output]
path_style = "full"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.path_style': expected one of: relative, absolute
```
