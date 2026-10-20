# Examples for task 157

In spec 2 a write tool with one working directory given as an absolute path writes the directory once in the `in:`
line and its paths relative to it, as `find` does
([TOOLS.md#one-working-directory-of-a-write-tool](../TOOLS.md#one-working-directory-of-a-write-tool)). The examples
are written for the current spec version 2 and use the fixture `one-dir` of task 148: `proj/src/a.rs`, `proj/src/b.rs`,
`proj/src/c.rs` and `other/lib.rs` start with the line `use crate::cache;`, `proj/src/a.rs` has `fn a() { cache(); }`
in line 2. Most examples work in the absolute working directory `<fixture>/proj`, as an agent in a git worktree outside
the default directory does. They are startup examples with `--allow-write`; the trash is the test hook of AGENTS.md.

## E157.1 Replace

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. the dry run

```json request replace
{"dirs": ["<fixture>/proj"], "query": "text has 'cache();' @old", "replace": {"@old": "load();"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1
in: <fixture>/proj/

src/
a.rs  matches: 1
-2:fn a() { cache(); }
+2:fn a() { load(); }
```

2. the real call

```json request replace
{"dirs": ["<fixture>/proj"], "query": "text has 'cache();' @old", "replace": {"@old": "load();"}, "expect_matches": 1}
```
```text response
replaced: 1  matches: 1
in: <fixture>/proj/

src/
a.rs  matches: 1
-2:fn a() { cache(); }
+2:fn a() { load(); }
```

## E157.2 Rename

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. the dry run: the target relative to the working directory too

```json request rename
{"dirs": ["<fixture>/proj"], "query": "name = 'b.rs'", "to": "{dir}/d.rs", "dry_run": true}
```
```text response
dry run  would rename: 1
in: <fixture>/proj/

src/
b.rs -> src/d.rs
```

2. the real call

```json request rename
{"dirs": ["<fixture>/proj"], "query": "name = 'b.rs'", "to": "{dir}/d.rs", "expect": 1}
```
```text response
renamed: 1
in: <fixture>/proj/

src/
b.rs -> src/d.rs
```

## E157.3 Copy

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. the dry run

```json request copy
{"dirs": ["<fixture>/proj"], "query": "name ~ '*.rs'", "to": "backup/{path}", "dry_run": true}
```
```text response
dry run  would copy: 3
in: <fixture>/proj/

src/
a.rs -> backup/src/a.rs
b.rs -> backup/src/b.rs
c.rs -> backup/src/c.rs
```

2. the real call

```json request copy
{"dirs": ["<fixture>/proj"], "query": "name ~ '*.rs'", "to": "backup/{path}", "expect": 3}
```
```text response
copied: 3
in: <fixture>/proj/

src/
a.rs -> backup/src/a.rs
b.rs -> backup/src/b.rs
c.rs -> backup/src/c.rs
```

## E157.4 Delete

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. the dry run

```json request delete
{"dirs": ["<fixture>/proj"], "query": "name = 'c.rs'", "dry_run": true}
```
```text response
dry run  would delete: 1
in: <fixture>/proj/

src/
c.rs
```
```text trash
```

2. the real call

```json request delete
{"dirs": ["<fixture>/proj"], "query": "name = 'c.rs'", "expect": 1}
```
```text response
deleted: 1
in: <fixture>/proj/

src/
c.rs
```
```text trash
proj/
proj/src/
proj/src/c.rs
```

## E157.5 A target in another directory

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. `rename` out of the working directory: the target with `..`, as `to` resolves it

```json request rename
{"dirs": ["<fixture>/proj"], "query": "name = 'a.rs'", "to": "../other/{name}"}
```
```text response
renamed: 1
in: <fixture>/proj/

src/
a.rs -> ../other/a.rs
```

2. `copy` into another directory inside it

```json request copy
{"dirs": ["<fixture>/proj"], "query": "name = 'b.rs'", "to": "lib/{stem}.bak.rs"}
```
```text response
copied: 1
in: <fixture>/proj/

src/
b.rs -> lib/b.bak.rs
```

3. the paths resolve against the working directories: `../other/a.rs` is in `other`

```json request
{"dirs": ["<fixture>/other"], "query": "name = 'a.rs'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  result: r1
in: <fixture>/other/

./
a.rs
```

## E157.6 A result and the warnings

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. a `find` of the working directory

```json request
{"dirs": ["<fixture>/proj"], "query": "name ~ '*.rs'", "output": {"detail": "files"}}
```
```text response
files: 3  lines: 0  result: r1
in: <fixture>/proj/

src/
a.rs
b.rs
c.rs
```

```text setup
file proj/src/b.rs 3
```

2. `delete` of the result: its working directory once, the skipped file relative to it

```json request delete
{"result": "r1", "dry_run": true}
```
```text response
dry run  would delete: 2  skipped: 1
in: <fixture>/proj/

src/
a.rs
c.rs

warning: file_changed (1): src/b.rs
```

3. a path copied from the response selects the file again

```json request delete
{"dirs": ["<fixture>/proj"], "query": "path ~ 'src/a.rs'", "dry_run": true}
```
```text response
dry run  would delete: 1
in: <fixture>/proj/

src/
a.rs
```

## E157.7 In JSON

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. `replace`

```json request replace
{"dirs": ["<fixture>/proj"], "query": "text has 'cache();' @old", "replace": {"@old": "load();"}, "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 2,
  "replaced": 1,
  "matches": 1,
  "dry_run": true,
  "dirs": ["<fixture>/proj/"],
  "files": [
    {"path": "src/a.rs", "matches": 1, "changes": [{"line": 2, "old": ["fn a() { cache(); }"], "new_line": 2, "new": ["fn a() { load(); }"]}]}
  ]
}
```

2. `rename`

```json request rename
{"dirs": ["<fixture>/proj"], "query": "name = 'b.rs'", "to": "{dir}/d.rs", "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 2,
  "renamed": 1,
  "dry_run": true,
  "dirs": ["<fixture>/proj/"],
  "files": [
    {"path": "src/b.rs", "to": "src/d.rs"}
  ]
}
```

## E157.8 Several working directories, a relative one, spec 1: unchanged

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

1. two working directories: every path with its working directory

```json request delete
{"dirs": ["<fixture>/proj", "<fixture>/other"], "query": "name ~ '*.rs'", "dry_run": true}
```
```text response
dry run  would delete: 4

<fixture>/other/
lib.rs
<fixture>/proj/src/
a.rs
b.rs
c.rs
```

2. a relative working directory

```json request delete
{"dirs": ["proj"], "query": "name ~ '*.rs'", "dry_run": true}
```
```text response
dry run  would delete: 3

proj/src/
a.rs
b.rs
c.rs
```

3. spec 1

```json request delete
{"version": 1, "dirs": ["<fixture>/proj"], "query": "name ~ '*.rs'", "dry_run": true}
```
```text response
deleted: 3  dry run

<fixture>/proj/src/
a.rs
b.rs
c.rs
```

## E157.9 An empty selection: only the header

Fixture: `one-dir`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"dirs": ["<fixture>/proj"], "query": "name = 'none.rs'", "dry_run": true}
```
```text response
dry run  would delete: 0
```
