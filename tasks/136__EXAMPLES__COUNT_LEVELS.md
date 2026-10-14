# Examples for task 136

Spec: `1`

`output.count_by` sets the grain of the counts of `detail = count`: the totals, the counts of every file, or of every
matched line too (OUTPUT.md#counts-per-file-and-per-line).

## Fixtures

### count-by

```text tree
docs/notes.md      2026-10-01 10:00
src/lib.rs         2026-10-01 10:00
src/main.rs        2026-10-01 10:00
tests/load.rs      2026-10-01 10:00
```

File contents are in `tests/fixtures/count-by/`: `src/lib.rs` has `unwrap(` twice in line 2 and once in line 4,
`src/main.rs` once in line 2, `tests/load.rs` once in lines 3, 4 and 5; `docs/notes.md` is `begin`, `end`, `begin`,
`end`. With `sort = relevance` the files of `text has 'unwrap('` are in the order `src/lib.rs`, `tests/load.rs`,
`src/main.rs`.

### count-by-many

```text tree
many/f01.txt       2026-10-01 10:00
many/f02.txt       2026-10-01 10:00
many/f03.txt       2026-10-01 10:00
many/f04.txt       2026-10-01 10:00
many/f05.txt       2026-10-01 10:00
many/f06.txt       2026-10-01 10:00
many/f07.txt       2026-10-01 10:00
many/f08.txt       2026-10-01 10:00
many/f09.txt       2026-10-01 10:00
many/f10.txt       2026-10-01 10:00
many/f11.txt       2026-10-01 10:00
many/f12.txt       2026-10-01 10:00
many/f13.txt       2026-10-01 10:00
many/f14.txt       2026-10-01 10:00
many/f15.txt       2026-10-01 10:00
many/f16.txt       2026-10-01 10:00
many/f17.txt       2026-10-01 10:00
many/f18.txt       2026-10-01 10:00
many/f19.txt       2026-10-01 10:00
many/f20.txt       2026-10-01 10:00
many/f21.txt       2026-10-01 10:00
many/f22.txt       2026-10-01 10:00
many/f23.txt       2026-10-01 10:00
many/f24.txt       2026-10-01 10:00
```

```text setup
file many/f01.txt 2
file many/f02.txt 2
file many/f03.txt 2
file many/f04.txt 2
file many/f05.txt 2
file many/f06.txt 2
file many/f07.txt 2
file many/f08.txt 2
file many/f09.txt 2
file many/f10.txt 2
file many/f11.txt 2
file many/f12.txt 2
file many/f13.txt 2
file many/f14.txt 2
file many/f15.txt 2
file many/f16.txt 2
file many/f17.txt 2
file many/f18.txt 2
file many/f19.txt 2
file many/f20.txt 2
file many/f21.txt 2
file many/f22.txt 2
file many/f23.txt 2
file many/f24.txt 2
```

Twenty-four files `many/f01.txt` … `many/f24.txt`, each `xx`: one line with two matches of `text has 'x'`.

## E136.1 The grains of one query

Fixture: `count-by`

1. the totals, the default

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r1
```

2. per file

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r2

src/
lib.rs  lines: 2  matches: 3
tests/
load.rs  lines: 3  matches: 3
src/
main.rs  lines: 1  matches: 1
```

3. per line

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r3

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
tests/
load.rs  lines: 3  matches: 3
  3: 1
  4: 1
  5: 1
src/
main.rs  lines: 1  matches: 1
  2: 1
```

4. the lines themselves: `detail = lines` ignores `count_by`

```json request
{"query": "text has 'unwrap('", "output": {"count_by": "line"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r4
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 3
⇥4
2:let raw = read().⟦#1:unwrap(⟧).⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
tests/
load.rs  [#1]  lines: 3  matches: 3
⇥4
3:load().⟦#1:unwrap(⟧);
4:load().⟦#1:unwrap(⟧);
5:load().⟦#1:unwrap(⟧);
src/
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);
```

## E136.2 The grains in JSON

Fixture: `count-by`

1. the totals

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "format": "json"}}
```
```json response
{"version": 1, "exists": true, "result": "r1", "counts": {"files": 3, "lines": 6, "matches": 7}}
```

2. per file

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": {"files": 3, "lines": 6, "matches": 7},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 3},
    {"path": "tests/load.rs", "lines": 3, "matches": 3},
    {"path": "src/main.rs", "lines": 1, "matches": 1}
  ]
}
```

3. per line

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r3",
  "counts": {"files": 3, "lines": 6, "matches": 7},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 3,
      "matched_lines": [{"n": 2, "matches": 2}, {"n": 4, "matches": 1}]},
    {"path": "tests/load.rs", "lines": 3, "matches": 3,
      "matched_lines": [{"n": 3, "matches": 1}, {"n": 4, "matches": 1}, {"n": 5, "matches": 1}]},
    {"path": "src/main.rs", "lines": 1, "matches": 1, "matched_lines": [{"n": 2, "matches": 1}]}
  ]
}
```

## E136.3 Per file is the file list without the conditions

Fixture: `count-by`

`detail = files` with the legend and the conditions left out is the response of `count_by = file`.

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files", "omit": ["legend", "conditions"]}}
```
```text response
files: 3  lines: 6  matches: 7  result: r1

src/
lib.rs  lines: 2  matches: 3
tests/
load.rs  lines: 3  matches: 3
src/
main.rs  lines: 1  matches: 1
```

## E136.4 The parts of `omit`

Fixture: `count-by`

1. `counts`

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "omit": ["counts"]}}
```
```text response
result: r1

src/
lib.rs  lines: 2  matches: 3
tests/
load.rs  lines: 3  matches: 3
src/
main.rs  lines: 1  matches: 1
```

2. `file_counts`: the paths alone

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "omit": ["file_counts"]}}
```
```text response
files: 3  lines: 6  matches: 7  result: r2

