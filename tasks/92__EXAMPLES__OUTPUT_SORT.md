# Examples for task 92

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
order of the default `sort = relevance` comes with task 93, so the examples set another order.

## Fixtures

### sorting

```text tree
a.txt                      2026-10-05 10:00
b.txt                      2026-10-03 10:00
c.txt                      2026-10-03 10:00
d/e.txt                    2026-10-01 10:00
```

In `tests/fixtures/sorting/`: `a.txt` has one `TODO` (5 bytes), `b.txt` three (15 bytes), `c.txt` and `d/e.txt` two
(10 bytes each), all on their first line. Times are UTC.

## E92.1 The default

Fixture: `sorting`

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
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E92.2 By path

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path desc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r2
#1 text has 'TODO'

d/
e.txt  [#1]  lines: 1  matches: 2
./
c.txt  [#1]  lines: 1  matches: 2
b.txt  [#1]  lines: 1  matches: 3
a.txt  [#1]  lines: 1  matches: 1
```

## E92.3 By modification date

`b.txt` and `c.txt` have the same date, so they are in path order, also with `desc`. A header is written whenever
the directory changes.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "modified desc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "modified asc"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r2
#1 text has 'TODO'

d/
e.txt  [#1]  lines: 1  matches: 2
./
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
a.txt  [#1]  lines: 1  matches: 1
```

## E92.4 By size

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "size asc", "meta": true}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1  file 5B 2026-10-05 10:00 unknown other
c.txt  [#1]  lines: 1  matches: 2  file 10B 2026-10-03 10:00 unknown other
d/
e.txt  [#1]  lines: 1  matches: 2  file 10B 2026-10-01 10:00 unknown other
./
b.txt  [#1]  lines: 1  matches: 3  file 15B 2026-10-03 10:00 unknown other
```

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "size desc", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r2
#1 text has 'TODO'

b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/e.txt  [#1]  lines: 1  matches: 2
a.txt  [#1]  lines: 1  matches: 1
```

## E92.5 By the number of matches

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "matches desc", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

b.txt  [#1]  lines: 1  matches: 3
1:⟦#1:TODO⟧ ⟦#1:TODO⟧ ⟦#1:TODO⟧
c.txt  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ ⟦#1:TODO⟧
d/e.txt  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ ⟦#1:TODO⟧
a.txt  [#1]  lines: 1  matches: 1
1:⟦#1:TODO⟧
```

## E92.6 In JSON

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "matches asc", "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 4, "lines": 4, "matches": 8 },
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "a.txt", "lines": 1, "matches": 1, "conditions": ["#1"] },
    { "path": "c.txt", "lines": 1, "matches": 2, "conditions": ["#1"] },
    { "path": "d/e.txt", "lines": 1, "matches": 2, "conditions": ["#1"] },
    { "path": "b.txt", "lines": 1, "matches": 3, "conditions": ["#1"] }
  ]
}
```

## E92.7 For the session

Fixture: `sorting`

1. `update_config`

```json request update_config
{"set": "output.sort = \"modified asc\""}
```

```text response
[output]
sort = "modified asc"    # was "relevance"
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

d/e.txt  [#1]  lines: 1  matches: 2
b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
a.txt  [#1]  lines: 1  matches: 1
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
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "modified asc"    # update
path_style = "relative"    # default
```

## E92.8 An invalid value in a request

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "name"}}
```

```text error
error[invalid_option]: 'output.sort' must be one of: relevance, path asc, path desc, modified asc, modified desc, size asc, size desc, matches asc, matches desc
```

## E92.9 An invalid value for `update_config`

Fixture: `sorting`

```json request update_config
{"set": "output.sort = \"path\""}
```

```text error
error[invalid_value]: invalid value for 'output.sort'
  hint: expected one of: relevance, path asc, path desc, modified asc, modified desc, size asc, size desc, matches asc, matches desc
```

## E92.10 From the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
sort = "size desc"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

b.txt  [#1]  lines: 1  matches: 3
c.txt  [#1]  lines: 1  matches: 2
d/e.txt  [#1]  lines: 1  matches: 2
a.txt  [#1]  lines: 1  matches: 1
```

## E92.11 An invalid value in the config file

Fixture: `sorting`

```toml conf/o.toml
[output]
sort = "size"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.sort': expected one of: relevance, path asc, path desc, modified asc, modified desc, size asc, size desc, matches asc, matches desc
```
