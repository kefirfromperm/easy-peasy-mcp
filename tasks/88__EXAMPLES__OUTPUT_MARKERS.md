# Examples for task 88

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. Some
examples use the fixture `highlight` of task 87: `src/config.rs` with the line
`    let cfg: Config = toml::from_str(raw).unwrap();`.

## Fixtures

### marker-text

```text tree
src/notes.txt              2026-10-01 10:00
```

`src/notes.txt` (in `tests/fixtures/marker-text/`):

```
see ⟦x⟧ here: TODO
TODO: plain
```

## E88.1 The default markers

Fixture: `highlight`

1. `find`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).⟦#1:unwrap(⟧);
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

## E88.2 Other markers in a request

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"markers": ["[[", "]]"]}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).[[#1:unwrap(]]);
```

## E88.3 Other markers in JSON

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"format": "json", "highlight": "markers", "markers": ["«", "»"]}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "text has 'unwrap('" },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        { "indent": 4,
          "lines": [ { "n": 2, "text": "let cfg: Config = toml::from_str(raw).«#1:unwrap(»);" } ] }
      ]
    }
  ]
}
```

## E88.4 A marker in a matched line

The response has the warning `marker_in_content`.

Fixture: `marker-text`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

src/
notes.txt  [#1]  lines: 2  matches: 2
1:see ⟦x⟧ here: ⟦#1:TODO⟧
2:⟦#1:TODO⟧: plain

warning: marker_in_content (1): src/notes.txt
```

## E88.5 The warning in JSON

Fixture: `marker-text`

```json request
{"query": "text has 'TODO'", "output": {"format": "json", "highlight": "markers", "detail": "files"}}
```

The warning is only given in `lines` detail, when the lines are shown:

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has 'TODO'" },
  "files": [ { "path": "src/notes.txt", "lines": 2, "matches": 2, "conditions": ["#1"] } ]
}
```

```json request
{"query": "text has 'TODO'", "output": {"format": "json", "highlight": "markers"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has 'TODO'" },
  "warnings": [ { "code": "marker_in_content", "count": 1, "paths": ["src/notes.txt"] } ],
  "files": [
    {
      "path": "src/notes.txt",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
            { "n": 1, "text": "see ⟦x⟧ here: ⟦#1:TODO⟧" },
            { "n": 2, "text": "⟦#1:TODO⟧: plain" }
          ] }
      ]
    }
  ]
}
```

## E88.6 No warning with other markers or with columns

Fixture: `marker-text`

```json request
{"query": "text has 'TODO'", "output": {"markers": ["«", "»"]}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

src/
notes.txt  [#1]  lines: 2  matches: 2
1:see ⟦x⟧ here: «#1:TODO»
2:«#1:TODO»: plain
```

```json request
{"query": "text has 'TODO'", "output": {"highlight": "columns"}}
```

```text response
files: 1  lines: 2  matches: 2  result: r2
#1 text has 'TODO'

src/
notes.txt  [#1]  lines: 2  matches: 2
1:see ⟦x⟧ here: TODO
  @ 15..19 #1
2:TODO: plain
  @ 1..5 #1
```

## E88.7 For the session

Fixture: `marker-text`

1. `update_config`

```json request update_config
{"set": "output.markers = ['«', '»']"}
```

```text response
[output]
markers = ['«', '»']    # was ['⟦', '⟧']
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

src/
notes.txt  [#1]  lines: 2  matches: 2
1:see ⟦x⟧ here: «#1:TODO»
2:«#1:TODO»: plain
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
markers = ['«', '»']    # update
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

## E88.8 Invalid values in a request

Fixture: `highlight`

```json request
{"query": "text has 'unwrap('", "output": {"markers": ["<"]}}
```

```text error
error[invalid_option]: 'output.markers' must be two non-empty strings
```

```json request
{"query": "text has 'unwrap('", "output": {"markers": ["", ">"]}}
```

```text error
error[invalid_option]: 'output.markers' must be two non-empty strings
```

## E88.9 An invalid value for `update_config`

Fixture: `highlight`

```json request update_config
{"set": "output.markers = '<>'"}
```

```text error
error[invalid_value]: invalid value for 'output.markers'
  hint: two non-empty strings are expected
```

## E88.10 From the config file

Fixture: `highlight`

```toml conf/o.toml
[output]
markers = ["<<", ">>"]
```

```text command
easy-peasy-mcp --config conf/o.toml
```

1. `get_config`

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
markers = ['<<', '>>']    # file: conf/o.toml
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

2. `find`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/
config.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg: Config = toml::from_str(raw).<<#1:unwrap(>>);
```

## E88.11 An invalid value in the config file

Fixture: `highlight`

```toml conf/o.toml
[output]
markers = ["<", ">", "!"]
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.markers': two non-empty strings are expected
```
