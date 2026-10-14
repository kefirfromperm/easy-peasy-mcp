# Examples for task 10

Spec: `1`

## Fixtures

### empties

```text tree
.gitignore                 2026-10-01 10:00
cache/                     2026-10-01 10:00
logs/                      2026-10-01 10:00
logs/.keep                 2026-10-01 10:00
notes/                     2026-10-01 10:00
notes/a.txt                2026-10-01 10:00
notes/b.txt                2026-10-01 10:00
notes/c.txt                2026-10-01 10:00
notes/d.txt                2026-10-01 10:00
out/                       2026-10-01 10:00
tmp/                       2026-10-01 10:00
tmp/run.log                2026-10-01 10:00
```

```text setup
mkdir cache
mkdir out
```

File contents are in `tests/fixtures/empties/`; `notes/b.txt` and `notes/c.txt` are empty; the empty directories
`cache/` and `out/` are created by the test setup. `logs/` holds only the hidden `.keep`, `tmp/` only the ignored
`run.log`: both are not empty. Times are UTC.

## E10.1 Empty files

Fixture: `empties`

```json request
{ "query": "empty" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 empty

notes/
b.txt  [#1]
c.txt  [#1]
```

## E10.2 `not empty`

The files match only through `not`, so no condition is listed.

Fixture: `empties`

```json request
{ "query": "not empty" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 empty

notes/
a.txt
d.txt
```

## E10.3 Empty directories

`logs/` and `tmp/` have entries on disk, hidden or ignored ones, so they are not empty.

Fixture: `empties`

```json request
{ "query": "type = dir and empty" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = dir
#2 empty

./
cache/  [#1 #2]
out/  [#1 #2]
```

## E10.4 `empty` with other conditions

Fixture: `empties`

```json request
{ "query": "EMPTY and name ends 'c.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 EMPTY
#2 name ends 'c.txt'

notes/
c.txt  [#1 #2]
```

## E10.5 `empty` takes no operator

Fixture: `empties`

```json request
{ "query": "empty = true" }
```

```text error
error[unexpected_token]: unexpected '='
  empty = true
        ^
  hint: 'empty' stands alone, without an operator and a value
```
