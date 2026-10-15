# Examples for task 141

The defaults of spec 2 (OUTPUT.md#defaults-of-spec-2): the warnings left out with a line which says so, the legend,
the conditions and the ids of the matches only with more than one condition, the gaps only with context lines;
`output.show` brings parts back, the `omit` of a request is added to the configured one; `"version": 1` keeps the
defaults of spec 1. The examples are written for the current spec version 2.

## Fixtures

### gaps

```text tree
far.rs             2026-10-01 10:00
```

File contents are in `tests/fixtures/gaps/`: `far.rs` has `unwrap(` in lines 1 and 6.

The other examples use the fixtures `omit` of task 134 (`src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs`
with `unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`) and `timeout` of
task 40.

## E141.1 One condition, a hit

Fixture: `omit`

1. the defaults of spec 2

```json request
{"query": "text has 'unwrap('"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warnings left out: binary_skipped (1)
```

2. the defaults of spec 1

```json request
{"version": 1, "query": "text has 'unwrap('"}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

## E141.2 Several conditions

Fixture: `omit`

The legend explains the ids, so it comes with the conditions of the files and the ids of the matches; without context
lines the line numbers show the gap between the lines 2 and 4.

```json request
{"query": "text has 'unwrap(' or text has 'TODO'"}
```
```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'unwrap('
#2 text has 'TODO'

src/
main.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);
3:// ⟦#2:TODO⟧ handle the error
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
4:parse(&text).⟦#1:unwrap(⟧)

warnings left out: binary_skipped (1)
```

## E141.3 No hit

Fixture: `omit`

1. spec 2

```json request
{"query": "text has 'zzz'"}
```
```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  text has 'zzz'                         0

warnings left out: binary_skipped (1)
```

2. spec 1

```json request
{"version": 1, "query": "text has 'zzz'"}
```
```text response
files: 0  lines: 0  matches: 0  result: r2
funnel:
  text has 'zzz'                         0

warning: binary_skipped (1): data.bin
```

## E141.4 The list of the files

Fixture: `omit`

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
```

## E141.5 JSON

Fixture: `omit`

1. spec 2

```json request
{"query": "text has 'unwrap('", "output": {"format": "json"}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 2, "blocks": [
      {"indent": 4, "lines": [{"n": 2, "text": "let raw = read().unwrap();", "fragments": [{"col": [22, 29], "text": "unwrap("}]}]},
      {"indent": 4, "lines": [{"n": 4, "text": "parse(&text).unwrap()", "fragments": [{"col": [18, 25], "text": "unwrap("}]}]}
    ]},
    {"path": "src/main.rs", "lines": 1, "matches": 1, "blocks": [
      {"indent": 4, "lines": [{"n": 2, "text": "let cfg = load().unwrap();", "fragments": [{"col": [22, 29], "text": "unwrap("}]}]}
    ]}
  ],
  "left_out": {"warnings": {"binary_skipped": 1}}
}
```

2. spec 1

```json request
{"version": 1, "query": "text has 'unwrap('", "output": {"format": "json", "detail": "files"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "legend": {"#1": "text has 'unwrap('"},
  "warnings": [{"code": "binary_skipped", "count": 1, "paths": ["data.bin"]}],
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 2, "conditions": ["#1"]},
    {"path": "src/main.rs", "lines": 1, "matches": 1, "conditions": ["#1"]}
  ]
}
```

## E141.6 A part back with `show`

Fixture: `omit`

1. the warnings; the other defaults stay

```json request
{"query": "text has 'unwrap('", "output": {"show": ["warnings"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warning: binary_skipped (1): data.bin
```

2. the legend of one condition

```json request
{"query": "text has 'unwrap('", "output": {"show": ["legend"], "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
#1 text has 'unwrap('

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
```

3. all: the output of spec 1

```json request
{"query": "text has 'unwrap('", "output": {"show": "all"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r3
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

## E141.7 The `omit` of a request adds to the defaults

Fixture: `omit`

1. spec 2: the warnings stay left out; without line numbers the gaps are written

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["line_numbers"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
let raw = read().⟦unwrap(⟧);
--
parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
let cfg = load().⟦unwrap(⟧);

warnings left out: binary_skipped (1)
```

