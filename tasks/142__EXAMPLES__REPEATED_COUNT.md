# Examples for task 142

`×N` of a repeated line (OUTPUT.md#repeated-lines) counts every file of the result which has the line, also the files
which are not listed; the locations are those of the listed files, `… M more` counts the others. The examples are
written for the current spec version 2.

## Fixtures

### repeated-count

```text tree
a1.rs              2026-10-01 10:00
a2.rs              2026-10-01 10:00
a3.rs              2026-10-01 10:00
a4.rs              2026-10-01 10:00
a5.rs              2026-10-01 10:00
a6.rs              2026-10-01 10:00
z1.rs              2026-10-01 10:00
z2.rs              2026-10-01 10:00
z3.rs              2026-10-01 10:00
```

File contents are in `tests/fixtures/repeated-count/`: `a1.rs` … `a6.rs` are `use common;` and `use aN;`,
`z1.rs` … `z3.rs` are `use late;` and `use zN;`. With `max_files = 4` and `sort = path asc` the first page lists
`a1.rs` … `a4.rs`.

## E142.1 More files than `max_files`

`use common;` is in six files, four of them listed; `use late;` is only in files which are not listed.

Fixture: `repeated-count`

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc"}}
```

```text response
files: 9  lines: 18  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDRjNGVjZmUiLCJmIjo0LCJ0IjpbOSwxOCwxOF19

repeated:
  ×6 ⟦use ⟧common;    a1.rs:1, a2.rs:1, a3.rs:1, … 3 more
./
a1.rs  lines: 2
2:⟦use ⟧a1;
a2.rs  lines: 2
2:⟦use ⟧a2;
a3.rs  lines: 2
2:⟦use ⟧a3;
a4.rs  lines: 2
2:⟦use ⟧a4;
```

## E142.2 All the locations

Fixture: `repeated-count`

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc", "repeated_locations": 0}}
```

```text response
files: 9  lines: 18  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDJlZDE0NWYiLCJmIjo0LCJ0IjpbOSwxOCwxOF19

repeated:
  ×6 ⟦use ⟧common;    a1.rs:1, a2.rs:1, a3.rs:1, a4.rs:1, … 2 more
./
a1.rs  lines: 2
2:⟦use ⟧a1;
a2.rs  lines: 2
2:⟦use ⟧a2;
a3.rs  lines: 2
2:⟦use ⟧a3;
a4.rs  lines: 2
2:⟦use ⟧a4;
```

## E142.3 JSON

Fixture: `repeated-count`

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc", "format": "json"}}
```

```json response
{"version":2,"exists":true,"result":"r1","repeated":[{"count":6,"text":"use common;","fragments":[{"col":[1,5],"text":"use "}],"locations":["a1.rs:1","a2.rs:1","a3.rs:1"]}],"counts":{"files":9,"lines":18,"matches":18},"truncated":true,"truncated_by":["max_files"],"next_cursor":"eyJ2IjoxLCJoIjoiOTlhOTNhMmQiLCJmIjo0LCJ0IjpbOSwxOCwxOF19","files":[{"path":"a1.rs","lines":2,"matches":2,"blocks":[{"lines":[{"n":2,"text":"use a1;","fragments":[{"col":[1,5],"text":"use "}]}]}]},{"path":"a2.rs","lines":2,"matches":2,"blocks":[{"lines":[{"n":2,"text":"use a2;","fragments":[{"col":[1,5],"text":"use "}]}]}]},{"path":"a3.rs","lines":2,"matches":2,"blocks":[{"lines":[{"n":2,"text":"use a3;","fragments":[{"col":[1,5],"text":"use "}]}]}]},{"path":"a4.rs","lines":2,"matches":2,"blocks":[{"lines":[{"n":2,"text":"use a4;","fragments":[{"col":[1,5],"text":"use "}]}]}]}]}
```

## E142.4 The pages of a cursor

Every page writes `use common;` with `×6` and the locations of its own files; `use late;` comes with its files.

Fixture: `repeated-count`

1. the first page

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc"}}
```

```text response
files: 9  lines: 18  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDRjNGVjZmUiLCJmIjo0LCJ0IjpbOSwxOCwxOF19

repeated:
  ×6 ⟦use ⟧common;    a1.rs:1, a2.rs:1, a3.rs:1, … 3 more
./
a1.rs  lines: 2
2:⟦use ⟧a1;
a2.rs  lines: 2
2:⟦use ⟧a2;
a3.rs  lines: 2
2:⟦use ⟧a3;
a4.rs  lines: 2
2:⟦use ⟧a4;
```

2. the second page

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiMDRjNGVjZmUiLCJmIjo0LCJ0IjpbOSwxOCwxOF19"}}
```

```text response
files: 9  lines: 18  result: r2  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDRjNGVjZmUiLCJmIjo4LCJ0IjpbOSwxOCwxOF19

repeated:
  ×6 ⟦use ⟧common;    a5.rs:1, a6.rs:1, … 4 more
  ×3 ⟦use ⟧late;    z1.rs:1, z2.rs:1, … 1 more
./
a5.rs  lines: 2
2:⟦use ⟧a5;
a6.rs  lines: 2
2:⟦use ⟧a6;
z1.rs  lines: 2
2:⟦use ⟧z1;
z2.rs  lines: 2
2:⟦use ⟧z2;
```

3. the last page

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiMDRjNGVjZmUiLCJmIjo4LCJ0IjpbOSwxOCwxOF19"}}
```

```text response
files: 9  lines: 18  result: r3

repeated:
  ×3 ⟦use ⟧late;    z3.rs:1, … 2 more
./
z3.rs  lines: 2
2:⟦use ⟧z3;
```

## E142.5 A budget

Fixture: `repeated-count`

```json request
{"query": "text has 'use '", "output": {"max_files": 4, "sort": "path asc", "budget": 100}}
```

```text response
files: 9  lines: 18  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNTRmYTJhZmMiLCJmIjo0LCJ0IjpbOSwxOCwxOF19

repeated:
  ×6 ⟦use ⟧common;    a1.rs:1, a2.rs:1, a3.rs:1, … 3 more
./
a1.rs  lines: 2
2:⟦use ⟧a1;
a2.rs  lines: 2
2:⟦use ⟧a2;
a3.rs  lines: 2
2:⟦use ⟧a3;
a4.rs  lines: 2
2:⟦use ⟧a4;
```
