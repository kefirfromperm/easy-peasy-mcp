# Examples for task 33

Spec: `1`

## Fixtures

### perms

```text tree
root-only/                 2026-10-01 10:00
root-only/x.txt            2026-10-01 10:00
secrets/                   2026-10-01 10:00
secrets/a.key              2026-10-01 10:00
secrets/b.key              2026-10-01 10:00
secrets/public.key         2026-10-01 10:00
src/                       2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

```text setup
chmod secrets/a.key 000
chmod secrets/b.key 000
chmod root-only 000
```

File contents are in `tests/fixtures/perms/`: `src/main.rs` has `// TODO: check`, `secrets/a.key`, `secrets/b.key`
and `root-only/x.txt` have a `TODO` too, `secrets/public.key` has none. The setup removes all permissions of the two
keys and of `root-only/`, so the examples run only where permissions can be removed (Unix, not as root). Times are
UTC.

## E33.1 Unreadable files and directories are skipped and reported

The content of every file is needed: the keys can't be opened, `root-only/` can't be listed.

Fixture: `perms`

```json request
{ "query": "text has 'TODO'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO'

src/
main.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧: check

warning: permission_denied (3): root-only/, secrets/a.key, secrets/b.key
```

## E33.2 A matched file is opened before it is returned

The two keys match by name, but they can't be opened.

Fixture: `perms`

```json request
{ "query": "name ~ '*.key'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.key'

secrets/
public.key  [#1]

warning: permission_denied (3): root-only/, secrets/a.key, secrets/b.key
```

## E33.3 An unreadable directory is not returned by `type = dir`

Fixture: `perms`

```json request
{ "query": "type = dir" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = dir

./
secrets/  [#1]
src/  [#1]

warning: permission_denied (1): root-only/
```

## E33.4 Files which don't match are not opened

Fixture: `perms`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]

warning: permission_denied (1): root-only/
```
