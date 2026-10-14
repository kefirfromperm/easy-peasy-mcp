# Examples for task 9

Spec: `1`

## Fixtures

### types

```text tree
.gitignore                 2026-10-01 10:00
.hidden/                   2026-10-01 10:00
.hidden/x.txt              2026-10-01 10:00
api/                       2026-10-01 10:00
api/handlers.py            2026-10-01 10:00
build/                     2026-10-01 10:00
build/out.txt              2026-10-01 10:00
docs/                      2026-10-01 10:00
docs/guide.md              2026-10-01 10:00
latest                     2026-10-01 10:00
lib/                       2026-10-01 10:00
lib/config.rs              2026-10-01 10:00
lib/current.rs             2026-10-01 10:00
lib/main.rs                2026-10-01 10:00
lib/sub/                   2026-10-01 10:00
lib/sub/util.rs            2026-10-01 10:00
```

```text setup
symlink lib/current.rs -> config.rs
symlink latest -> lib
```

File contents are in `tests/fixtures/types/`; the symlinks are created by the test setup. Times are UTC, also of the
directories and the symlinks. `.hidden/` is hidden and `build/` is ignored by `.gitignore`; symlinks are not
followed.

## E9.1 `type = dir`

Directories end with `/`. `lib/` is an entry of the working directory and the header of its own entries.

Fixture: `types`

```json request
{ "query": "type = dir" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 type = dir

./
api/  [#1]
docs/  [#1]
lib/  [#1]
lib/
sub/  [#1]
```

## E9.2 A directory by name

Fixture: `types`

```json request
{ "query": "type = dir and name = 'sub'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = dir
#2 name = 'sub'

lib/
sub/  [#1 #2]
```

## E9.3 `type = link`: symlinks to a directory and to a file

Fixture: `types`

```json request
{ "query": "type = link" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = link

./
latest  [#1]
lib/
current.rs  [#1]
```

## E9.4 `type in ( ... )`

Fixture: `types`

```json request
{ "query": "type in (file, link) and name ends '.rs'" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 type in (file, link)
#2 name ends '.rs'

lib/
config.rs  [#1 #2]
main.rs  [#1 #2]
lib/sub/
util.rs  [#1 #2]
lib/
current.rs  [#1 #2]
```

## E9.5 `type != file`

Fixture: `types`

```json request
{ "query": "type != file and name starts 'l'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type != file
#2 name starts 'l'

./
latest  [#1 #2]
lib/  [#1 #2]
```

## E9.6 Values in any case

Fixture: `types`

```json request
{ "query": "TYPE = DIR and name = 'docs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 TYPE = DIR
#2 name = 'docs'

./
docs/  [#1 #2]
```

## E9.7 Without a `type` condition only regular files are returned

The symlink `lib/current.rs` is not returned: the implicit `type = file` takes only regular files.

Fixture: `types`

```json request
{ "query": "name ends '.rs'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ends '.rs'

lib/
config.rs  [#1]
main.rs  [#1]
lib/sub/
util.rs  [#1]
```

## E9.8 Unknown value

Fixture: `types`

```json request
{ "query": "type = folder" }
```

```text error
error[unknown_value]: unknown value 'folder'
  type = folder
         ^^^^^^
  hint: did you mean 'dir'? expected one of: file, dir, link
```
