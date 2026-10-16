# Examples for task 138

Spec: `1`

`output.repeated_locations` sets how many locations of a repeated line are written, `0` all of them
(OUTPUT.md#repeated-lines).

## Fixtures

### repeated-locations

```text tree
src/a.rs          2026-10-01 10:00
src/b.rs          2026-10-01 10:00
src/c.rs          2026-10-01 10:00
src/d.rs          2026-10-01 10:00
src/e.rs          2026-10-01 10:00
src/f.rs          2026-10-01 10:00
src/g.rs          2026-10-01 10:00
src/h.rs          2026-10-01 10:00
src/i.rs          2026-10-01 10:00
src/j.rs          2026-10-01 10:00
src/k.rs          2026-10-01 10:00
src/l.rs          2026-10-01 10:00
src/main.rs       2026-10-01 10:00
```

File contents are in `tests/fixtures/repeated-locations/`: every file starts with `use std::io;`, `src/a.rs`,
`src/b.rs` and `src/c.rs` have `use std::fmt;` in line 2, `src/main.rs` has `let a = std::io::stdin();` in line 2.
With `text has 'std::'` the first line is repeated in 13 files, the second one in 3; only `src/main.rs` has a line
of its own. The examples sort by path.

## E138.1 How many locations

Fixture: `repeated-locations`

1. three by default

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc"}}
```
```text response
files: 13  lines: 17  matches: 17  result: r1
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, … 10 more
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, src/c.rs:2
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

2. five

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "repeated_locations": 5}}
```
```text response
files: 13  lines: 17  matches: 17  result: r2
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, src/d.rs:1, src/e.rs:1, … 8 more
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, src/c.rs:2
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

3. all of them

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "repeated_locations": 0}}
```
```text response
files: 13  lines: 17  matches: 17  result: r3
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, src/d.rs:1, src/e.rs:1, src/f.rs:1, src/g.rs:1, src/h.rs:1, src/i.rs:1, src/j.rs:1, src/k.rs:1, src/l.rs:1, src/main.rs:1
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, src/c.rs:2
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

4. all of them in JSON

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "repeated_locations": 0, "format": "json", "omit": ["legend", "conditions"]}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r4",
  "repeated": [
    {"count": 13, "text": "use std::io;", "fragments": [{"col": [5, 10], "text": "std::", "ids": ["#1"]}],
      "locations": ["src/a.rs:1", "src/b.rs:1", "src/c.rs:1", "src/d.rs:1", "src/e.rs:1", "src/f.rs:1", "src/g.rs:1", "src/h.rs:1", "src/i.rs:1", "src/j.rs:1", "src/k.rs:1", "src/l.rs:1", "src/main.rs:1"]},
    {"count": 3, "text": "use std::fmt;", "fragments": [{"col": [5, 10], "text": "std::", "ids": ["#1"]}],
      "locations": ["src/a.rs:2", "src/b.rs:2", "src/c.rs:2"]}
  ],
  "counts": {"files": 13, "lines": 17, "matches": 17},
  "files": [
    {"path": "src/main.rs", "lines": 2, "matches": 2,
      "blocks": [{"lines": [{"n": 2, "text": "let a = std::io::stdin();", "fragments": [{"col": [9, 14], "text": "std::", "ids": ["#1"]}]}]}]}
  ]
}
```

5. the default in JSON: the first three, `count` of all the files

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "format": "json", "omit": ["legend", "conditions"]}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r5",
  "repeated": [
    {"count": 13, "text": "use std::io;", "fragments": [{"col": [5, 10], "text": "std::", "ids": ["#1"]}],
      "locations": ["src/a.rs:1", "src/b.rs:1", "src/c.rs:1"]},
    {"count": 3, "text": "use std::fmt;", "fragments": [{"col": [5, 10], "text": "std::", "ids": ["#1"]}],
      "locations": ["src/a.rs:2", "src/b.rs:2", "src/c.rs:2"]}
  ],
  "counts": {"files": 13, "lines": 17, "matches": 17},
  "files": [
    {"path": "src/main.rs", "lines": 2, "matches": 2,
      "blocks": [{"lines": [{"n": 2, "text": "let a = std::io::stdin();", "fragments": [{"col": [9, 14], "text": "std::", "ids": ["#1"]}]}]}]}
  ]
}
```

## E138.2 The locations left out

Fixture: `repeated-locations`

1. `locations`: the line and the number of files stay

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "omit": ["locations"]}}
```
```text response
files: 13  lines: 17  matches: 17  result: r1
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;
  ×3 use ⟦#1:std::⟧fmt;
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

