# Examples for task 126

The `rename` tool ([TOOLS.md](../TOOLS.md#rename)). It is a write tool: the selection, `expect`, `dry_run`, the
skipped entries and the response follow the shared rules of task 125, so these examples are about the target, the
conflicts and the response. Most are startup examples with `--allow-write`; a replaced file goes to the trash of the
test hook, listed by a `text trash` block (AGENTS.md).

## Fixtures

### rename

```text tree
README.md            2026-10-01 10:00
docs/guide.md        2026-10-01 10:00
lib/a.js             2026-10-01 10:00
lib/b.js             2026-10-01 10:00
src/eval_test.rs     2026-10-01 10:00
src/main.rs          2026-10-01 10:00
src/parse_test.rs    2026-10-01 10:00
web/app.ts           2026-10-01 10:00
web/view.ts          2026-10-01 10:00
web/view.tsx         2026-10-01 10:00
```

File contents are in `tests/fixtures/rename/`: `web/view.ts` holds `export const view = 1;`, `web/view.tsx` holds
`export const View = () => null;`.

## E126.1 `--allow-write` lists `rename`

Fixture: `rename`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}, {"name": "delete"}, {"name": "rename", "annotations": {"readOnlyHint": false, "destructiveHint": true, "idempotentHint": false, "openWorldHint": false}}, {"name": "copy"}, {"name": "replace"}]}}
```

## E126.2 Without `allow_write`

Fixture: `rename`

```json request rename
{"query": "name ~ '*.js'", "to": "{dir}/{stem}.ts"}
```
```text error
error[write_disabled]: write tools are disabled
  hint: the configuration has server.allow_write = false
```

## E126.3 Change the extension of the files of a result

Fixture: `rename`

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "name ~ '*.js'", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.js'

lib/
a.js  [#1]
b.js  [#1]
```

2. rename its files

```json request rename
{"result": "r1", "to": "{dir}/{stem}.ts"}
```
```text response
renamed: 2

lib/
a.js -> lib/a.ts
b.js -> lib/b.ts
```

3. the directory after

```json request
{"dirs": ["lib"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r2

lib/
a.ts
b.ts
```

## E126.4 Move under a directory which doesn't exist

Fixture: `rename`

`archive` and `archive/docs` are created.

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "name ~ '*.md'", "to": "archive/{path}"}
```
```text response
renamed: 2

