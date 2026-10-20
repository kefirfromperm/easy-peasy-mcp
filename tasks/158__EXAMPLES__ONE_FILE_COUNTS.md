# Examples for task 158

In spec 2 the text format writes the only file of a result as its path alone: without `lines: N  matches: N`, which
repeat the totals of the header, and without its directory header (OUTPUT.md#one-file). The examples are written for
the current spec version 2.

## Fixtures

The examples use fixtures of earlier tasks:

- `one-dir` of task 148: `proj/src/a.rs`, `proj/src/b.rs`, `proj/src/c.rs` start with the line `use crate::cache;`,
  `proj/src/a.rs` has `fn a() { cache(); }` in line 2
- `omit` of task 134: `src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs` with `unwrap(` in line 2 and `TODO`
  in line 3, the binary `data.bin` which gives `binary_skipped`
- `match-counts` of task 152: `a.rs` has `foo` in lines 1 and 2, `b.rs` has `foo` twice in line 1

## E158.1 One file in an absolute working directory

Fixture: `one-dir`

The file is directly in the working directory: no `./` header, no counts after its name.

1. count by file

```json request
{"dirs": ["<fixture>/proj/src"], "query": "text has 'cache();'", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 1  lines: 1  result: r1
in: <fixture>/proj/src/

a.rs
```

2. files

```json request
{"dirs": ["<fixture>/proj/src"], "query": "text has 'cache();'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 1  result: r2
in: <fixture>/proj/src/

a.rs
```

3. lines

```json request
{"dirs": ["<fixture>/proj/src"], "query": "text has 'cache();'"}
```
```text response
files: 1  lines: 1  result: r3
in: <fixture>/proj/src/

a.rs
2:fn a() { ⟦cache();⟧ }
```

## E158.2 One file below the working directory

Fixture: `omit`

The file is written with its path relative to the working directory instead of under the header `src/`.

1. count by file

```json request
{"query": "text has 'TODO'", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 1  lines: 1  result: r1

src/main.rs

warnings left out: binary_skipped (1)
```

2. files

```json request
{"query": "text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 1  result: r2

src/main.rs

warnings left out: binary_skipped (1)
```

3. lines

```json request
{"query": "text has 'TODO'"}
```
```text response
files: 1  lines: 1  result: r3

src/main.rs
⇥4
3:// ⟦TODO⟧ handle the error

warnings left out: binary_skipped (1)
```

## E158.3 One file with more matches than lines

Fixture: `match-counts`

The header keeps `matches: 2`; the file in the working directory `.` has no `./` header.

1. count by file

```json request
{"query": "name = 'b.rs' and text has 'foo'", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 1  lines: 1  matches: 2  result: r1

b.rs
```

2. files

```json request
{"query": "name = 'b.rs' and text has 'foo'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 1  matches: 2  result: r2

b.rs
```

## E158.4 Two files: unchanged

Fixture: `omit`

1. count by file

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 2  lines: 3  result: r1

src/
lib.rs  lines: 2
main.rs  lines: 1

warnings left out: binary_skipped (1)
```

2. files

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  result: r2

src/
lib.rs  lines: 2
main.rs  lines: 1

warnings left out: binary_skipped (1)
```

## E158.5 The counts brought back

Fixture: `omit`

1. `show = ["file_counts"]` writes the counts; the path stays without its directory header

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "show": ["file_counts"]}}
```
```text response
files: 1  lines: 1  result: r1

src/main.rs  lines: 1

warnings left out: binary_skipped (1)
```

2. `omit = ["counts"]`: the header has no counts, so the file keeps them

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "omit": ["counts"]}}
```
```text response
result: r2

src/main.rs  lines: 1

warnings left out: binary_skipped (1)
```

3. `"version": 1` writes spec 1

```json request
{"version": 1, "query": "text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r3
#1 text has 'TODO'

src/
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

4. `show = "all"` writes spec 1 too

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "show": "all"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r4
#1 text has 'TODO'

src/
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

## E158.6 Count by line: unchanged

Fixture: `omit`

```json request
{"query": "text has 'TODO'", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 1  lines: 1  result: r1

src/
main.rs  lines: 1
  3

warnings left out: binary_skipped (1)
```

## E158.7 JSON: unchanged

Fixture: `match-counts`

```json request
{"query": "name = 'b.rs' and text has 'foo'", "output": {"detail": "files", "format": "json"}}
```
```json response
{"version":2,"exists":true,"result":"r1","counts":{"files":1,"lines":1,"matches":2},"files":[{"path":"b.rs","lines":1,"matches":2}]}
```