2. all

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "repeated_locations": 0, "omit": "all"}}
```
```text response
repeated:
  ×13 use ⟦std::⟧io;
  ×3 use ⟦std::⟧fmt;
src/
main.rs
let a = ⟦std::⟧io::stdin();
```

3. in JSON

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "format": "json", "omit": ["locations", "legend", "conditions", "match_ids", "line_numbers"], "highlight": "none"}}
```
```json response
{
  "version": 1,
  "exists": true,
  "result": "r3",
  "repeated": [
    {"count": 13, "text": "use std::io;"},
    {"count": 3, "text": "use std::fmt;"}
  ],
  "counts": {"files": 13, "lines": 17, "matches": 17},
  "files": [
    {"path": "src/main.rs", "lines": 2, "matches": 2,
      "blocks": [{"lines": [{"text": "let a = std::io::stdin();", "match": true}]}]}
  ]
}
```

## E138.3 The locations cut by `max_output` and the budget

Fixture: `repeated-locations`

1. `max_output`: the locations take 94 characters with eight of them, the ninth would go over 100; the next line
   keeps only its count of the locations left out

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "repeated_locations": 0, "max_output": 100}}
```
```text response
files: 13  lines: 17  matches: 17  result: r1  truncated: yes (max_output)
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, src/d.rs:1, src/e.rs:1, src/f.rs:1, src/g.rs:1, src/h.rs:1, … 5 more
  ×3 use ⟦#1:std::⟧fmt;    … 3 more
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

2. the budget, with three characters a token: three locations fit into 100 tokens (96), all of them don't (133)

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "budget": 100, "chars_per_token": 3}}
```
```text response
files: 13  lines: 17  matches: 17  result: r2
#1 text has 'std::'

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, … 10 more
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, src/c.rs:2
src/
main.rs  [#1]  lines: 2  matches: 2
2:let a = ⟦#1:std::⟧io::stdin();
```

3. all of them don't: the budget lowers the detail

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "budget": 100, "chars_per_token": 3, "repeated_locations": 0}}
```
```text response
files: 13  lines: 17  matches: 17  result: r3  detail: facets (budget 100)  truncated: yes (budget)
#1 text has 'std::'
dir:        src/ 13
lang:       rust 13
kind:       source 13
condition:  #1 13
```

## E138.4 For the session and per query

Fixture: `repeated-locations`

1. `update_config`

```json request update_config
{"set": "output.repeated_locations = 0"}
```
```text response
[output]
repeated_locations = 0    # was 3
```

2. a request

```json request
{"query": "text has 'std::'", "output": {"sort": "path asc", "omit": ["legend", "conditions", "file_counts"]}}
```
```text response
files: 13  lines: 17  matches: 17  result: r1

repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, src/c.rs:1, src/d.rs:1, src/e.rs:1, src/f.rs:1, src/g.rs:1, src/h.rs:1, src/i.rs:1, src/j.rs:1, src/k.rs:1, src/l.rs:1, src/main.rs:1
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, src/c.rs:2
src/
main.rs
2:let a = ⟦#1:std::⟧io::stdin();
```

3. every query of a batch with its own

```json request
{"output": {"sort": "path asc", "omit": ["legend", "conditions", "file_counts", "result", "counts"]}, "queries": [{"label": "two", "query": "text has 'std::'", "output": {"repeated_locations": 2}}, {"label": "session", "query": "text has 'std::fmt'"}]}
```
```text response
== two ==  repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, … 11 more
  ×3 use ⟦#1:std::⟧fmt;    src/a.rs:2, src/b.rs:2, … 1 more
src/
main.rs
2:let a = ⟦#1:std::⟧io::stdin();
== session ==  repeated:
  ×3 use ⟦#1:std::fmt⟧;    src/a.rs:2, src/b.rs:2, src/c.rs:2
```

## E138.5 Wrong values

Fixture: `repeated-locations`

1. a request

```json request
{"query": "text has 'std::'", "output": {"repeated_locations": -1}}
```
```text error
error[invalid_option]: 'output.repeated_locations' must be a whole number
```

2. `update_config`

```json request update_config
{"set": "output.repeated_locations = 'all'"}
```
```text error
error[invalid_value]: invalid value for 'output.repeated_locations'
  hint: a whole number is expected (0: all)
```

3. an unknown part of `omit`: the list names `locations`

```json request
{"query": "text has 'std::'", "output": {"omit": ["location"]}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, more_lines, locations, warnings, left_out)
```
