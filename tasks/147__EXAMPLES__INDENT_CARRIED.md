# Examples for task 147

In spec 2 a `⇥N` holds for the blocks after it until the next one, from 0 at every file (OUTPUT.md#dedent). The
examples are written for the current spec version 2.

## Fixtures

### indent

```text tree
src/one.rs         2026-10-01 10:00
src/two.rs         2026-10-01 10:00
src/mixed.rs       2026-10-01 10:00
src/deep.rs        2026-10-01 10:00
```

File contents are in `tests/fixtures/indent/`: `src/one.rs` `let a` in line 2, `run();` in line 3 and `let b` in
line 4, `src/two.rs` `let c` in line 2 and `run();` in line 3, all indented by 4; `src/mixed.rs` `let` indented by 4
(lines 2 and 6), by 8 (line 4) and not at all (line 8); `src/deep.rs` two functions in a module, `let` in lines 3
and 8, indented by 8, the lines around them by 4 and 8.

## E147.1 The same indentation in one file and in two files

Fixture: `indent`

```json request
{"query": "name ~ '{one,two}.rs' and text has 'let'"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
one.rs  lines: 2  matches: 2
⇥4
2:⟦let⟧ a = 1;
4:⟦let⟧ b = 2;
two.rs  lines: 1  matches: 1
⇥4
2:⟦let⟧ c = 3;
```

## E147.2 Different indentation, and none after some

Fixture: `indent`

```json request
{"query": "name = 'mixed.rs' and text has 'let'"}
```
```text response
files: 1  lines: 4  matches: 4  result: r1

src/
mixed.rs  lines: 4  matches: 4
⇥4
2:⟦let⟧ x = 1;
⇥8
4:⟦let⟧ y = x;
⇥4
6:⟦let⟧ z = x;
⇥0
8:⟦let⟧ w = 0;
```

## E147.3 Context lines

Fixture: `indent`

Both blocks have the indentation 4: one `⇥4`, the gap between them as always.

```json request
{"query": "name = 'deep.rs' and text has 'let'", "output": {"context": 1}}
```
```text response
files: 1  lines: 2  matches: 2  result: r1

src/
deep.rs  lines: 2  matches: 2
⇥4
2-fn f() {
3:    ⟦let⟧ a = 1;
4-    a
--
7-fn g() {
8:    ⟦let⟧ b = 2;
9-    b
```

## E147.4 `omit = ["indent"]`

Fixture: `indent`

```json request
{"query": "name = 'mixed.rs' and text has 'let'", "output": {"omit": ["indent"]}}
```
```text response
files: 1  lines: 4  matches: 4  result: r1

src/
mixed.rs  lines: 4  matches: 4
2:⟦let⟧ x = 1;
4:⟦let⟧ y = x;
6:⟦let⟧ z = x;
8:⟦let⟧ w = 0;
```

## E147.5 A repeated line between blocks

Fixture: `indent`

`run();` of both files is in the repeated section; every file starts with 0 after it.

```json request
{"query": "name ~ '{one,two}.rs' and text =~ 'let|run'", "output": {"group_repeated": 2}}
```
```text response
files: 2  lines: 5  matches: 5  result: r1

repeated:
  ×2 ⟦run⟧();    src/one.rs:3, src/two.rs:3
src/
one.rs  lines: 3  matches: 3
⇥4
2:⟦let⟧ a = 1;
4:⟦let⟧ b = 2;
two.rs  lines: 2  matches: 2
⇥4
2:⟦let⟧ c = 3;
```

## E147.6 A page starts with 0

Fixture: `indent`

1. the first page

```json request
{"query": "name ~ '{one,two}.rs' and text has 'let'", "output": {"max_files": 1, "sort": "path asc"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMWY2Nzk4NDAiLCJmIjoxLCJ0IjpbMiwzLDNdfQ

src/
one.rs  lines: 2  matches: 2
⇥4
2:⟦let⟧ a = 1;
4:⟦let⟧ b = 2;
```

2. the second page

```json request
{"query": "name ~ '{one,two}.rs' and text has 'let'", "output": {"max_files": 1, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiMWY2Nzk4NDAiLCJmIjoxLCJ0IjpbMiwzLDNdfQ"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2

src/
two.rs  lines: 1  matches: 1
⇥4
2:⟦let⟧ c = 3;
```

## E147.7 Spec 1 is unchanged

Fixture: `indent`

```json request
{"version": 1, "query": "name = 'mixed.rs' and text has 'let'"}
```
```text response
files: 1  lines: 4  matches: 4  result: r1
#1 name = 'mixed.rs'
#2 text has 'let'

src/
mixed.rs  [#1 #2]  lines: 4  matches: 4
⇥4
2:⟦#2:let⟧ x = 1;
--
⇥8
4:⟦#2:let⟧ y = x;
--
⇥4
6:⟦#2:let⟧ z = x;
--
8:⟦#2:let⟧ w = 0;
```

## E147.8 Every `⇥N` back with `show`

Fixture: `indent`

```json request
{"query": "name = 'mixed.rs' and text has 'let'", "output": {"show": ["indent"]}}
```
```text response
files: 1  lines: 4  matches: 4  result: r1

src/
mixed.rs  lines: 4  matches: 4
⇥4
2:⟦let⟧ x = 1;
⇥8
4:⟦let⟧ y = x;
⇥4
6:⟦let⟧ z = x;
8:⟦let⟧ w = 0;
```
