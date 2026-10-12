# Examples for task 102

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The hints
of empty and huge results come with tasks 103 and 104.

## Fixtures

### funnel

```text tree
docs/c.md                  2026-10-01 10:00
many/f01.txt               2026-10-01 10:00
many/f02.txt               2026-10-01 10:00
many/f03.txt               2026-10-01 10:00
many/f04.txt               2026-10-01 10:00
many/f05.txt               2026-10-01 10:00
many/f06.txt               2026-10-01 10:00
many/f07.txt               2026-10-01 10:00
many/f08.txt               2026-10-01 10:00
many/f09.txt               2026-10-01 10:00
many/f10.txt               2026-10-01 10:00
many/f11.txt               2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
tools/d.py                 2026-10-01 10:00
```

In `tests/fixtures/funnel/`: `src/a.rs` is `fn parse_query() {}`, `src/b.rs` `fn other() {}`, `docs/c.md`
`parseQuery is documented here`, `tools/d.py` `def parse_query(): pass`, the eleven `many/*.txt` are `x`. Times are
UTC.

## E102.1 An empty result

Fixture: `funnel`

1. `find`

```json request
{"query": "lang = rust and text has 'parseQuery'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  lang = rust                            2
  and text has 'parseQuery'              0
hints:
  text has 'parseQuery' /n               1 file: parse_query
```

2. `get_config`

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

## E102.2 Metadata steps first

The `lang` condition is counted before the content condition, which comes first in the query.

Fixture: `funnel`

```json request
{"query": "text has 'parse' and lang = rust", "output": {"funnel": "always"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'parse'
#2 lang = rust
funnel:
  lang = rust                            2
  and text has 'parse'                   1

src/
a.rs  [#1 #2]  lines: 1  matches: 1
1:fn ⟦#1:parse⟧_query() {}
```

## E102.3 The branches of `or`

A file is counted in its first branch only: `src/a.rs` in the first, `src/b.rs` in the second.

Fixture: `funnel`

```json request
{"query": "name = 'a.rs' or lang = rust or text has 'parse'", "output": {"funnel": "always", "detail": "count"}}
```

```text response
files: 4  lines: 3  matches: 3  result: r1
funnel:
  name = 'a.rs'                          1
  or lang = rust                         1
  or text has 'parse'                    2
```

## E102.4 Brackets are one step

Fixture: `funnel`

```json request
{"query": "lang = rust and (text has 'parse' or text has 'other')", "output": {"funnel": "always", "detail": "count"}}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
funnel:
  lang = rust                                 2
  and (text has 'parse' or text has 'other')  2
```

## E102.5 A huge result

More than 10 × `max_files` files.

Fixture: `funnel`

```json request
{"query": "name ~ '*.txt'", "output": {"detail": "files", "sort": "path asc", "max_files": 1}}
```

```text response
files: 11  lines: 0  matches: 0  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMTg3MjQyMjgiLCJmIjoxLCJ0IjpbMTEsMCwwXX0
#1 name ~ '*.txt'
funnel:
  name ~ '*.txt'                        11
hints:
  dir: many/ 11
  lang: unknown 11
  narrow with: kind = source

many/
f01.txt  [#1]
```

## E102.6 In JSON

Fixture: `funnel`

```json request
{"query": "lang = rust and text has 'parseQuery'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": false,
  "result": "r1",
  "funnel": [
    { "step": "lang = rust", "files": 2 },
    { "step": "and text has 'parseQuery'", "files": 0 }
  ],
  "hints": ["text has 'parseQuery' /n  1 file: parse_query"],
  "counts": { "files": 0, "lines": 0, "matches": 0 }
}
```

## E102.7 Never

Fixture: `funnel`

```json request
{"query": "lang = rust and text has 'parseQuery'", "output": {"funnel": "never"}}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
```

## E102.8 For the session

Fixture: `funnel`

1. `update_config`

```json request update_config
{"set": "output.funnel = \"always\""}
```

```text response
[output]
funnel = "always"    # was "auto"
```

2. `find`

```json request
{"query": "name = 'd.py'", "output": {"detail": "count"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
funnel:
  name = 'd.py'                          1
```

3. `get_config`

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
chars_per_token = 4    # default
funnel = "always"    # update
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

## E102.9 An invalid value in a request

Fixture: `funnel`

```json request
{"query": "name = 'd.py'", "output": {"funnel": "sometimes"}}
```

```text error
error[invalid_option]: 'output.funnel' must be one of: auto, always, never
```

## E102.10 An invalid value for `update_config`

Fixture: `funnel`

```json request update_config
{"set": "output.funnel = true"}
```

```text error
error[invalid_value]: invalid value for 'output.funnel'
  hint: expected one of: auto, always, never
```

## E102.11 From the config file

Fixture: `funnel`

```toml conf/o.toml
[output]
funnel = "never"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "lang = rust and text has 'parseQuery'"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
```

## E102.12 An invalid value in the config file

Fixture: `funnel`

```toml conf/o.toml
[output]
funnel = "yes"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.funnel': expected one of: auto, always, never
```
