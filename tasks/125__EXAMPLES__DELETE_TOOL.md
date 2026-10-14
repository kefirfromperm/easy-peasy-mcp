# Examples for task 125

The `delete` tool and the shared rules of the write tools ([TOOLS.md](../TOOLS.md#write-tools)). The write tools
need `server.allow_write = true`, which only the config file and the command line set, so most examples are startup
examples with `--allow-write`. They never touch the trash of the machine: the test hook of the library moves the
entries into the directory `trash` next to the fixture, keeping their paths in the allowed root, and a `text trash`
block lists what it holds after the request before it (AGENTS.md).

## Fixtures

### delete

```text tree
README.md            2026-10-01 10:00
build/cache/a.o      2026-10-01 10:00
build/out.txt        2026-10-01 10:00
data/x.txt           2026-10-01 10:00
logs/a.zip           2026-10-01 10:00
logs/app.log         2026-10-01 10:00
logs/old.log         2026-10-01 10:00
src/legacy.rs        2026-10-01 10:00
src/main.rs          2026-10-01 10:00
src/old.rs           2026-10-01 10:00
```

File contents are in `tests/fixtures/delete/`: `src/old.rs` and `src/legacy.rs` start with `// TODO remove`,
`logs/a.zip` holds `inner/zip.log` and `inner/nested.zip`.

## E125.1 Write tools are off by default

Fixture: `delete`

`delete` is not listed without `server.allow_write = true`.

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}]}}
```

## E125.2 A call made anyway fails

Fixture: `delete`

```json request delete
{"query": "name ~ '*.log'"}
```
```text error
error[write_disabled]: write tools are disabled
  hint: the configuration has server.allow_write = false
