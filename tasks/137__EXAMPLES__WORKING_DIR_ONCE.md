# Examples for task 137

Spec: `1`

`path_style = in_dir` writes the working directory once and the paths relative to it
(OUTPUT.md#the-working-directory-once).

## Fixtures

### in-dir

```text tree
other/lib.rs              2026-10-01 10:00
proj/data.bin             2026-10-01 10:00
proj/src/cache.rs         2026-10-01 10:00
proj/src/output/json.rs   2026-10-01 10:00
```

File contents are in `tests/fixtures/in-dir/`: `proj/src/cache.rs` has `cache(` in lines 1 and 2,
`proj/src/output/json.rs` in line 1, `other/lib.rs` in line 1; `proj/data.bin` is binary, so a content condition
gives the `binary_skipped` warning. The examples search the absolute working directory `<fixture>/proj`, as an agent
in a git worktree outside the default directory does.

## E137.1 One absolute working directory

Fixture: `in-dir`

1. `relative`, the default: the working directory before every path

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 text has 'cache('

<fixture>/proj/src/
cache.rs  [#1]  lines: 2  matches: 2
1:fn ⟦#1:cache(⟧) {}
2:let x = ⟦#1:cache(⟧);
<fixture>/proj/src/output/
json.rs  [#1]  lines: 1  matches: 1
1:let y = ⟦#1:cache(⟧);

warning: binary_skipped (1): <fixture>/proj/data.bin
```

2. `in_dir`: once

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
in: <fixture>/proj/
#1 text has 'cache('

src/
cache.rs  [#1]  lines: 2  matches: 2
1:fn ⟦#1:cache(⟧) {}
2:let x = ⟦#1:cache(⟧);
src/output/
json.rs  [#1]  lines: 1  matches: 1
1:let y = ⟦#1:cache(⟧);

warning: binary_skipped (1): data.bin
```

3. without directory groups

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "group_dirs": false, "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r3
in: <fixture>/proj/
#1 text has 'cache('

src/cache.rs  [#1]  lines: 2  matches: 2
src/output/json.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

4. `absolute` is unchanged

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "absolute", "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r4
#1 text has 'cache('

<fixture>/proj/src/
cache.rs  [#1]  lines: 2  matches: 2
<fixture>/proj/src/output/
json.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): <fixture>/proj/data.bin
```

## E137.2 In JSON

Fixture: `in-dir`

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files", "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "dirs": ["<fixture>/proj/"],
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "legend": {"#1": "text has 'cache('"},
  "warnings": [{"code": "binary_skipped", "count": 1, "paths": ["data.bin"]}],
  "files": [
    {"path": "src/cache.rs", "lines": 2, "matches": 2, "conditions": ["#1"]},
    {"path": "src/output/json.rs", "lines": 1, "matches": 1, "conditions": ["#1"]}
  ]
}
```

## E137.3 The responses without files

Fixture: `in-dir`

1. the counts with a warning: the line comes for the path of the warning

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "count"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
in: <fixture>/proj/

warning: binary_skipped (1): data.bin
```

2. the counts without a path: no line

```json request
{"dirs": ["<fixture>/proj"], "query": "name ~ '*.rs' and text has 'cache('", "output": {"path_style": "in_dir", "detail": "count"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2
```

3. the facets: the directories relative to it

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "facets"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r3
in: <fixture>/proj/
#1 text has 'cache('
dir:        src/ 1 · src/output/ 1
lang:       rust 2
kind:       source 2
condition:  #1 2

warning: binary_skipped (1): data.bin
```

## E137.4 A relative working directory

Fixture: `in-dir`

1. `proj`

```json request
{"dirs": ["proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "group_dirs": false}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
in: proj/
#1 text has 'cache('

src/cache.rs  [#1]  lines: 2  matches: 2
1:fn ⟦#1:cache(⟧) {}
2:let x = ⟦#1:cache(⟧);
src/output/json.rs  [#1]  lines: 1  matches: 1
1:let y = ⟦#1:cache(⟧);

warning: binary_skipped (1): data.bin
```

2. `.` has no line

```json request
{"query": "name = 'lib.rs'", "output": {"path_style": "in_dir", "detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name = 'lib.rs'

other/
lib.rs  [#1]
```

## E137.5 Several working directories

Fixture: `in-dir`

1. by path: every working directory once

```json request
{"dirs": ["<fixture>/proj", "other"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'cache('

in: <fixture>/proj/
src/
cache.rs  [#1]  lines: 2  matches: 2
src/output/
json.rs  [#1]  lines: 1  matches: 1
in: other/
./
lib.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): <fixture>/proj/data.bin
```

2. by relevance: a line whenever the working directory changes

```json request
{"dirs": ["<fixture>/proj", "other"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r2
#1 text has 'cache('

in: <fixture>/proj/
src/
cache.rs  [#1]  lines: 2  matches: 2
in: other/
./
lib.rs  [#1]  lines: 1  matches: 1
in: <fixture>/proj/
src/output/
json.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): <fixture>/proj/data.bin
```

3. in JSON

```json request
{"dirs": ["<fixture>/proj", "other"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files", "sort": "path asc", "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r3",
  "dirs": ["<fixture>/proj/", "other/"],
  "counts": {"files": 3, "lines": 4, "matches": 4},
  "legend": {"#1": "text has 'cache('"},
  "warnings": [{"code": "binary_skipped", "count": 1, "paths": ["<fixture>/proj/data.bin"]}],
  "files": [
    {"path": "src/cache.rs", "dir": 0, "lines": 2, "matches": 2, "conditions": ["#1"]},
    {"path": "src/output/json.rs", "dir": 0, "lines": 1, "matches": 1, "conditions": ["#1"]},
    {"path": "lib.rs", "dir": 1, "lines": 1, "matches": 1, "conditions": ["#1"]}
  ]
}
```

## E137.6 Left out with `omit`

Fixture: `in-dir`

1. `dir`: the paths stay relative

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "omit": ["dir"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 text has 'cache('

src/
cache.rs  [#1]  lines: 2  matches: 2
1:fn ⟦#1:cache(⟧) {}
2:let x = ⟦#1:cache(⟧);
src/output/
json.rs  [#1]  lines: 1  matches: 1
1:let y = ⟦#1:cache(⟧);

warning: binary_skipped (1): data.bin
```

2. all

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "omit": "all"}}
```
```text response
src/
cache.rs
fn ⟦cache(⟧) {}
let x = ⟦cache(⟧);
src/output/
json.rs
let y = ⟦cache(⟧);
```

3. `dir` in JSON

```json request
{"dirs": ["<fixture>/proj", "other"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files", "sort": "path asc", "format": "json", "omit": ["dir", "legend", "conditions", "warnings"]}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r3",
  "counts": {"files": 3, "lines": 4, "matches": 4},
  "files": [
    {"path": "src/cache.rs", "lines": 2, "matches": 2},
    {"path": "src/output/json.rs", "lines": 1, "matches": 1},
    {"path": "lib.rs", "lines": 1, "matches": 1}
  ]
}
```

## E137.7 `within` and `since`

Fixture: `in-dir`

1. the result

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
in: <fixture>/proj/
#1 text has 'cache('

src/
cache.rs  [#1]  lines: 2  matches: 2
src/output/
json.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

2. of its files, which have `x =`

```json request
{"within": "r1", "query": "text has 'x ='", "output": {"path_style": "in_dir"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r2
in: <fixture>/proj/
#1 text has 'x ='

src/
cache.rs  [#1]  lines: 1  matches: 1
2:let ⟦#1:x =⟧ cache();
```

3. a file is removed

```text setup
remove proj/src/output/json.rs
```

4. the changes since the result

```json request
{"since": "r1", "dirs": ["<fixture>/proj"], "query": "text has 'cache('", "output": {"path_style": "in_dir", "detail": "files"}}
```
```text response
files: 1  lines: 2  matches: 2  result: r3  since: r1  unchanged: 1
in: <fixture>/proj/
#1 text has 'cache('

- src/output/json.rs

warning: binary_skipped (1): data.bin
```

## E137.8 Wrong values

Fixture: `in-dir`

1. a request

```json request
{"query": "text has 'cache('", "output": {"path_style": "once"}}
```
```text error
error[invalid_option]: 'output.path_style' must be one of: relative, absolute, in_dir
```

2. an unknown part of `omit`: the list names `dir`

```json request
{"query": "text has 'cache('", "output": {"omit": ["dirs"]}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out)
```
