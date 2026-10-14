# Examples for task 140

The query of a write tool selects files only, like `find`; `type = dir` selects directories and `type = link`
symlinks, and the descriptions of `delete`, `rename` and `copy` say so ([TOOLS.md](../TOOLS.md#selection)).

## Fixtures

### select-types

```text tree
a.tmp                2026-10-01 10:00
keep.txt             2026-10-01 10:00
old.tmp/x.txt        2026-10-01 10:00
```

```text setup
symlink link.tmp -> keep.txt
```

File contents are in `tests/fixtures/select-types/`: `a.tmp` is a file, `old.tmp` a directory and `link.tmp` a
symlink to `keep.txt`, all three named `*.tmp`.

## E140.1 The descriptions name `type = dir` and `type = link`

Fixture: `select-types`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}, {"name": "delete", "description": "*\nThe query selects files only, like find: type = dir selects directories, deleted with everything in them;\n  type = link symlinks, deleted themselves, never followed.\n*"}, {"name": "rename", "description": "*The query selects files only, like find: type = dir selects directories, moved\n  with everything in them; type = link symlinks, moved themselves.\n*"}, {"name": "copy", "description": "*The query selects files only, like find: type = dir selects directories, copied\n  with everything in them; type = link symlinks, copied as symlinks.\n*"}, {"name": "replace"}]}}
```

## E140.2 A name selects the file only

Fixture: `select-types`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.tmp'", "dry_run": true}
```
```text response
deleted: 1  dry run

a.tmp
```

## E140.3 `type = link` selects the symlink

Fixture: `select-types`

```text command
easy-peasy-mcp --allow-write
```

1. delete the symlink

```json request delete
{"query": "name ~ '*.tmp' and type = link"}
```
```text response
deleted: 1

link.tmp
```
```text trash
link.tmp -> keep.txt
```

2. its target stays

```json request
{"output": {"detail": "files", "sort": "path asc", "omit": ["legend", "conditions"]}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1

./
a.tmp
keep.txt
old.tmp/
x.txt
```

## E140.4 Files and symlinks, and directories

Fixture: `select-types`

```text command
easy-peasy-mcp --allow-write
```

1. files and symlinks

```json request delete
{"query": "name ~ '*.tmp' and type in (file, link)", "dry_run": true}
```
```text response
deleted: 2  dry run

a.tmp
link.tmp
```

2. a directory with everything in it

```json request delete
{"query": "name ~ '*.tmp' and type = dir", "dry_run": true}
```
```text response
deleted: 1  dry run

old.tmp/
```
