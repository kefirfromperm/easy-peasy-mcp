# Examples for task 130

Only the matches of some groups of the query ([OUTPUT.md#shown-groups](../OUTPUT.md#shown-groups)) and only the
matched text ([OUTPUT.md#only-the-matches](../OUTPUT.md#only-the-matches)). The examples are written for the current
spec version, 2: the legend, the conditions of the files and the ids of the matches follow its rules
([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)). They set `sort = "path asc"` and use the fixture
`query-groups` of task 128.

## Fixtures

### query-groups

```text tree
Cargo.toml                 2026-10-01 10:00
lib.rs                     2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
src/c.rs                   2026-10-01 10:00
```

In `tests/fixtures/query-groups/`:

- `src/a.rs`: `use serde::Deserialize;` (line 1), `fn main() {` (line 3), `    let apple = 1;` (line 4)
- `src/b.rs`: `use serde_json::Value;` (line 1), `// apple pie` (line 2)
- `src/c.rs`: `use serde::Serialize;`, no `apple`
- `lib.rs`: `pub fn load_config(path: &Path) -> Config {` (line 1), `    parse(path)` (line 2),
  `fn parse(path: &Path) -> Config {` (line 5), `    todo!()` (line 6)
- `Cargo.toml`: a `[package]`

## E130.1 Only the `serde` lines

`apple` chooses the files, only the matches of `@ser` are shown and counted; `files` is the number of matched files.
The list has one reference, so the matches have no ids. In `b.rs` the `serde` and `serde_json` matches overlap and are
marked as one part.

Fixture: `query-groups`

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"groups": ["@ser"], "sort": "path asc"}}
```

```text response
files: 2  lines: 2  matches: 3  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'

src/
a.rs  [@ser #1 #3]  lines: 1  matches: 1
1:use ⟦serde⟧::Deserialize;
b.rs  [@ser #1 #2 #3]  lines: 1  matches: 2
1:use ⟦serde_json⟧::Value;
```

## E130.2 Two references: the ids of the matches

A match has only the references of the list: the `serde` match of `#1` is not marked with `@ser`, the `serde_json`
match of `#2` is not shown.

Fixture: `query-groups`

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"groups": ["#1", "#3"], "sort": "path asc"}}
```

```text response
files: 2  lines: 4  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'

src/
a.rs  [@ser #1 #3]  lines: 2
1:use ⟦#1:serde⟧::Deserialize;
⇥4
4:let ⟦#3:apple⟧ = 1;
b.rs  [@ser #1 #2 #3]  lines: 2
1:use ⟦#1:serde⟧_json::Value;
2:// ⟦#3:apple⟧ pie
```

## E130.3 Only the names of the functions

A named capture group of a regex is a reference; `only_match` writes its text with the number of its line, like
`grep -o`.

Fixture: `query-groups`

```json request
{"query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": {"groups": ["@def.fname"], "only_match": true, "sort": "path asc"}}
```

```text response
files: 2  lines: 3  result: r1

./
lib.rs  lines: 2
1:load_config
5:parse
src/
a.rs  lines: 1
3:main
```

## E130.4 A group and its sub-group, with markers

Two references: every match has its ids, also with one condition in the legend. The matches of a line are written in
the order of their start.

Fixture: `query-groups`

```json request
{"query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": {"groups": ["@def", "@def.fname"], "only_match": true, "highlight": "markers", "sort": "path asc"}}
```

```text response
files: 2  lines: 3  matches: 6  result: r1

./
lib.rs  lines: 2  matches: 4
1:⟦@def:fn load_config(⟧
1:⟦@def.fname:load_config⟧
5:⟦@def:fn parse(⟧
5:⟦@def.fname:parse⟧
src/
a.rs  lines: 1  matches: 2
3:⟦@def:fn main(⟧
3:⟦@def.fname:main⟧
```

## E130.5 Only the matches in JSON

The file has `fragments` instead of `blocks`, every one with the number of its line; `col` counts in the line.

Fixture: `query-groups`

```json request
{"query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": {"groups": ["@def.fname"], "only_match": true, "format": "json", "sort": "path asc"}}
```

```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "files": [
    {"path": "lib.rs", "lines": 2, "matches": 2, "fragments": [
      {"n": 1, "col": [8, 19], "text": "load_config"},
      {"n": 5, "col": [4, 9], "text": "parse"}
    ]},
    {"path": "src/a.rs", "lines": 1, "matches": 1, "fragments": [
      {"n": 3, "col": [4, 8], "text": "main"}
    ]}
  ]
}
```

## E130.6 A group inside `line( ... )`

The lines are the ones of `line( ... )`, the shown matches the `load` and `parse` of `@call`, not the `fn`.

Fixture: `query-groups`

```json request
{"query": "line((text has 'load' or text has 'parse') @call and text has 'fn')", "output": {"groups": ["@call"]}}
```

```text response
files: 1  lines: 2  result: r1

./
lib.rs  lines: 2
1:pub fn ⟦load⟧_config(path: &Path) -> Config {
5:fn ⟦parse⟧(path: &Path) -> Config {
```

## E130.7 One side of `near( ... )`

`parse` in line 5 is near `todo` in line 6 and makes the file match; only the `todo` side is shown.

Fixture: `query-groups`

```json request
{"query": "near(text has 'parse' @p, text has 'todo' @t, 1)", "output": {"groups": ["@t"]}}
```

```text response
files: 1  lines: 1  result: r1

./
lib.rs  lines: 1
⇥4
6:⟦todo⟧!()
```

## E130.8 A file without a shown match

`b.rs` and `c.rs` match through `@s` and have no match of `@f`: they are listed without counts. `count` detail counts
the shown matches too.

Fixture: `query-groups`

1. `files` detail

```json request
{"query": "text has 'serde' @s or text has 'fn' @f", "output": {"groups": ["@f"], "detail": "files", "sort": "path asc"}}
```

```text response
files: 4  lines: 3  result: r1
@s text has 'serde'
@f text has 'fn'

./
lib.rs  [@f]  lines: 2
src/
a.rs  [@s @f]  lines: 1
b.rs  [@s]
c.rs  [@s]
```

2. `count` detail

```json request
{"query": "text has 'serde' @s or text has 'fn' @f", "output": {"groups": ["@f"], "detail": "count"}}
```

```text response
files: 4  lines: 3  result: r2
```

## E130.9 `max_matches` and the cursor count the shown matches

The `fn` matches of `lib.rs` and `a.rs` are not shown, so they don't count in `max_matches`: the first page lists the
`serde` matches of `a.rs` and `b.rs`, the next one starts at `c.rs`. The totals of the cursor are the shown ones.

Fixture: `query-groups`

1. the first page

```json request
{"query": "text has 'serde' @s or text has 'fn' @f", "output": {"groups": ["@s"], "sort": "path asc", "max_matches": 2}}
```

```text response
files: 4  lines: 3  result: r1  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiNWEyNDNhZTAiLCJmIjozLCJ0IjpbNCwzLDNdfQ
@s text has 'serde'
@f text has 'fn'

./
lib.rs  [@f]
src/
a.rs  [@s @f]  lines: 1
1:use ⟦serde⟧::Deserialize;
b.rs  [@s]  lines: 1
1:use ⟦serde⟧_json::Value;
c.rs  [@s]  lines: 1
```

2. the next page: no `results_changed` warning

```json request
{"query": "text has 'serde' @s or text has 'fn' @f", "output": {"groups": ["@s"], "sort": "path asc", "max_matches": 2, "cursor": "eyJ2IjoxLCJoIjoiNWEyNDNhZTAiLCJmIjozLCJ0IjpbNCwzLDNdfQ"}}
```

```text response
files: 4  lines: 3  result: r2
@s text has 'serde'
@f text has 'fn'

src/
c.rs  [@s]  lines: 1
1:use ⟦serde⟧::Serialize;
```

## E130.10 The budget estimates the shown matches

The response of E130.1 has 302 characters, 98 tokens of 3.1 characters: it fits into the budget of 100 as requested.
Without `groups` the response has 366 characters, 119 tokens, and the budget would lower the detail.

Fixture: `query-groups`

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"groups": ["@ser"], "sort": "path asc", "budget": 100, "chars_per_token": 3.1}}
```

```text response
files: 2  lines: 2  matches: 3  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'

src/
a.rs  [@ser #1 #3]  lines: 1  matches: 1
1:use ⟦serde⟧::Deserialize;
b.rs  [@ser #1 #2 #3]  lines: 1  matches: 2
1:use ⟦serde_json⟧::Value;
```

## E130.11 An unknown group

The references are checked in the order of the list; the position is in the reference.

Fixture: `query-groups`

1. text

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"groups": ["@ser", "@x"]}}
```

```text error
error[unknown_group]: unknown group '@x'
  @x
  ^^
  hint: the groups of the query: @ser, #1, #2, #3
```

2. a capture group which the regex doesn't have, in JSON

```json request
{"query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": {"groups": ["@def.2"], "format": "json"}}
```

```json error
{
  "error": {
    "code": "unknown_group",
    "message": "unknown group '@def.2'",
    "output.groups": "@def.2",
    "position": {"col": 1, "len": 6},
    "hint": "the groups of the query: @def, @def.1, @def.fname"
  }
}
```

## E130.12 Not a list of references

Fixture: `query-groups`

1. a string

```json request
{"query": "text has 'serde'", "output": {"groups": "@ser"}}
```

```text error
error[invalid_option]: 'output.groups' must be a list of group references
```

2. `only_match` not a boolean

```json request
{"query": "text has 'serde'", "output": {"only_match": "yes"}}
```

```text error
error[invalid_option]: 'output.only_match' must be true or false
```

## E130.13 The configuration

`get_config` shows both keys; a configured list applies to every query, so a query without the group is
`unknown_group`; the list of a request replaces it.

Fixture: `query-groups`

1. the defaults

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
max_files = 20
max_lines = 100
max_matches = 200
max_matches_per_file = 10
max_line_length = 300
max_output_per_file = 800
max_output = 8000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
repeated_locations = 3
sort = "relevance"
path_style = "relative"
omit = ['warnings']
show = []
```

2. `update_config`

```json request update_config
{"set": "output.groups = ['@ser']\noutput.only_match = true"}
```

```text response
[output]
groups = ['@ser']    # was []
only_match = true    # was false
```

3. a query with the group

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 2  matches: 3  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'

src/
a.rs  [@ser #1 #3]  lines: 1  matches: 1
1:serde
b.rs  [@ser #1 #2 #3]  lines: 1  matches: 2
1:serde
1:serde_json
```

4. a query without it

```json request
{"query": "text has 'apple' @a"}
```

```text error
error[unknown_group]: unknown group '@ser'
  @ser
  ^^^^
  hint: the groups of the query: @a
```

5. the list of the request replaces it

```json request
{"query": "text has 'apple' @a", "output": {"groups": ["@a"], "sort": "path asc"}}
```

```text response
files: 2  lines: 2  result: r2

src/
a.rs  lines: 1
4:apple
b.rs  lines: 1
2:apple
```

6. an invalid value

```json request update_config
{"set": "output.groups = 3"}
```

```text error
error[invalid_value]: invalid value for 'output.groups'
  hint: a list of group references is expected
```
