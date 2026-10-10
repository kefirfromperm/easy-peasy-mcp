# Examples for task 99

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. Some
examples use the fixture `long-lines` of task 98: `notes.txt` with lines of 89, 10, 55 and 73 characters, `TODO` in
the first, the second and the fourth.

## Fixtures

### output-limits

```text tree
a.txt                      2026-10-01 10:00
b.txt                      2026-10-01 10:00
```

In `tests/fixtures/output-limits/`: `a.txt` has three lines of 49, 50 and 50 characters with a `TODO` each, `b.txt`
the line `a short TODO`. Times are UTC.

## E99.1 4000 characters per file by default

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

## E99.2 The first lines of a file

The third line of `a.txt` would make 149 characters; the file is marked `(cut)` and the next file is listed as usual.

Fixture: `output-limits`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output_per_file": 100}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_output_per_file)
#1 text has 'TODO'

./
a.txt  [#1]  lines: 3  matches: 3  (cut)
1:the first line has a ⟦#1:TODO⟧ and some words after it
2:the second line has a ⟦#1:TODO⟧ and more words after it
b.txt  [#1]  lines: 1  matches: 1
1:a short ⟦#1:TODO⟧
```

## E99.3 Context lines count

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"context": 1, "max_output_per_file": 100}}
```

```text response
files: 1  lines: 3  matches: 5  result: r1  truncated: yes (max_output_per_file)
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 3  matches: 5  (cut)
1:The quick brown fox jumps over the lazy dog and then ⟦#1:TODO⟧ fix the fence near the old barn
2:short ⟦#1:TODO⟧
```

## E99.4 The lines as they are shown

Cut to 20 characters, the lines of `notes.txt` have 50 characters.

Fixture: `long-lines`

```json request
{"query": "text has 'TODO'", "output": {"max_line_length": 20, "max_output_per_file": 100}}
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

## E99.5 In JSON

Fixture: `output-limits`

```json request
{"query": "text has 'TODO' and name = 'a.txt'", "output": {"max_output_per_file": 100, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 3, "matches": 3 },
  "truncated": true,
  "truncated_by": ["max_output_per_file"],
  "legend": { "#1": "text has 'TODO'", "#2": "name = 'a.txt'" },
  "files": [
    {
      "path": "a.txt",
      "truncated": true,
      "lines": 3,
      "matches": 3,
      "conditions": ["#1", "#2"],
      "blocks": [
        { "lines": [
          { "n": 1, "text": "the first line has a TODO and some words after it",
            "fragments": [ { "col": [22, 26], "text": "TODO", "ids": ["#1"] } ] },
          { "n": 2, "text": "the second line has a TODO and more words after it",
            "fragments": [ { "col": [23, 27], "text": "TODO", "ids": ["#1"] } ] }
        ] }
      ]
    }
  ]
}
```

## E99.6 For the session

Fixture: `output-limits`

1. `update_config`

```json request update_config
{"set": "output.max_output_per_file = 120"}
```

```text response
[output]
max_output_per_file = 120    # was 4000
```

2. `find`

```json request
{"query": "text has 'TODO' and name = 'a.txt'"}
```

```text response
files: 1  lines: 3  matches: 3  result: r1  truncated: yes (max_output_per_file)
#1 text has 'TODO'
#2 name = 'a.txt'

./
a.txt  [#1 #2]  lines: 3  matches: 3  (cut)
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
max_output_per_file = 120    # update
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E99.7 An invalid value in a request

Fixture: `output-limits`

```json request
{"query": "text has 'TODO'", "output": {"max_output_per_file": 99}}
```

```text error
error[invalid_option]: 'output.max_output_per_file' must be a whole number of at least 100
```

## E99.8 An invalid value for `update_config`

Fixture: `output-limits`

```json request update_config
{"set": "output.max_output_per_file = 50"}
```

```text error
error[invalid_value]: invalid value for 'output.max_output_per_file'
  hint: a whole number of at least 100 is expected
```

## E99.9 From the config file

Fixture: `output-limits`

```toml conf/o.toml
[output]
max_output_per_file = 149
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO' and name = 'a.txt'"}
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 text has 'TODO'
#2 name = 'a.txt'

./
a.txt  [#1 #2]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
2:the second line has a ⟦#1:TODO⟧ and more words after it
3:the third line has a ⟦#1:TODO⟧ and the last words of it
```

## E99.10 An invalid value in the config file

Fixture: `output-limits`

```toml conf/o.toml
[output]
max_output_per_file = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.max_output_per_file': a whole number of at least 100 is expected
```
