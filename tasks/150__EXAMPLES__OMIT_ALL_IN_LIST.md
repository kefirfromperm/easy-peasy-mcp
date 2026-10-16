# Examples for task 150

A list with `"all"` among its items is the list form of `"all"` in `output.omit` and `output.show`
(OUTPUT.md#omit, OUTPUT.md#defaults-of-spec-2, CONFIG.md#output): in a request, a query of a batch, the config file and
`update_config`, in every spec version. An unknown part among the items is still an error.

## Fixtures

The examples use the fixture `omit` of task 134 (`src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs` with
`unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`).

## E150.1 `omit = ["all"]`

Fixture: `omit`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["all"]}}
```
```text response
src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
main.rs
let cfg = load().⟦unwrap(⟧);
```

## E150.2 `show = ["all"]`

Fixture: `omit`

```json request
{"query": "text has 'unwrap('", "output": {"show": ["all"]}}
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

## E150.3 `"all"` among other parts

Fixture: `omit`

1. `omit = ["legend", "all"]`: every part

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["legend", "all"], "detail": "files"}}
```
```text response
src/
lib.rs
main.rs
```

2. `show = ["all", "legend"]` against `omit = "all"`: every part comes back

```json request
{"query": "text has 'unwrap('", "output": {"omit": "all", "show": ["all", "legend"], "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

## E150.4 The queries of a batch

Fixture: `omit`

The `omit` of an item is added to the shared one: `["result"]` and `["all"]` are `"all"`.

```json request
{"output": {"detail": "files", "omit": ["result"]}, "queries": [{"label": "plain", "query": "text has 'unwrap('"}, {"label": "bare", "query": "text has 'unwrap('", "output": {"omit": ["all"]}}]}
```
```text response
== plain ==  files: 2  lines: 3

src/
lib.rs  lines: 2
main.rs  lines: 1

warnings left out: binary_skipped (1)
== bare ==  src/
lib.rs
main.rs
```

## E150.5 `update_config`

Fixture: `omit`

1. the list is `"all"`

```json request update_config
{"set": "output.omit = ['all']"}
```
```text response
[output]
omit = "all"    # was ['warnings']
```

2. every part is left out

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
src/
lib.rs
main.rs
```

3. `show` too

```json request update_config
{"set": "output.show = ['all', 'legend']"}
```
```text response
[output]
show = "all"    # was []
```

## E150.6 The config file

Fixture: `omit`

```toml conf/o.toml
[output]
omit = ["all", "legend"]
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
max_files = 20    # default
max_lines = 100    # default
max_matches = 200    # default
max_matches_per_file = 10    # default
max_line_length = 300    # default
max_output_per_file = 800    # default
max_output = 8000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
repeated_locations = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = "all"    # file: conf/o.toml
show = []    # default
```

2. `find`: every part is left out

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
src/
lib.rs
main.rs
```

## E150.7 `"version": 1`

Fixture: `omit`

```json request
{"version": 1, "query": "text has 'unwrap('", "output": {"omit": ["all"]}}
```
```text response
src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
main.rs
let cfg = load().⟦unwrap(⟧);
```

## E150.8 Wrong values

Fixture: `omit`

An unknown part is an error, with `"all"` among the items too.

1. `omit`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["all", "colour"]}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out)
```

2. `show` in `update_config`

```json request update_config
{"set": "output.show = ['all', 'colour']"}
```
```text error
error[invalid_value]: invalid value for 'output.show'
  hint: "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out) is expected
```
