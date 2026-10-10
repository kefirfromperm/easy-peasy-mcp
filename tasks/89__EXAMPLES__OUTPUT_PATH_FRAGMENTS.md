# Examples for task 89

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### path-frags

```text tree
docs/guide.md              2026-10-01 10:00
src/config.rs              2026-10-01 10:00
src/config_loader.rs       2026-10-01 10:00
```

`src/config.rs` is `pub struct Config;`, `src/config_loader.rs` is `pub fn load() -> Config { Config }` (in
`tests/fixtures/path-frags/`).

## E89.1 Off by default

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

## E89.2 The matched part of a name

Fixture: `path-frags`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'config'

src/
config.rs  [#1]
  path: src/⟦#1:config⟧.rs
config_loader.rs  [#1]
  path: src/⟦#1:config⟧_loader.rs
```

## E89.3 In JSON

Fixture: `path-frags`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 2, "lines": 0, "matches": 0 },
  "legend": { "#1": "name has 'config'" },
  "files": [
    { "path": "src/config.rs", "conditions": ["#1"],
      "path_fragments": [ { "col": [5, 11], "text": "config", "ids": ["#1"] } ] },
    { "path": "src/config_loader.rs", "conditions": ["#1"],
      "path_fragments": [ { "col": [5, 11], "text": "config", "ids": ["#1"] } ] }
  ]
}
```

## E89.4 `path` and `name` conditions

Fixture: `path-frags`

```json request
{"query": "path starts 'src/' and name ends '.rs'", "output": {"path_fragments": true}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path starts 'src/'
#2 name ends '.rs'

src/
config.rs  [#1 #2]
  path: ⟦#1:src/⟧config⟦#2:.rs⟧
config_loader.rs  [#1 #2]
  path: ⟦#1:src/⟧config_loader⟦#2:.rs⟧
```

## E89.5 A wildcard matches the whole path

Fixture: `path-frags`

```json request
{"query": "path ~ 'src/*_*.rs'", "output": {"path_fragments": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ 'src/*_*.rs'

src/
config_loader.rs  [#1]
  path: ⟦#1:src/config_loader.rs⟧
```

## E89.6 The characters of a fuzzy match

Fixture: `path-frags`

```json request
{"query": "name ~~ 'cfgldr'", "output": {"path_fragments": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'

src/
config_loader.rs  [#1]
  path: src/⟦#1:c⟧on⟦#1:f⟧i⟦#1:g⟧_⟦#1:l⟧oa⟦#1:d⟧er.⟦#1:r⟧s
```

## E89.7 A directory

Fixture: `path-frags`

Without a `type` condition only files are returned, so the query asks for directories.

```json request
{"query": "type = dir and name = 'docs'", "output": {"path_fragments": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = dir
#2 name = 'docs'

./
docs/  [#1 #2]
  path: ⟦#2:docs⟧/
```

## E89.8 With columns

Fixture: `path-frags`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true, "highlight": "columns"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'config'

src/
config.rs  [#1]
  path @ 5..11 #1
config_loader.rs  [#1]
  path @ 5..11 #1
```

## E89.9 Before the lines; not counted as matches

Fixture: `path-frags`

```json request
{"query": "name has 'loader' and text has 'Config'", "output": {"path_fragments": true}}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 name has 'loader'
#2 text has 'Config'

src/
config_loader.rs  [#1 #2]  lines: 1  matches: 2
  path: src/config_⟦#1:loader⟧.rs
1:pub fn load() -> ⟦#2:Config⟧ { ⟦#2:Config⟧ }
```

## E89.10 For the session

Fixture: `path-frags`

1. `update_config`

```json request update_config
{"set": "output.path_fragments = true"}
```

```text response
[output]
path_fragments = true    # was false
```

2. `find`

```json request
{"query": "name has 'loader'", "output": {"detail": "files"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'loader'

src/
config_loader.rs  [#1]
  path: src/config_⟦#1:loader⟧.rs
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
path_fragments = true    # update
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

## E89.11 An invalid value in a request

Fixture: `path-frags`

```json request
{"query": "name has 'loader'", "output": {"path_fragments": "yes"}}
```

```text error
error[invalid_option]: 'output.path_fragments' must be true or false
```

## E89.12 An invalid value for `update_config`

Fixture: `path-frags`

```json request update_config
{"set": "output.path_fragments = 1"}
```

```text error
error[invalid_value]: invalid value for 'output.path_fragments'
  hint: true or false is expected
```

## E89.13 From the config file

Fixture: `path-frags`

```toml conf/o.toml
[output]
path_fragments = true
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "name has 'loader'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'loader'

src/
config_loader.rs  [#1]
  path: src/config_⟦#1:loader⟧.rs
```

## E89.14 An invalid value in the config file

Fixture: `path-frags`

```toml conf/o.toml
[output]
path_fragments = "yes"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.path_fragments': true or false is expected
```
