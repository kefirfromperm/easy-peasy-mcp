# Examples for task 127

The `copy` tool ([TOOLS.md](../TOOLS.md#copy)). It is a write tool: the selection, `expect`, `dry_run`, the skipped
entries and the response follow the shared rules of task 125, the target and the conflicts the ones of task 126, so
these examples are about what is copied. Most are startup examples with `--allow-write`; a replaced file goes to the
trash of the test hook, listed by a `text trash` block (AGENTS.md).

## Fixtures

### copy

```text tree
README.md            2026-10-01 10:00
docs/.draft.md       2026-10-01 10:00
docs/api.md          2026-10-01 10:00
docs/guide.md        2026-10-01 10:00
src/lib.bak.rs       2026-10-01 10:00
src/lib.rs           2026-10-01 10:00
src/main.rs          2026-10-01 10:00
```

```text setup
symlink docs/latest.md -> guide.md
```

File contents are in `tests/fixtures/copy/`: `src/lib.rs` holds `pub fn lib() {}`, `src/lib.bak.rs` holds
`// old lib`.

## E127.1 `--allow-write` lists `copy`

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}, {"name": "delete"}, {"name": "rename"}, {"name": "copy", "annotations": {"readOnlyHint": false, "destructiveHint": true, "idempotentHint": true, "openWorldHint": false}}, {"name": "replace"}]}}
```

## E127.2 Without `allow_write`

Fixture: `copy`

```json request copy
{"query": "name ~ '*.md'", "to": "{dir}/{stem}.bak.{ext}"}
```
```text error
error[write_disabled]: write tools are disabled
  hint: the configuration has server.allow_write = false
```

## E127.3 Back up the files of a result next to themselves

Fixture: `copy`

The copies keep the modification time of their files. The symlink `docs/latest.md` is not walked by `find` (it doesn't
follow symlinks), so it is not in the result.

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "name ~ '*.md'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~ '*.md'

./
README.md  [#1]
docs/
api.md  [#1]
guide.md  [#1]
```

2. copy its files

```json request copy
{"result": "r1", "to": "{dir}/{stem}.bak.{ext}"}
```
```text response
copied: 3

./
README.md -> README.bak.md
docs/
api.md -> docs/api.bak.md
guide.md -> docs/guide.bak.md
```

3. the copies, with the times of the originals

```json request
{"query": "name ~ '*.bak.md' and modified < 2026-10-02", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r2
#1 name ~ '*.bak.md'
#2 modified < 2026-10-02

./
README.bak.md  [#1 #2]
docs/
api.bak.md  [#1 #2]
guide.bak.md  [#1 #2]
```

4. the originals are still there

```json request
{"dirs": ["docs"], "query": "name ~ 'guide*'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r3
#1 name ~ 'guide*'

docs/
guide.bak.md  [#1]
guide.md  [#1]
```

## E127.4 A directory with its whole tree

Fixture: `copy`

`docs` is copied into `backup`, which is created, with everything in it: the hidden `.draft.md`, which the request
doesn't walk, and the symlink, which stays a symlink (`find` lists it only with `type = link`).

```text command
easy-peasy-mcp --allow-write
```

```json request copy
{"query": "type = dir and name = 'docs'", "to": "backup/{path}"}
```
```text response
copied: 1

./
docs/ -> backup/docs/
```

```json request
{"dirs": ["backup"], "traversal": {"hidden": true}, "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1

backup/docs/
.draft.md
api.md
guide.md
```

```json request
{"dirs": ["backup"], "query": "type = link", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r2
#1 type = link

backup/docs/
latest.md  [#1]
```

## E127.5 A symlink is copied as a symlink

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

```json request copy
{"query": "type = link", "to": "{dir}/current.md"}
```
```text response
copied: 1

docs/
latest.md -> docs/current.md
```

```json request
{"query": "type = link", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = link

docs/
current.md  [#1]
latest.md  [#1]
```

## E127.6 A target which exists

Fixture: `copy`

`src/lib.bak.rs` exists: with `fail` nothing is copied, also in a dry run; with `skip` the other file is copied; with
`overwrite` the existing file goes to the trash.

```text command
easy-peasy-mcp --allow-write
```

1. fail

```json request copy
{"query": "path ~ 'src/*.rs' and not name ~ '*.bak.rs'", "to": "{dir}/{stem}.bak.{ext}"}
```
```text error
error[copy_conflict]: targets conflict; nothing was changed
  src/lib.rs -> src/lib.bak.rs: exists
  hint: on_conflict = "skip" copies the other files, "overwrite" replaces the existing ones
```

2. a dry run

```json request copy
{"query": "path ~ 'src/*.rs' and not name ~ '*.bak.rs'", "to": "{dir}/{stem}.bak.{ext}", "dry_run": true}
```
```text error
error[copy_conflict]: targets conflict; nothing was changed
  src/lib.rs -> src/lib.bak.rs: exists
  hint: on_conflict = "skip" copies the other files, "overwrite" replaces the existing ones
```

3. skip

```json request copy
{"query": "path ~ 'src/*.rs' and not name ~ '*.bak.rs'", "to": "{dir}/{stem}.bak.{ext}", "on_conflict": "skip"}
```
```text response
copied: 1  skipped: 1

src/
main.rs -> src/main.bak.rs

warning: target_conflict (1): src/lib.rs
```

4. overwrite

