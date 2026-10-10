# Examples for task 98

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### long-lines

```text tree
notes.txt                  2026-10-01 10:00
```

`notes.txt` (in `tests/fixtures/long-lines/`):

```
The quick brown fox jumps over the lazy dog and then TODO fix the fence near the old barn
short TODO
This context line is much longer than twenty characters
TODO at the start and TODO again at the end of this rather long line TODO
```

## E98.1 300 characters by default

Fixture: `long-lines`

1. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 1  lines: 3  matches: 5  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5
1:The quick brown fox jumps over the lazy dog and then ⟦#1:TODO⟧ fix the fence near the old barn
2:short ⟦#1:TODO⟧
--
4:⟦#1:TODO⟧ at the start and ⟦#1:TODO⟧ again at the end of this rather long line ⟦#1:TODO⟧
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
# budget =  (not set)
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

## E98.2 Windows around the fragments

A fragment of four characters in a line cut to 20 characters keeps eight characters on each side; three fragments
keep one character on each side.

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 20}}
```

```text response
files: 1  lines: 3  matches: 5  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5
1:…nd then ⟦#1:TODO⟧ fix the…
2:short ⟦#1:TODO⟧
--
4:⟦#1:TODO⟧ … ⟦#1:TODO⟧ … ⟦#1:TODO⟧
```

## E98.3 A context line keeps its start

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 20, "context": 1}}
```

```text response
files: 1  lines: 3  matches: 5  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5
1:…nd then ⟦#1:TODO⟧ fix the…
2:short ⟦#1:TODO⟧
3-This context line is…
4:⟦#1:TODO⟧ … ⟦#1:TODO⟧ … ⟦#1:TODO⟧
```

## E98.4 In JSON

`col` refers to the original line.

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 60, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 3, "matches": 5 },
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    {
      "path": "notes.txt",
      "lines": 3,
      "matches": 5,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 1, "cut": true, "text": "… over the lazy dog and then TODO fix the fence near the old …",
            "fragments": [ { "col": [54, 58], "text": "TODO", "ids": ["#1"] } ] },
          { "n": 2, "text": "short TODO",
            "fragments": [ { "col": [7, 11], "text": "TODO", "ids": ["#1"] } ] }
        ] },
        { "lines": [
          { "n": 4, "cut": true, "text": "TODO at the …art and TODO again a…ng line TODO",
            "fragments": [
              { "col": [1, 5], "text": "TODO", "ids": ["#1"] },
              { "col": [23, 27], "text": "TODO", "ids": ["#1"] },
              { "col": [70, 74], "text": "TODO", "ids": ["#1"] }
            ] }
        ] }
      ]
    }
  ]
}
```

## E98.5 With columns

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 20, "highlight": "columns"}}
```

```text response
files: 1  lines: 3  matches: 5  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5
1:…nd then TODO fix the…
  @ 54..58 #1
2:short TODO
  @ 7..11 #1
--
4:TODO … TODO … TODO
  @ 1..5 #1  @ 23..27 #1  @ 70..74 #1
```

## E98.6 For the session

Fixture: `long-lines`

1. `update_config`

```json request update_config
{"set": "output.max_line_length = 60"}
```

```text response
[output]
max_line_length = 60    # was 300
```

2. `find`

```json request
{"query": "text has 'TODO'"}
```

```text response
files: 1  lines: 3  matches: 5  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5
1:… over the lazy dog and then ⟦#1:TODO⟧ fix the fence near the old …
2:short ⟦#1:TODO⟧
--
4:⟦#1:TODO⟧ at the …art and ⟦#1:TODO⟧ again a…ng line ⟦#1:TODO⟧
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
# budget =  (not set)    # default
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
max_line_length = 60    # update
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E98.7 An invalid value in a request

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 19}}
```

```text error
error[invalid_option]: 'output.max_line_length' must be a whole number of at least 20
```

## E98.8 An invalid value for `update_config`

Fixture: `long-lines`

```json request update_config
{"set": "output.max_line_length = 10"}
```

```text error
error[invalid_value]: invalid value for 'output.max_line_length'
  hint: a whole number of at least 20 is expected
```

## E98.9 From the config file

Fixture: `long-lines`

```toml conf/o.toml
[output]
max_line_length = 20
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'fox'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'fox'

./
notes.txt  [#1]  lines: 1  matches: 1
1:…k brown ⟦#1:fox⟧ jumps o…
```

## E98.10 An invalid value in the config file

Fixture: `long-lines`

```toml conf/o.toml
[output]
max_line_length = "wide"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_line_length': a whole number of at least 20 is expected
```
