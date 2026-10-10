# Examples for task 100

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. Most
examples use the fixture `output-limits` of task 99: `a.txt` with three lines of 49, 50 and 50 characters, `b.txt`
with one of 12, a `TODO` in each line.

## Fixtures

### long-names

```text tree
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.md   2026-10-01 10:00
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.md   2026-10-01 10:00
```

Two files with names of 63 characters (in `tests/fixtures/long-names/`), each with the line `x`. Times are UTC.

## E100.1 40000 characters by default

Fixture: `output-limits`

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

## E100.2 The paths and lines which fit

`a.txt` and its first line make 54 characters, its second line would make 104: nothing is listed after it.

Fixture: `output-limits`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output": 100}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYTY3ZTM0ZGEiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
```

With 160 characters `a.txt` is listed whole (154), the path of `b.txt` fits, its line doesn't:

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output": 160}}
```

```text response
files: 2  lines: 4  matches: 4  result: r2  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiMzdlYzNkMzQiLCJmIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
2:the second line has a ⟦#1:TODO⟧ and more words after it
3:the third line has a ⟦#1:TODO⟧ and the last words of it
b.txt  [#1]  lines: 1  matches: 1
```

## E100.3 Paths in `files` detail

Fixture: `long-names`

```json request
{"query": "name ~ '*.md'", "output": {"sort": "path asc", "detail": "files", "max_output": 100}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiNDYyNzQxNWIiLCJmIjoxLCJ0IjpbMiwwLDBdfQ
#1 name ~ '*.md'

./
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.md  [#1]
```

## E100.4 In JSON

Fixture: `output-limits`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output": 100, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 2, "lines": 4, "matches": 4 },
  "truncated": true,
  "truncated_by": ["max_output"],
  "next_cursor": "eyJ2IjoxLCJoIjoiMmQzYWQwN2IiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ",
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    {
      "path": "a.txt",
      "lines": 3,
      "matches": 3,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 1, "text": "the first line has a TODO and some words after it",
            "fragments": [ { "col": [22, 26], "text": "TODO", "ids": ["#1"] } ] }
        ] }
      ]
    }
  ]
}
```

## E100.5 For the session

Fixture: `output-limits`

1. `update_config`

```json request update_config
{"set": "output.max_output = 160"}
```

```text response
[output]
max_output = 160    # was 40000
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path desc"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYzQ0ZjA3MGIiLCJmIjoxLCJtIjoyLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
b.txt  [#1]  lines: 1  matches: 1
1:a short ⟦#1:TODO⟧
a.txt  [#1]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
2:the second line has a ⟦#1:TODO⟧ and more words after it
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
max_output = 160    # update
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E100.6 An invalid value in a request

Fixture: `output-limits`

```json request
{"query": "text has 'TODO'", "output": {"max_output": 10}}
```

```text error
error[invalid_option]: 'output.max_output' must be a whole number of at least 100
```

## E100.7 An invalid value for `update_config`

Fixture: `output-limits`

```json request update_config
{"set": "output.max_output = 99"}
```

```text error
error[invalid_value]: invalid value for 'output.max_output'
  hint: a whole number of at least 100 is expected
```

## E100.8 From the config file

Fixture: `output-limits`

```toml conf/o.toml
[output]
max_output = 100
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO'", "output": {"sort": "path desc"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYzQ0ZjA3MGIiLCJmIjoxLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
b.txt  [#1]  lines: 1  matches: 1
1:a short ⟦#1:TODO⟧
a.txt  [#1]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
```

## E100.9 An invalid value in the config file

Fixture: `output-limits`

```toml conf/o.toml
[output]
max_output = []
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_output': a whole number of at least 100 is expected
```