src/
lib.rs
tests/
load.rs
src/
main.rs
```

3. `line_numbers`: the counts of the lines

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": ["line_numbers"]}}
```
```text response
files: 3  lines: 6  matches: 7  result: r3

src/
lib.rs  lines: 2  matches: 3
  2
  1
tests/
load.rs  lines: 3  matches: 3
  1
  1
  1
src/
main.rs  lines: 1  matches: 1
  1
```

4. `line_counts`: the matched line numbers

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": ["line_counts"]}}
```
```text response
files: 3  lines: 6  matches: 7  result: r4

src/
lib.rs  lines: 2  matches: 3
  2
  4
tests/
load.rs  lines: 3  matches: 3
  3
  4
  5
src/
main.rs  lines: 1  matches: 1
  2
```

5. both: no lines are written

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": ["line_numbers", "line_counts"]}}
```
```text response
files: 3  lines: 6  matches: 7  result: r5

src/
lib.rs  lines: 2  matches: 3
tests/
load.rs  lines: 3  matches: 3
src/
main.rs  lines: 1  matches: 1
```

6. all

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": "all"}}
```
```text response
src/
lib.rs
tests/
load.rs
src/
main.rs
```

7. the line numbers in JSON, without the counts of the files

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": ["line_counts", "file_counts"], "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r7",
  "counts": {"files": 3, "lines": 6, "matches": 7},
  "files": [
    {"path": "src/lib.rs", "matched_lines": [{"n": 2}, {"n": 4}]},
    {"path": "tests/load.rs", "matched_lines": [{"n": 3}, {"n": 4}, {"n": 5}]},
    {"path": "src/main.rs", "matched_lines": [{"n": 2}]}
  ]
}
```

8. the counts of the lines in JSON

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": ["line_numbers"], "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r8",
  "counts": {"files": 3, "lines": 6, "matches": 7},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 3, "matched_lines": [{"matches": 2}, {"matches": 1}]},
    {"path": "tests/load.rs", "lines": 3, "matches": 3, "matched_lines": [{"matches": 1}, {"matches": 1}, {"matches": 1}]},
    {"path": "src/main.rs", "lines": 1, "matches": 1, "matched_lines": [{"matches": 1}]}
  ]
}
```

9. all in JSON

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "omit": "all", "format": "json"}}
```
```json response
{"version": 1, "exists": true, "files": [{"path": "src/lib.rs"}, {"path": "tests/load.rs"}, {"path": "src/main.rs"}]}
```

