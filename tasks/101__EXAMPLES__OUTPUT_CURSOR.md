# Examples for task 101

A JSON response is compared as a JSON value. The examples use the fixture `sorting` of task 92: `a.txt` (one
`TODO`), `b.txt` (three), `c.txt` and `d/e.txt` (two each), the fixture `detail` of task 79: `src/a.rs` and
`src/b.rs` with two lines with `TODO` each, and the fixture `output-limits` of task 99: `a.txt` with three lines of 49,
50 and 50 characters, `b.txt` with one of 12. The cursor is opaque: the examples show the cursors this version makes.

## E101.1 The next page of the files

Fixture: `sorting`

1. `find`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 2}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 1  matches: 1
b.txt  [#1]  lines: 1  matches: 3
```

2. `find` — the same request with the cursor; the last page has no cursor

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 2, "cursor": "eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r2
#1 text has 'TODO'

./
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2
```

## E101.2 Inside a file

`max_lines` cut `src/b.rs` after its first line; the next page starts with its second line.

Fixture: `detail`

1. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 3}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiMGQ1OTAxOTAiLCJmIjoxLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ one
2:// ⟦#1:TODO⟧ four
b.rs  [#1]  lines: 2  matches: 2
1:// ⟦#1:TODO⟧ two
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_lines": 3, "cursor": "eyJ2IjoxLCJoIjoiMGQ1OTAxOTAiLCJmIjoxLCJtIjoxLCJ0IjpbMiw0LDRdfQ"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r2
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 2  matches: 2
2:// ⟦#1:TODO⟧ three
```

## E101.3 In JSON

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 3, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 4, "lines": 4, "matches": 8 },
  "truncated": true,
  "truncated_by": ["max_files"],
  "next_cursor": "eyJ2IjoxLCJoIjoiYmExZDc1NWIiLCJmIjozLCJ0IjpbNCw0LDhdfQ",
  "legend": { "#1": "text has 'TODO'" },
  "files": [
    { "path": "a.txt", "lines": 1, "matches": 1, "conditions": ["#1"] },
    { "path": "b.txt", "lines": 1, "matches": 3, "conditions": ["#1"] },
    { "path": "c.txt", "lines": 1, "matches": 2, "conditions": ["#1"] }
  ]
}
```

## E101.4 After `max_output`

The first line of `a.txt` fits, the second doesn't; the next page continues with it.

Fixture: `output-limits`

1. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output": 100}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYTY3ZTM0ZGEiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 3  matches: 3
1:the first line has a ⟦#1:TODO⟧ and some words after it
```

2. `find`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_output": 100, "cursor": "eyJ2IjoxLCJoIjoiYTY3ZTM0ZGEiLCJmIjowLCJtIjoxLCJ0IjpbMiw0LDRdfQ"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r2  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYTY3ZTM0ZGEiLCJmIjowLCJtIjoyLCJ0IjpbMiw0LDRdfQ
#1 text has 'TODO'

./
a.txt  [#1]  lines: 3  matches: 3
2:the second line has a ⟦#1:TODO⟧ and more words after it
```

## E101.5 No cursor for the per file limits

They cut by design.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"sort": "path asc", "max_matches_per_file": 1, "group_dirs": false}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1  truncated: yes (max_matches_per_file)
#1 text has 'TODO'

a.txt  [#1]  lines: 1  matches: 1
1:TODO
b.txt  [#1]  lines: 1  matches: 3  (cut)
1:⟦#1:TODO⟧ TODO TODO
c.txt  [#1]  lines: 1  matches: 2  (cut)
1:⟦#1:TODO⟧ TODO
d/e.txt  [#1]  lines: 1  matches: 2  (cut)
1:⟦#1:TODO⟧ TODO
```

## E101.6 A cursor of another request

Fixture: `sorting`

```json request
{"query": "text has 'FIXME'", "output": {"detail": "files", "sort": "path asc", "max_files": 2, "cursor": "eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ"}}
```

```text error
error[cursor_mismatch]: the cursor belongs to another request
  hint: repeat the request of the cursor with only output.cursor added
```

## E101.7 A cursor which can't be decoded

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"cursor": "not a cursor"}}
```

```text error
error[cursor_invalid]: the cursor can't be decoded
  hint: pass next_cursor of the previous response unchanged
```

```json request
{"query": "text has 'TODO'", "output": {"cursor": 5}}
```

```text error
error[invalid_option]: 'output.cursor' must be a string
```

## E101.8 Changed results

The totals of the cursor differ from the ones of the search: the files changed between the pages.

Fixture: `sorting`

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "sort": "path asc", "max_files": 2, "cursor": "eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNSw1LDldfQ"}}
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 text has 'TODO'

./
c.txt  [#1]  lines: 1  matches: 2
d/
e.txt  [#1]  lines: 1  matches: 2

warning: results_changed
```

## E101.9 Not a configuration key

Fixture: `sorting`

```json request update_config
{"set": "output.cursor = 'x'"}
```

```text error
error[unknown_key]: unknown key 'output.cursor'
```