2. spec 1: the list replaces the default one

```json request
{"version": 1, "query": "text has 'unwrap('", "output": {"omit": ["line_numbers"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
let raw = read().⟦#1:unwrap(⟧);
--
⇥4
parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

## E141.8 The gaps with context lines

Fixture: `gaps`

1. without context lines the line numbers show the gap

```json request
{"query": "text has 'unwrap('"}
```
```text response
files: 1  lines: 2  matches: 2  result: r1

./
far.rs  lines: 2  matches: 2
1:a.⟦unwrap(⟧);
6:f.⟦unwrap(⟧);
```

2. with context lines `--` ends a block

```json request
{"query": "text has 'unwrap('", "output": {"context": 1}}
```
```text response
files: 1  lines: 2  matches: 2  result: r2

./
far.rs  lines: 2  matches: 2
1:a.⟦unwrap(⟧);
2-b
--
5-e
6:f.⟦unwrap(⟧);
```

## E141.9 The line of the omitted warnings left out

Fixture: `omit`

1. text

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["left_out"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);
```

2. JSON

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["left_out"], "format": "json", "detail": "count"}}
```
```json response
{"version": 2, "exists": true, "result": "r2", "counts": {"files": 2, "lines": 3, "matches": 3}}
```

3. all

```json request
{"query": "text has 'unwrap('", "output": {"omit": "all"}}
```
```text response
src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
main.rs
let cfg = load().⟦unwrap(⟧);
```

## E141.10 A timeout is never hidden

Fixture: `timeout`

Slow reads: `2s`

The header says that the search stopped, and the line of the omitted warnings names the code.

```json request
{"timeout": "200ms", "query": "type = dir or text has 'x'"}
```
```text response
files: 1  lines: 0  matches: 0  result: r1  truncated: yes (timeout)

./
sub/

warnings left out: timeout
```

## E141.11 The defaults in the config file

Fixture: `omit`

```toml conf/o.toml
[output]
show = "all"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

1. `get_config`

```json request get_config
{"section": "output", "sources": true}
```
```text response
# easy-peasy-mcp <version>, spec 2, supported specs: 1, 2
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
repeated_locations = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = ['warnings']    # default
show = "all"    # file: conf/o.toml
```

2. `find`: every part

```json request
{"query": "text has 'unwrap('"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

## E141.12 The defaults for the session

Fixture: `omit`

1. `update_config`

```json request update_config
{"set": "output.omit = []"}
```
```text response
[output]
omit = []    # was ['warnings']
```

2. the warnings come back, the rules stay

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

3. `get_config`

```json request get_config
{"section": "output"}
```
```text response
# easy-peasy-mcp <version>, spec 2, supported specs: 1, 2
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

## E141.13 The queries of a batch

Fixture: `omit`

Every item adds its `omit` to the shared one and has its own `show`.

```json request
{"output": {"detail": "files", "omit": ["result"]}, "queries": [{"label": "plain", "query": "text has 'unwrap('"}, {"label": "bare", "query": "text has 'unwrap('", "output": {"omit": ["file_counts"]}}, {"label": "full", "query": "text has 'unwrap('", "output": {"show": "all"}}]}
```
```text response
== plain ==  files: 2  lines: 3  matches: 3

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
== bare ==  files: 2  lines: 3  matches: 3

src/
lib.rs
main.rs

warnings left out: binary_skipped (1)
== full ==  files: 2  lines: 3  matches: 3  result: r3
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

## E141.14 Wrong values

Fixture: `omit`

1. `show`

```json request
{"query": "text has 'unwrap('", "output": {"show": "none"}}
```
```text error
error[invalid_option]: 'output.show' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out)
```

2. `show` in `update_config`

```json request update_config
{"set": "output.show = ['colour']"}
```
```text error
error[invalid_value]: invalid value for 'output.show'
  hint: "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out) is expected
```

3. a spec version which doesn't exist

```json request
{"version": 3, "query": "text has 'unwrap('"}
```
```text error
error[unsupported_version]: the spec version 3 is not supported
  hint: supported spec versions: 1, 2
```