## E136.5 A multiline match and a `line( ... )` group

Fixture: `count-by`

1. a multiline match is in its first line: the line it continues into has `0`

```json request
{"query": "text =~ `begin\\nend` /m", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 1  lines: 4  matches: 2  result: r1

docs/
notes.md  lines: 4  matches: 2
  1: 1
  2: 0
  3: 1
  4: 0
```

2. the same in JSON

```json request
{"query": "text =~ `begin\\nend` /m", "output": {"detail": "count", "count_by": "line", "format": "json"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": {"files": 1, "lines": 4, "matches": 2},
  "files": [
    {"path": "docs/notes.md", "lines": 4, "matches": 2,
      "matched_lines": [{"n": 1, "matches": 1}, {"n": 2, "matches": 0}, {"n": 3, "matches": 1}, {"n": 4, "matches": 0}]}
  ]
}
```

3. a group without text conditions: the whole line is its match

```json request
{"query": "name = 'lib.rs' and line(len > 25)", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 1  lines: 2  matches: 2  result: r3

src/
lib.rs  lines: 2  matches: 2
  2: 1
  3: 1
```

## E136.6 Without directory groups, sorted

Fixture: `count-by`

1. by path

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "group_dirs": false, "sort": "path asc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r1

src/lib.rs  lines: 2  matches: 3
src/main.rs  lines: 1  matches: 1
tests/load.rs  lines: 3  matches: 3
```

2. the most matches first

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "group_dirs": false, "sort": "matches desc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r2

src/lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
tests/load.rs  lines: 3  matches: 3
  3: 1
  4: 1
  5: 1
src/main.rs  lines: 1  matches: 1
  2: 1
```

## E136.7 The list cut by the limits

Fixture: `count-by`

The totals are the real ones; a list cut by `max_files`, `max_lines` or `max_matches` has a cursor.

1. `max_files`

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "max_files": 2, "sort": "path asc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiZWU1MDRkZTciLCJmIjoyLCJ0IjpbMyw2LDddfQ

src/
lib.rs  lines: 2  matches: 3
main.rs  lines: 1  matches: 1
```

2. the next page

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file", "max_files": 2, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiZWU1MDRkZTciLCJmIjoyLCJ0IjpbMyw2LDddfQ"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r2

tests/
load.rs  lines: 3  matches: 3
```

3. `max_lines`: the next page starts at the first line which is not listed

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "max_lines": 4, "sort": "path asc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r3  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiM2UyY2FiYTUiLCJmIjoyLCJtIjoxLCJ0IjpbMyw2LDddfQ

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
main.rs  lines: 1  matches: 1
  2: 1
tests/
load.rs  lines: 3  matches: 3
  3: 1
```

4. `max_matches_per_file` cuts every file by design, without a cursor

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "max_matches_per_file": 2, "sort": "path asc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r4  truncated: yes (max_matches_per_file)

src/
lib.rs  lines: 2  matches: 3  (cut)
  2: 2
main.rs  lines: 1  matches: 1
  2: 1
tests/
load.rs  lines: 3  matches: 3  (cut)
  3: 1
  4: 1
```

5. `max_matches`: a line shows the matches which are listed, the files after it have no lines

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "max_matches": 1, "sort": "path asc"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r5  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiOTljNmZmZmUiLCJmIjowLCJtIjoxLCJ0IjpbMyw2LDddfQ

src/
lib.rs  lines: 2  matches: 3
  2: 1
main.rs  lines: 1  matches: 1
tests/
load.rs  lines: 3  matches: 3
```

6. its next page: the other match of the line

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "max_matches": 1, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiOTljNmZmZmUiLCJmIjowLCJtIjoxLCJ0IjpbMyw2LDddfQ"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r6  truncated: yes (max_matches)  next cursor: eyJ2IjoxLCJoIjoiOTljNmZmZmUiLCJmIjowLCJtIjoyLCJ0IjpbMyw2LDddfQ

src/
lib.rs  lines: 2  matches: 3
  2: 1
main.rs  lines: 1  matches: 1
tests/
load.rs  lines: 3  matches: 3
```

