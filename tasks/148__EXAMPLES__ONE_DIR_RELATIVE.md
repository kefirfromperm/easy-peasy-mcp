# Examples for task 148

In spec 2 `path_style = relative` (the default) with one working directory given as an absolute path writes the
directory once in the `in:` line and the paths relative to it, as `in_dir` does (OUTPUT.md#one-working-directory). The examples are
written for the current spec version 2.

## Fixtures

### one-dir

```text tree
other/lib.rs          2026-10-01 10:00
proj/data.bin         2026-10-01 10:00
proj/src/a.rs         2026-10-01 10:00
proj/src/b.rs         2026-10-01 10:00
proj/src/c.rs         2026-10-01 10:00
```

File contents are in `tests/fixtures/one-dir/`: `proj/src/a.rs`, `proj/src/b.rs`, `proj/src/c.rs` and `other/lib.rs`
start with the line `use crate::cache;`, `proj/src/a.rs` has `fn a() { cache(); }` in line 2; `proj/data.bin` is
binary, so a content condition gives the `binary_skipped` warning. Most examples search the absolute working
directory `<fixture>/proj`, as an agent in a git worktree outside the default directory does.

## E148.1 One absolute working directory

Fixture: `one-dir`

1. `lines` detail: the directory once, the locations of the repeated line relative to it

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'"}
```
```text response
files: 3  lines: 4  result: r1
in: <fixture>/proj/

repeated:
  ×3 use crate::⟦cache⟧;    src/a.rs:1, src/b.rs:1, src/c.rs:1
src/
a.rs  lines: 2
2:fn a() { ⟦cache⟧(); }

warnings left out: binary_skipped (1)
```

2. `files` detail, the warning relative to it too

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files", "show": ["warnings"]}}
```
```text response
files: 3  lines: 4  result: r2
in: <fixture>/proj/

src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warning: binary_skipped (1): data.bin
```

3. `facets` detail: the directories relative to it

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "facets"}}
```
```text response
files: 3  lines: 4  result: r3
in: <fixture>/proj/
dir:        src/ 3
lang:       rust 3
kind:       source 3
condition:  #1 3

warnings left out: binary_skipped (1)
```

## E148.2 In JSON

Fixture: `one-dir`

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files", "format": "json"}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "dirs": ["<fixture>/proj/"],
  "counts": {"files": 3, "lines": 4, "matches": 4},
  "files": [
    {"path": "src/a.rs", "lines": 2, "matches": 2},
    {"path": "src/b.rs", "lines": 1, "matches": 1},
    {"path": "src/c.rs", "lines": 1, "matches": 1}
  ],
  "left_out": {"warnings": {"binary_skipped": 1}}
}
```

## E148.3 A relative working directory is unchanged

Fixture: `one-dir`

1. `proj/src`: the directory before the paths, shorter than `in: proj/src/` with a `./` header

```json request
{"dirs": ["proj/src"], "query": "text has 'cache'", "output": {"detail": "files"}}
```
```text response
files: 3  lines: 4  result: r1

proj/src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1
```

2. `.`: no line

```json request
{"query": "text has 'cache'", "output": {"detail": "files"}}
```
```text response
files: 4  lines: 5  result: r2

other/
lib.rs  lines: 1
proj/src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warnings left out: binary_skipped (1)
```

## E148.4 Unchanged: several working directories, `absolute`

Fixture: `one-dir`

1. two working directories: every path with its working directory

```json request
{"dirs": ["<fixture>/proj", "other"], "query": "text has 'cache'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 4  lines: 5  result: r1

<fixture>/proj/src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1
other/
lib.rs  lines: 1

warnings left out: binary_skipped (1)
```

2. `absolute`

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"path_style": "absolute", "detail": "files"}}
```
```text response
files: 3  lines: 4  result: r2

<fixture>/proj/src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warnings left out: binary_skipped (1)
```

## E148.5 `omit`, `show` and spec 1

Fixture: `one-dir`

1. `omit = ["dir"]`: no line, the paths stay relative

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files", "omit": ["dir"]}}
```
```text response
files: 3  lines: 4  result: r1

src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warnings left out: binary_skipped (1)
```

2. `show = ["dir"]`: the working directory before every path, as in spec 1

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files", "show": ["dir"]}}
```
```text response
files: 3  lines: 4  result: r2

<fixture>/proj/src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warnings left out: binary_skipped (1)
```

3. `"version": 1` is unchanged

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files"}, "version": 1}
```
```text response
files: 3  lines: 4  matches: 4  result: r3
#1 text has 'cache'

<fixture>/proj/src/
a.rs  [#1]  lines: 2  matches: 2
b.rs  [#1]  lines: 1  matches: 1
c.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): <fixture>/proj/data.bin
```

## E148.6 `within` and `since`

Fixture: `one-dir`

1. the result

```json request
{"dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files"}}
```
```text response
files: 3  lines: 4  result: r1
in: <fixture>/proj/

src/
a.rs  lines: 2
b.rs  lines: 1
c.rs  lines: 1

warnings left out: binary_skipped (1)
```

2. of its files, which have `fn`: the working directory of `r1` once

```json request
{"within": "r1", "query": "text has 'fn'"}
```
```text response
files: 1  lines: 1  result: r2
in: <fixture>/proj/

src/a.rs
2:⟦fn⟧ a() { cache(); }
```

3. a file is removed

```text setup
remove proj/src/c.rs
```

4. the changes since the result

```json request
{"since": "r1", "dirs": ["<fixture>/proj"], "query": "text has 'cache'", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  result: r3  since: r1  unchanged: 2
in: <fixture>/proj/

- src/c.rs

warnings left out: binary_skipped (1)
```
