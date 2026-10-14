# Examples for task 132

Spec: `1`

`highlight = auto` in the text format writes a matched line without markers when its markers would add more
characters than the line has (OUTPUT.md#highlight).

## Fixtures

### auto-highlight

```text tree
boundary.txt               2026-10-01 10:00
dedent.rs                  2026-10-01 10:00
docs/table.md              2026-10-01 10:00
marked.txt                 2026-10-01 10:00
marker.txt                 2026-10-01 10:00
multi.txt                  2026-10-01 10:00
rep/a.txt                  2026-10-01 10:00
rep/b.txt                  2026-10-01 10:00
rep/c.txt                  2026-10-01 10:00
```

File contents are in `tests/fixtures/auto-highlight/`. `docs/table.md`:

```
| Task | Status |
|------|--------|
| Parse the query | done |
| Walk the file tree | todo |
```

`boundary.txt`: `a x b` (5 characters) and `ax b` (4). `dedent.rs`: `fn f() {`, `    x y`, `}`. `rep/a.txt`,
`rep/b.txt`, `rep/c.txt`: `let x = 1;` and `ax`. `marker.txt`: `⟦x⟧`. `marked.txt`: `see ⟦ the x ⟧ here`.
`multi.txt`: `a`, `b`, `long`, `line`.

## E132.1 A table: lines whose markers outgrow them are written plain

Fixture: `auto-highlight`

The three markers `⟦#2,#3:|⟧` of a line add 24 characters: more than the header and the separator have (17), less
than the rows (26 and 29).

```json request
{"query": "name ~ '*.md' and line(text has '|')"}
```

```text response
files: 1  lines: 4  matches: 12  result: r1
#1 name ~ '*.md'
#2 line(text has '|')
#3 text has '|'

docs/
table.md  [#1 #2]  lines: 4  matches: 12
1:| Task | Status |
2:|------|--------|
3:⟦#2,#3:|⟧ Parse the query ⟦#2,#3:|⟧ done ⟦#2,#3:|⟧
4:⟦#2,#3:|⟧ Walk the file tree ⟦#2,#3:|⟧ todo ⟦#2,#3:|⟧
```

## E132.2 `markers` set explicitly marks every fragment

Fixture: `auto-highlight`

```json request
{"query": "name ~ '*.md' and line(text has '|')", "output": {"highlight": "markers"}}
```

```text response
files: 1  lines: 4  matches: 12  result: r1
#1 name ~ '*.md'
#2 line(text has '|')
#3 text has '|'

docs/
table.md  [#1 #2]  lines: 4  matches: 12
1:⟦#2,#3:|⟧ Task ⟦#2,#3:|⟧ Status ⟦#2,#3:|⟧
2:⟦#2,#3:|⟧------⟦#2,#3:|⟧--------⟦#2,#3:|⟧
3:⟦#2,#3:|⟧ Parse the query ⟦#2,#3:|⟧ done ⟦#2,#3:|⟧
4:⟦#2,#3:|⟧ Walk the file tree ⟦#2,#3:|⟧ todo ⟦#2,#3:|⟧
```

## E132.3 The boundary: as many characters as the line keeps the markers

Fixture: `auto-highlight`

`⟦#2:x⟧` adds 5 characters: as many as `a x b` has, one more than `ax b` has.

```json request
{"query": "name = 'boundary.txt' and text has 'x'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'boundary.txt'
#2 text has 'x'

./
boundary.txt  [#1 #2]  lines: 2  matches: 2
1:a ⟦#2:x⟧ b
2:ax b
```

## E132.4 The line is counted after the dedent

Fixture: `auto-highlight`

1. `find`: the dedented line `x y` has 3 characters

```json request
{"query": "name = 'dedent.rs' and text has 'x'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'dedent.rs'
#2 text has 'x'

./
dedent.rs  [#1 #2]  lines: 1  matches: 1
⇥4
2:x y
```

2. `find` without the dedent: `    x y` has 7

```json request
{"query": "name = 'dedent.rs' and text has 'x'", "output": {"dedent": false}}
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 name = 'dedent.rs'
#2 text has 'x'

./
dedent.rs  [#1 #2]  lines: 1  matches: 1
2:    ⟦#2:x⟧ y
```

## E132.5 A repeated line follows the rule of its text

Fixture: `auto-highlight`

```json request
{"query": "path ~ 'rep/*' and text has 'x'"}
```

```text response
files: 3  lines: 6  matches: 6  result: r1
#1 path ~ 'rep/*'
#2 text has 'x'

repeated:
  ×3 let ⟦#2:x⟧ = 1;    rep/a.txt:1, rep/b.txt:1, rep/c.txt:1
  ×3 ax    rep/a.txt:2, rep/b.txt:2, rep/c.txt:2
```

## E132.6 A line with a marker written plain has no warning

Fixture: `auto-highlight`

`⟦x⟧` is written plain, `marked.txt` keeps its markers and is listed in `marker_in_content`.

```json request
{"query": "name ~ 'marke*' and text has 'x'"}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 name ~ 'marke*'
#2 text has 'x'

./
marked.txt  [#1 #2]  lines: 1  matches: 1
1:see ⟦ the ⟦#2:x⟧ ⟧ here
marker.txt  [#1 #2]  lines: 1  matches: 1
1:⟦x⟧

warning: marker_in_content (1): marked.txt
```

## E132.7 The lines of a multiline fragment are counted together

Fixture: `auto-highlight`

`a` and `b` have 2 characters, their markers 5; `long` and `line` have 8.

```json request
{"query": "name = 'multi.txt' and text =~ `a\\nb|long\\nline` /m"}
```

```text response
files: 1  lines: 4  matches: 2  result: r1
#1 name = 'multi.txt'
#2 text =~ `a\nb|long\nline` /m

./
multi.txt  [#1 #2]  lines: 4  matches: 2
1:a
2:b
3:⟦#2:long
4:line⟧
```