## E136.8 The budget

Fixture: `count-by-many`

The counts per line of the 24 files take 941 characters (236 tokens), the counts per file 827 with the mark of the
budget (207 tokens); the budget lowers `count_by = line` to `file`, then cuts the list of the files, then leaves the
totals.

1. as requested

```json request
{"query": "text has 'x'", "output": {"detail": "count", "count_by": "line", "budget": 240, "sort": "path asc"}}
```
```text response
files: 24  lines: 24  matches: 48  result: r1

many/
f01.txt  lines: 1  matches: 2
  1: 2
f02.txt  lines: 1  matches: 2
  1: 2
f03.txt  lines: 1  matches: 2
  1: 2
f04.txt  lines: 1  matches: 2
  1: 2
f05.txt  lines: 1  matches: 2
  1: 2
f06.txt  lines: 1  matches: 2
  1: 2
f07.txt  lines: 1  matches: 2
  1: 2
f08.txt  lines: 1  matches: 2
  1: 2
f09.txt  lines: 1  matches: 2
  1: 2
f10.txt  lines: 1  matches: 2
  1: 2
f11.txt  lines: 1  matches: 2
  1: 2
f12.txt  lines: 1  matches: 2
  1: 2
f13.txt  lines: 1  matches: 2
  1: 2
f14.txt  lines: 1  matches: 2
  1: 2
f15.txt  lines: 1  matches: 2
  1: 2
f16.txt  lines: 1  matches: 2
  1: 2
f17.txt  lines: 1  matches: 2
  1: 2
f18.txt  lines: 1  matches: 2
  1: 2
f19.txt  lines: 1  matches: 2
  1: 2
f20.txt  lines: 1  matches: 2
  1: 2
f21.txt  lines: 1  matches: 2
  1: 2
f22.txt  lines: 1  matches: 2
  1: 2
f23.txt  lines: 1  matches: 2
  1: 2
f24.txt  lines: 1  matches: 2
  1: 2
```

2. per file

```json request
{"query": "text has 'x'", "output": {"detail": "count", "count_by": "line", "budget": 210, "sort": "path asc"}}
```
```text response
files: 24  lines: 24  matches: 48  result: r2  count by: file (budget 210)  truncated: yes (budget)

many/
f01.txt  lines: 1  matches: 2
f02.txt  lines: 1  matches: 2
f03.txt  lines: 1  matches: 2
f04.txt  lines: 1  matches: 2
f05.txt  lines: 1  matches: 2
f06.txt  lines: 1  matches: 2
f07.txt  lines: 1  matches: 2
f08.txt  lines: 1  matches: 2
f09.txt  lines: 1  matches: 2
f10.txt  lines: 1  matches: 2
f11.txt  lines: 1  matches: 2
f12.txt  lines: 1  matches: 2
f13.txt  lines: 1  matches: 2
f14.txt  lines: 1  matches: 2
f15.txt  lines: 1  matches: 2
f16.txt  lines: 1  matches: 2
f17.txt  lines: 1  matches: 2
f18.txt  lines: 1  matches: 2
f19.txt  lines: 1  matches: 2
f20.txt  lines: 1  matches: 2
f21.txt  lines: 1  matches: 2
f22.txt  lines: 1  matches: 2
f23.txt  lines: 1  matches: 2
f24.txt  lines: 1  matches: 2
```

3. the files which fit, with a cursor

```json request
{"query": "text has 'x'", "output": {"detail": "count", "count_by": "line", "budget": 150, "sort": "path asc"}}
```
```text response
files: 24  lines: 24  matches: 48  result: r3  count by: file (budget 150)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiYWNjZDBmNTkiLCJmIjoxMywidCI6WzI0LDI0LDQ4XX0

many/
f01.txt  lines: 1  matches: 2
f02.txt  lines: 1  matches: 2
f03.txt  lines: 1  matches: 2
f04.txt  lines: 1  matches: 2
f05.txt  lines: 1  matches: 2
f06.txt  lines: 1  matches: 2
f07.txt  lines: 1  matches: 2
f08.txt  lines: 1  matches: 2
f09.txt  lines: 1  matches: 2
f10.txt  lines: 1  matches: 2
f11.txt  lines: 1  matches: 2
f12.txt  lines: 1  matches: 2
f13.txt  lines: 1  matches: 2
```

