# Examples for task 139

The header of a dry run starts with what it is: `dry run  would delete: 2` ([TOOLS.md#dry-run](../TOOLS.md#dry-run)).
The examples are written for spec 2 and use the fixtures of the write tools: `delete` of task 125, `rename` of task
126, `copy` of task 127, `replace` of task 129. They are startup examples with `--allow-write`; the trash is the test
hook of AGENTS.md.

## E139.1 A dry run with entries

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "dry_run": true}
```
```text response
dry run  would delete: 2

logs/
app.log
old.log
```
```text trash
```

## E139.2 With the other counts and `permanent`

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "traversal": {"archives": "all"}, "dry_run": true, "permanent": true}
```
```text response
dry run  would delete: 2  skipped: 1  permanent

logs/
app.log
old.log

warning: archive_member (1): logs/a.zip!/inner/zip.log
```

## E139.3 An empty selection

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name = 'none.txt'", "dry_run": true}
```
```text response
dry run  would delete: 0
```

## E139.4 JSON keeps its keys

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "traversal": {"archives": "all"}, "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 2,
  "deleted": 2,
  "skipped": 1,
  "dry_run": true,
  "files": [
    { "path": "logs/app.log" },
    { "path": "logs/old.log" }
  ],
  "warnings": [ { "code": "archive_member", "count": 1, "paths": ["logs/a.zip!/inner/zip.log"] } ]
}
```

## E139.5 The real call is unchanged

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "traversal": {"archives": "all"}}
```
```text response
deleted: 2  skipped: 1

logs/
app.log
old.log

warning: archive_member (1): logs/a.zip!/inner/zip.log
```
```text trash
logs/
logs/app.log
logs/old.log
```

## E139.6 Spec 1 keeps its header

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"version": 1, "query": "name ~ '*.log'", "dry_run": true, "permanent": true}
```
```text response
deleted: 2  permanent  dry run

logs/
app.log
old.log
```

## E139.7 `expect` which fails: the error is unchanged

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "expect": 3, "dry_run": true}
```
```text error
error[count_mismatch]: the selection has 2 files, expected 3
  hint: nothing was changed; check the selection with dry_run: true
```

## E139.8 Rename

Fixture: `rename`

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "name ~ '*.md'", "to": "archive/{path}", "dry_run": true}
```
```text response
dry run  would rename: 2

./
README.md -> archive/README.md
docs/
guide.md -> archive/docs/guide.md
```

## E139.9 Copy

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

```json request copy
{"query": "name ~ '*.rs'", "to": "backup/{path}", "dry_run": true}
```
```text response
dry run  would copy: 3

src/
lib.bak.rs -> backup/src/lib.bak.rs
lib.rs -> backup/src/lib.rs
main.rs -> backup/src/main.rs
```

## E139.10 Replace

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'naming.rs' and text has 'parse query' /n @old", "replace": {"@old": "build request"}, "dry_run": true, "output": {"max_lines": 4}}
```
```text response
dry run  would replace: 1  matches: 6

src/
naming.rs  matches: 6
-1:fn parseQuery() {}
-2:const PARSE_QUERY: u8 = 1;
-3:struct ParseQuery;
-4:mod parse_query {}
… 6 more lines
```