./
README.md -> archive/README.md
docs/
guide.md -> archive/docs/guide.md
```

```json request
{"dirs": ["archive"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1

archive/
README.md
archive/docs/
guide.md
```

## E126.5 A pattern with a group

Fixture: `rename`

`src/main.rs` doesn't match the pattern: it is skipped.

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "path ~ 'src/*.rs'", "pattern": "src/(.*)_test\\.rs", "to": "tests/{1}.rs"}
```
```text response
renamed: 2  skipped: 1

src/
eval_test.rs -> tests/eval.rs
parse_test.rs -> tests/parse.rs

warning: pattern_mismatch (1): src/main.rs
```

```json request
{"dirs": ["src", "tests"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1

src/
main.rs
tests/
eval.rs
parse.rs
```

## E126.6 A named group in another working directory

Fixture: `rename`

The pattern and the template are relative to the working directory `lib`; the paths of the response have its prefix.

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"dirs": ["lib"], "query": "name ~ '*.js'", "pattern": "(?<base>.*)\\.js", "to": "{base}.mjs"}
```
```text response
renamed: 2

lib/
a.js -> lib/a.mjs
b.js -> lib/b.mjs
```

## E126.7 A target which exists

Fixture: `rename`

`web/view.tsx` exists: with `fail` nothing is renamed, also in a dry run; with `skip` the other file is renamed.

```text command
easy-peasy-mcp --allow-write
```

1. fail

```json request rename
{"query": "name ~ '*.ts'", "to": "{dir}/{stem}.tsx"}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  web/view.ts -> web/view.tsx: exists
  hint: on_conflict = "skip" renames the other files, "overwrite" replaces the existing ones
```

2. a dry run

```json request rename
{"query": "name ~ '*.ts'", "to": "{dir}/{stem}.tsx", "dry_run": true}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  web/view.ts -> web/view.tsx: exists
  hint: on_conflict = "skip" renames the other files, "overwrite" replaces the existing ones
```

3. skip

```json request rename
{"query": "name ~ '*.ts'", "to": "{dir}/{stem}.tsx", "on_conflict": "skip"}
```
```text response
renamed: 1  skipped: 1

web/
app.ts -> web/app.tsx

warning: target_conflict (1): web/view.ts
```

```json request
{"dirs": ["web"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 0  matches: 0  result: r1

web/
app.tsx
view.ts
view.tsx
```

## E126.8 Two files to one target

Fixture: `rename`

Two files to one target are never overwritten: `overwrite` fails like `fail`, `skip` skips both, so nothing of the
selection is done (`isError` is true).

```text command
easy-peasy-mcp --allow-write
```

1. fail

```json request rename
{"query": "name ~ '*.js'", "to": "lib/all.js"}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  lib/a.js -> lib/all.js: the same target as another file
  lib/b.js -> lib/all.js: the same target as another file
  hint: on_conflict = "skip" renames the other files, "overwrite" replaces the existing ones
```

2. overwrite

```json request rename
{"query": "name ~ '*.js'", "to": "lib/all.js", "on_conflict": "overwrite"}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  lib/a.js -> lib/all.js: the same target as another file
  lib/b.js -> lib/all.js: the same target as another file
  hint: on_conflict = "skip" renames the other files
```

3. skip

```json request rename
{"query": "name ~ '*.js'", "to": "lib/all.js", "on_conflict": "skip"}
```
```text error
renamed: 0  skipped: 2

warning: target_conflict (2): lib/a.js, lib/b.js
```

## E126.9 Another selected file, and an unchanged one

Fixture: `rename`

The target of `lib/b.js` is its own path: it is unchanged and doesn't conflict. The target of `lib/a.js` is
`lib/b.js`, another file of the selection. An unchanged entry counts as done: with `skip` the call is not an error.

```text command
easy-peasy-mcp --allow-write
```

1. fail

```json request rename
{"query": "name ~ '*.js'", "to": "{dir}/b.js"}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  lib/a.js -> lib/b.js: the target is another selected file
  hint: on_conflict = "skip" renames the other files, "overwrite" replaces the existing ones
```

2. skip

```json request rename
{"query": "name ~ '*.js'", "to": "{dir}/b.js", "on_conflict": "skip"}
```
```text response
renamed: 0  unchanged: 1  skipped: 1

lib/
b.js  unchanged

warning: target_conflict (1): lib/a.js
```

## E126.10 Overwrite

Fixture: `rename`

The existing `web/view.tsx` goes to the trash and `web/view.ts` takes its place. A directory is never replaced.

```text command
easy-peasy-mcp --allow-write
```

1. a file

```json request rename
{"query": "name = 'view.ts'", "to": "{dir}/{stem}.tsx", "on_conflict": "overwrite"}
```
```text response
renamed: 1

web/
view.ts -> web/view.tsx  replaced
```
```text trash
web/
web/view.tsx
```

```json request
{"dirs": ["web"], "query": "text has 'view = 1'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'view = 1'

web/
view.tsx  [#1]  lines: 1  matches: 1
```

2. a directory

```json request rename
{"query": "type = dir and name = 'docs'", "to": "lib", "on_conflict": "overwrite"}
```
```text error
error[rename_conflict]: targets conflict; nothing was changed
  docs/ -> lib/: a directory is never replaced
  hint: on_conflict = "skip" renames the other files
```

## E126.11 Targets which can't be used

Fixture: `rename`

A target outside of the allowed roots is skipped with `permission_denied`, an empty one with `invalid_target`;
nothing of the selection is done.

```text command
easy-peasy-mcp --allow-write
```

1. outside

```json request rename
{"query": "name = 'main.rs'", "to": "../../{name}"}
```
```text error
renamed: 0  skipped: 1

warning: permission_denied (1): src/main.rs
```

2. empty

```json request rename
{"query": "name = 'main.rs'", "to": "{dir}/.."}
```
```text error
renamed: 0  skipped: 1

warning: invalid_target (1): src/main.rs
```

## E126.12 A dry run

Fixture: `rename`

Nothing is renamed and no directory is created.

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "name ~ '*.md'", "to": "archive/{path}", "dry_run": true}
```
```text response
renamed: 2  dry run

./
README.md -> archive/README.md
docs/
guide.md -> archive/docs/guide.md
```

```json request
{"query": "type = dir", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 4  lines: 0  matches: 0  result: r1
#1 type = dir

./
docs/  [#1]
lib/  [#1]
src/  [#1]
web/  [#1]
```

## E126.13 Invalid templates and options

Fixture: `rename`

```text command
easy-peasy-mcp --allow-write
```

1. an unknown placeholder

```json request rename
{"query": "name ~ '*.js'", "to": "{dir}/{base}.ts"}
```
```text error
error[invalid_template]: unknown placeholder 'base'
  {dir}/{base}.ts
        ^^^^^^
  hint: the placeholders are dir, name, stem, ext, path; groups need a 'pattern'
```

2. a group which doesn't exist

```json request rename
{"query": "name ~ '*.js'", "pattern": "lib/(.*)\\.js", "to": "{2}.ts"}
```
```text error
error[invalid_template]: the group 2 doesn't exist
  {2}.ts
  ^^^
  hint: the placeholders are dir, name, stem, ext, path; the groups of 'pattern' are 1
```

3. an empty placeholder

```json request rename
{"query": "name ~ '*.js'", "to": "lib/{}.ts"}
```
```text error
error[invalid_template]: empty placeholder
  lib/{}.ts
      ^^
  hint: the placeholders are dir, name, stem, ext, path; groups need a 'pattern'
```

4. an unclosed brace

```json request rename
{"query": "name ~ '*.js'", "to": "{name.bak"}
```
```text error
error[invalid_template]: unclosed '{'
  {name.bak
  ^^^^^^^^^
  hint: write '{{' for a brace
```

5. an unmatched brace

```json request rename
{"query": "name ~ '*.js'", "to": "name}.bak"}
```
```text error
error[invalid_template]: unmatched '}'
  name}.bak
      ^
  hint: write '}}' for a brace
```

6. an absolute target

```json request rename
{"query": "name ~ '*.js'", "to": "/tmp/{name}"}
```
```text error
error[invalid_template]: the target must be relative
  /tmp/{name}
  ^
  hint: the target is relative to the working directory
```

7. a target which ends with `/`

```json request rename
{"query": "name ~ '*.js'", "to": "archive/"}
```
```text error
error[invalid_template]: the target ends with '/'
  archive/
         ^
  hint: end it with a name, e.g. 'archive/{name}'
```

8. an invalid pattern

```json request rename
{"query": "name ~ '*.js'", "pattern": "(?<=a)b", "to": "{name}"}
```
```text error
error[invalid_regex]: invalid regex
  (?<=a)b
  ^^^^^^^
  hint: look-arounds are not supported
```

9. a named group with the name of a placeholder

```json request rename
{"query": "name ~ '*.js'", "pattern": "(?<name>.*)", "to": "{name}"}
```
```text error
error[invalid_option]: the group 'name' of 'pattern' has the name of a placeholder
```

10. without `to`

```json request rename
{"query": "name ~ '*.js'"}
```
```text error
error[invalid_option]: 'to' is required
  hint: the new path, e.g. '{dir}/{stem}.bak'
```

11. an empty `to`

```json request rename
{"query": "name ~ '*.js'", "to": ""}
```
```text error
error[invalid_option]: 'to' must be a non-empty string
```

12. an unknown `on_conflict`

```json request rename
{"query": "name ~ '*.js'", "to": "{name}", "on_conflict": "replace"}
```
```text error
error[invalid_option]: 'on_conflict' must be one of: fail, skip, overwrite
```

```json request
{"dirs": ["lib"], "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 2  lines: 0  matches: 0  result: r1

lib/
a.js
b.js
```

## E126.14 The JSON format

Fixture: `rename`

```text command
easy-peasy-mcp --allow-write
```

1. a conflict which `overwrite` doesn't resolve: the target is another selected file

```json request rename
{"query": "path ~ 'web/*'", "to": "{dir}/{stem}.tsx", "on_conflict": "overwrite", "output": {"format": "json"}}
```
```json error
{"error": {"code": "rename_conflict", "message": "targets conflict; nothing was changed", "conflicts": [{"path": "web/view.ts", "to": "web/view.tsx", "reason": "selected"}], "hint": "on_conflict = \"skip\" renames the other files"}}
```

2. skip

```json request rename
{"query": "path ~ 'web/*'", "to": "{dir}/{stem}.tsx", "on_conflict": "skip", "output": {"format": "json"}}
```
```json response
{
  "version": 1,
  "renamed": 1,
  "unchanged": 1,
  "skipped": 1,
  "files": [
    { "path": "web/app.ts", "to": "web/app.tsx" },
    { "path": "web/view.tsx", "unchanged": true }
  ],
  "warnings": [ { "code": "target_conflict", "count": 1, "paths": ["web/view.ts"] } ]
}
```

3. a dry run of an overwrite

```json request rename
{"query": "name = 'view.ts'", "to": "{dir}/{stem}.tsx", "on_conflict": "overwrite", "dry_run": true, "output": {"format": "json"}}
```
```json response
{"version": 1, "renamed": 1, "dry_run": true, "files": [{"path": "web/view.ts", "to": "web/view.tsx", "replaced": true}]}
```

4. an invalid template

```json request rename
{"query": "name ~ '*.js'", "to": "{dir}/{base}.ts", "output": {"format": "json"}}
```
```json error
{"error": {"code": "invalid_template", "message": "unknown placeholder 'base'", "to": "{dir}/{base}.ts", "position": {"col": 7, "len": 6}, "hint": "the placeholders are dir, name, stem, ext, path; groups need a 'pattern'"}}
```

```text trash
```