4. the totals: fewer than ten files fit

```json request
{"query": "text has 'x'", "output": {"detail": "count", "count_by": "line", "budget": 100, "sort": "path asc"}}
```
```text response
files: 24  lines: 24  matches: 48  result: r4  count by: total (budget 100)  truncated: yes (budget)
```

5. in JSON

```json request
{"query": "text has 'x'", "output": {"detail": "count", "count_by": "file", "budget": 100, "sort": "path asc", "format": "json"}}
```
```json response
{"version": 1, "exists": true, "result": "r5", "count_by_used": "total", "counts": {"files": 24, "lines": 24, "matches": 48}, "truncated": true, "truncated_by": ["budget"]}
```

## E136.9 From the config file

Fixture: `count-by`

```toml conf/o.toml
[output]
detail = "count"
count_by = "file"
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
detail = "count"    # file: conf/o.toml
count_by = "file"    # file: conf/o.toml
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
omit = []    # default
show = []    # default
```

2. `find`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 3  lines: 6  matches: 7  result: r1

src/
lib.rs  lines: 2  matches: 3
tests/
load.rs  lines: 3  matches: 3
src/
main.rs  lines: 1  matches: 1
```

## E136.10 For the session and per query

Fixture: `count-by`

1. `update_config`

```json request update_config
{"set": "output.count_by = \"line\""}
```
```text response
[output]
count_by = "line"    # was "total"
```

2. `detail = count` lists the lines

```json request
{"query": "text has 'unwrap(' and name = 'lib.rs'", "output": {"detail": "count"}}
```
```text response
files: 1  lines: 2  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
```

3. `detail = files` ignores it

```json request
{"query": "text has 'unwrap(' and name = 'lib.rs'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 2  matches: 3  result: r2
#1 text has 'unwrap('
#2 name = 'lib.rs'

src/
lib.rs  [#1 #2]  lines: 2  matches: 3
```

4. every query of a batch with its own

```json request
{"output": {"detail": "count"}, "queries": [{"label": "lib", "query": "text has 'unwrap(' and name = 'lib.rs'"}, {"label": "main", "query": "text has 'unwrap(' and name = 'main.rs'", "output": {"count_by": "file"}}, {"label": "all", "query": "text has 'unwrap('", "output": {"count_by": "total"}}]}
```
```text response
== lib ==  files: 1  lines: 2  matches: 3  result: r3

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
== main ==  files: 1  lines: 1  matches: 1  result: r4

src/
main.rs  lines: 1  matches: 1
== all ==  files: 3  lines: 6  matches: 7  result: r5
```

## E136.11 The changes since a result

Fixture: `count-by`

1. the counts per line

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 3  lines: 6  matches: 7  result: r1

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
tests/
load.rs  lines: 3  matches: 3
  3: 1
  4: 1
  5: 1
src/
main.rs  lines: 1  matches: 1
  2: 1
```

2. a file is added, another one removed

```text setup
remove src/main.rs
bytes src/new.rs 612e756e7772617028622e756e777261702829293b0a
```

3. the differences

```json request
{"since": "r1", "query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 3  lines: 6  matches: 8  result: r2  since: r1  unchanged: 2

+ src/new.rs  lines: 1  matches: 2
  1: 2
- src/main.rs
```

## E136.12 Wrong values

Fixture: `count-by`

1. in a request

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "files"}}
```
```text error
error[invalid_option]: 'output.count_by' must be one of: total, file, line
```

2. in `update_config`

```json request update_config
{"set": "output.count_by = \"files\""}
```
```text error
error[invalid_value]: invalid value for 'output.count_by'
  hint: expected one of: total, file, line
```

3. an unknown part of `omit`: the list names `line_counts`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["line_count"]}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings, left_out)
```