```json request copy
{"query": "name = 'lib.rs'", "to": "{dir}/{stem}.bak.{ext}", "on_conflict": "overwrite"}
```
```text response
copied: 1

src/
lib.rs -> src/lib.bak.rs  replaced
```
```text trash
src/
src/lib.bak.rs
```

```json request
{"dirs": ["src"], "query": "text has 'pub fn lib'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'pub fn lib'

src/
lib.bak.rs  [#1]  lines: 1  matches: 1
lib.rs  [#1]  lines: 1  matches: 1
```

## E127.7 Two files to one target

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

1. fail

```json request copy
{"query": "name ~ '*.md'", "to": "all.md"}
```
```text error
error[copy_conflict]: targets conflict; nothing was changed
  README.md -> all.md: the same target as another file
  docs/api.md -> all.md: the same target as another file
  docs/guide.md -> all.md: the same target as another file
  hint: on_conflict = "skip" copies the other files, "overwrite" replaces the existing ones
```

2. overwrite

```json request copy
{"query": "name ~ '*.md'", "to": "all.md", "on_conflict": "overwrite"}
```
```text error
error[copy_conflict]: targets conflict; nothing was changed
  README.md -> all.md: the same target as another file
  docs/api.md -> all.md: the same target as another file
  docs/guide.md -> all.md: the same target as another file
  hint: on_conflict = "skip" copies the other files
```

## E127.8 A directory into itself

Fixture: `copy`

The target of `docs` is inside `docs`: `overwrite` doesn't resolve it, `skip` skips it.

```text command
easy-peasy-mcp --allow-write
```

1. overwrite

```json request copy
{"query": "type = dir and name = 'docs'", "to": "{path}/old", "on_conflict": "overwrite"}
```
```text error
error[copy_conflict]: targets conflict; nothing was changed
  docs/ -> docs/old/: the target is inside the copied directory
  hint: on_conflict = "skip" copies the other files
```

2. skip

```json request copy
{"query": "type = dir and name = 'docs'", "to": "{path}/old", "on_conflict": "skip"}
```
```text error
copied: 0  skipped: 1

warning: target_conflict (1): docs/
```

## E127.9 Targets which can't be used

Fixture: `copy`

A target outside of the allowed roots is skipped with `permission_denied`; a target which is the old path is not
copied and listed as `unchanged`.

```text command
easy-peasy-mcp --allow-write
```

1. outside

```json request copy
{"query": "name = 'main.rs'", "to": "../../{name}"}
```
```text error
copied: 0  skipped: 1

warning: permission_denied (1): src/main.rs
```

2. the old path

```json request copy
{"query": "name = 'main.rs'", "to": "{path}"}
```
```text response
copied: 0  unchanged: 1

src/
main.rs  unchanged
```

## E127.10 A dry run

Fixture: `copy`

Nothing is copied and no directory is created.

```text command
easy-peasy-mcp --allow-write
```

```json request copy
{"query": "name ~ '*.rs'", "to": "backup/{path}", "dry_run": true}
```
```text response
copied: 3  dry run

src/
lib.bak.rs -> backup/src/lib.bak.rs
lib.rs -> backup/src/lib.rs
main.rs -> backup/src/main.rs
```

```json request
{"query": "type = dir", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = dir

./
docs/  [#1]
src/  [#1]
```

## E127.11 An invalid template

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

```json request copy
{"query": "name ~ '*.rs'", "to": "{dir}/{base}.rs"}
```
```text error
error[invalid_template]: unknown placeholder 'base'
  {dir}/{base}.rs
        ^^^^^^
  hint: the placeholders are dir, name, stem, ext, path; groups need a 'pattern'
```

## E127.12 The JSON format

Fixture: `copy`

```text command
easy-peasy-mcp --allow-write
```

1. a conflict

```json request copy
{"query": "name = 'lib.rs'", "to": "{dir}/{stem}.bak.{ext}", "output": {"format": "json"}}
```
```json error
{"error": {"code": "copy_conflict", "message": "targets conflict; nothing was changed", "conflicts": [{"path": "src/lib.rs", "to": "src/lib.bak.rs", "reason": "exists"}], "hint": "on_conflict = \"skip\" copies the other files, \"overwrite\" replaces the existing ones"}}
```

2. a dry run of an overwrite

```json request copy
{"query": "name = 'lib.rs'", "to": "{dir}/{stem}.bak.{ext}", "on_conflict": "overwrite", "dry_run": true, "output": {"format": "json"}}
```
```json response
{"version": 1, "copied": 1, "dry_run": true, "files": [{"path": "src/lib.rs", "to": "src/lib.bak.rs", "replaced": true}]}
```

3. copied and unchanged

```json request copy
{"query": "path ~ 'src/*.rs' and not name ~ '*.bak.rs'", "pattern": "src/(main|lib)\\.rs", "to": "{dir}/{1}.rs", "output": {"format": "json"}}
```
```json response
{
  "version": 1,
  "copied": 0,
  "unchanged": 2,
  "files": [
    { "path": "src/lib.rs", "unchanged": true },
    { "path": "src/main.rs", "unchanged": true }
  ]
}
```

4. a copy

```json request copy
{"query": "name = 'main.rs'", "to": "{dir}/{stem}.orig.{ext}", "output": {"format": "json"}}
```
```json response
{"version": 1, "copied": 1, "files": [{"path": "src/main.rs", "to": "src/main.orig.rs"}]}
```

```text trash
```