```

Nothing was deleted:

```json request
{"query": "name ~ '*.log'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

logs/
app.log  [#1]
old.log  [#1]
```

## E125.3 `--allow-write` lists the write tools

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find", "annotations": {"readOnlyHint": true, "openWorldHint": false}}, {"name": "get_config"}, {"name": "update_config"}, {"name": "delete", "annotations": {"readOnlyHint": false, "destructiveHint": true, "idempotentHint": true, "openWorldHint": false}}, {"name": "rename"}, {"name": "copy"}, {"name": "replace"}]}}
```

## E125.4 `get_config` shows `allow_write`

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request get_config
{"section": "server", "sources": true}
```
```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
allow_update = true    # default
allow_write = true    # cli
result_cache = 50    # default
output_dir = '<output_dir>'    # default
max_output_file_size = "100M"    # default
```

## E125.5 `update_config` can't change `allow_write`

Fixture: `delete`

```json request update_config
{"set": "server.allow_write = true"}
```
```text error
error[read_only]: 'server.allow_write' is read-only
  hint: allow_write is set only at start
```

## E125.6 `allow_write` in the config file

Fixture: `delete`

```toml config.toml
[server]
allow_write = true
```
```text command
easy-peasy-mcp --config config.toml
```

```json request get_config
{"section": "server", "sources": true}
```
```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
allow_update = true    # default
allow_write = true    # file: config.toml
result_cache = 50    # default
output_dir = '<output_dir>'    # default
max_output_file_size = "100M"    # default
```

## E125.7 An invalid `allow_write`

Fixture: `delete`

```toml config.toml
[server]
allow_write = "yes"
```
```text command
easy-peasy-mcp --config config.toml
```
```text stderr exit 1
easy-peasy-mcp: config.toml:2: invalid value for 'server.allow_write': true or false is expected
```

## E125.8 Delete the files of a result

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "text has 'TODO remove'", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO remove'

src/
legacy.rs  [#1]  lines: 1  matches: 1
old.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): logs/a.zip
```

2. delete its files

```json request delete
{"result": "r1"}
```
```text response
deleted: 2

src/
legacy.rs
old.rs
```
```text trash
src/
src/legacy.rs
src/old.rs
```

3. the rest

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name ~ '*.rs'

src/
main.rs  [#1]
```

## E125.9 Delete by a query

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'"}
```
```text response
deleted: 2

logs/
app.log
old.log
```
```text trash
logs/
logs/app.log
logs/old.log
```

```json request
{"dirs": ["logs"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r1

logs/
a.zip
```

## E125.10 A directory as a whole

Fixture: `delete`

`type = dir` selects the directory; `build/out.txt` is inside of it, so it goes with it: `expect` counts the two
selected entries, the response lists one.

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "(type = dir and name = 'build') or name = 'out.txt'", "expect": 2}
```
```text response
deleted: 1

./
build/
```
```text trash
build/
build/cache/
build/cache/a.o
build/out.txt
```

```json request
{"query": "type = dir", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1
#1 type = dir

./
data/  [#1]
logs/  [#1]
src/  [#1]
```

## E125.11 A dry run changes nothing

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "dry_run": true}
```
```text response
deleted: 2  dry run

logs/
app.log
old.log
```
```text trash
```

```json request
{"query": "name ~ '*.log'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

logs/
app.log  [#1]
old.log  [#1]
```

## E125.12 `expect` which matches

Fixture: `delete`

Without directory groups the paths are full.

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "expect": 2, "output": {"group_dirs": false}}
```
```text response
deleted: 2

logs/app.log
logs/old.log
```
```text trash
logs/
logs/app.log
logs/old.log
```

## E125.13 `expect` which doesn't match

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "expect": 1}
```
```text error
error[count_mismatch]: the selection has 2 files, expected 1
  hint: nothing was changed; check the selection with dry_run: true
```
```text trash
```

```json request
{"query": "name ~ '*.log'", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

logs/
app.log  [#1]
old.log  [#1]
```

## E125.14 Files changed or deleted since the result

Fixture: `delete`

`src/old.rs` gets another size, `src/legacy.rs` is deleted after the find; both are skipped, `src/main.rs` is
deleted.

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src/
legacy.rs  [#1]
main.rs  [#1]
old.rs  [#1]
```

2. the files change

```text setup
file src/old.rs 3
remove src/legacy.rs
```

3. delete

```json request delete
{"result": "r1"}
```
```text response
deleted: 1  skipped: 2

src/
main.rs

warning: file_changed (2): src/legacy.rs, src/old.rs
```
```text trash
src/
src/main.rs
```

## E125.15 A member of an archive

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

## E125.16 A file outside of the allowed roots

Fixture: `delete`

After the find `data` becomes a symlink to a directory outside of the fixture: the path of the result leads out of
the allowed root, so it is skipped. Nothing of the selection was done: `isError` is true.

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "name = 'x.txt'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'x.txt'

data/
x.txt  [#1]
```

2. `data` leads outside

```text setup
file ../outside/x.txt 5
remove data
symlink data -> ../outside
```

3. delete

```json request delete
{"result": "r1"}
```
```text error
deleted: 0  skipped: 1

warning: permission_denied (1): data/x.txt
```
```text trash
```

## E125.17 A symlink is deleted itself

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```text setup
symlink logs/latest.log -> app.log
```

```json request delete
{"query": "type = link"}
```
```text response
deleted: 1

logs/
latest.log
```
```text trash
logs/
logs/latest.log -> app.log
```

The target stays:

```json request
{"dirs": ["logs"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1

logs/
a.zip
app.log
old.log
```

## E125.18 `permanent`

Fixture: `delete`

The files are deleted, not moved to the trash.

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name ~ '*.log'", "permanent": true}
```
```text response
deleted: 2  permanent

logs/
app.log
old.log
```
```text trash
```

```json request
{"dirs": ["logs"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r1

logs/
a.zip
```

## E125.19 No trash

Fixture: `delete`

`EASY_PEASY_TEST_TRASH=none` is the test hook of a file system without a trash: the files are skipped, not deleted
permanently (nothing done: `isError` is true); `permanent` deletes them.

```text env
EASY_PEASY_TEST_TRASH=none
```
```text command
easy-peasy-mcp --allow-write
```

1. to the trash

```json request delete
{"query": "name ~ '*.log'"}
```
```text error
deleted: 0  skipped: 2

warning: trash_unavailable (2): logs/app.log, logs/old.log
```

2. permanently

```json request delete
{"query": "name ~ '*.log'", "permanent": true}
```
```text response
deleted: 2  permanent

logs/
app.log
old.log
```

## E125.20 An empty selection

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

```json request delete
{"query": "name = 'missing.txt'"}
```
```text response
deleted: 0
```

## E125.21 Invalid selections

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

1. `result` and `query`

```json request delete
{"result": "r1", "query": "name ~ '*.log'"}
```
```text error
error[invalid_option]: 'result' and 'query' exclude each other
```

2. neither

```json request delete
{"expect": 2}
```
```text error
error[invalid_option]: 'result' or 'query' is required
  hint: pass the result id of a find, or a query
```

3. an expired result

```json request delete
{"result": "r7"}
```
```text error
error[result_expired]: the result r7 is unknown or no longer kept
  hint: the server keeps the last server.result_cache results
```

4. `dirs` with `result`

```json request delete
{"result": "r1", "dirs": ["logs"]}
```
```text error
error[invalid_option]: 'dirs' works only with 'query'
```

5. not a result id

```json request delete
{"result": "12"}
```
```text error
error[invalid_option]: 'result' must be a result id like 'r12'
```

6. an unknown field

```json request delete
{"query": "name ~ '*.log'", "dryrun": true}
```
```text error
error[invalid_option]: unknown field 'dryrun'
  hint: did you mean 'dry_run'? expected one of: version, result, query, dirs, traversal, kinds, languages, expect, dry_run, permanent, timeout, output
```

7. not a boolean

```json request delete
{"query": "name ~ '*.log'", "permanent": "yes"}
```
```text error
error[invalid_option]: 'permanent' must be a boolean
```

```text trash
```

## E125.22 The JSON format

Fixture: `delete`

```text command
easy-peasy-mcp --allow-write
```

1. a dry run

```json request delete
{"query": "name ~ '*.log'", "traversal": {"archives": "all"}, "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 1,
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

2. an error

```json request delete
{"query": "name ~ '*.log'", "expect": 3, "output": {"format": "json"}}
```
```json error
{"error": {"code": "count_mismatch", "message": "the selection has 2 files, expected 3", "hint": "nothing was changed; check the selection with dry_run: true"}}
```

3. delete

```json request delete
{"query": "name ~ '*.log'", "output": {"format": "json"}}
```
```json response
{"version": 1, "deleted": 2, "files": [{"path": "logs/app.log"}, {"path": "logs/old.log"}]}
```
```text trash
logs/
logs/app.log
logs/old.log
```
