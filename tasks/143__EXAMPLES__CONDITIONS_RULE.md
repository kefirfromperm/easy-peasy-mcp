# Examples for task 143

The rule of spec 2 for the legend, the conditions of the files and the ids of the matches counts the conditions the
agent wrote (OUTPUT.md#counted-conditions): a group counts what is inside it, a condition under `not` counts nothing,
a label adds nothing. The examples are written for the current spec version 2.

## Fixtures

The examples use the fixture `omit` of task 134: `src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs` with
`unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`.

## E143.1 A line group around one condition

Fixture: `omit`

```json request
{"query": "line(text has 'unwrap(')"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
⇥4
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warnings left out: binary_skipped (1)
```

## E143.2 Near counts both sides

Fixture: `omit`

```json request
{"query": "near(text has 'unwrap(', text has 'TODO', 1)"}
```
```text response
files: 1  lines: 2  matches: 2  result: r1
#1 near(text has 'unwrap(', text has 'TODO', 1)
#2 text has 'unwrap('
#3 text has 'TODO'

src/
main.rs  [#1]  lines: 2  matches: 2
⇥4
2:let cfg = load().⟦#1,#2:unwrap(⟧);
3:// ⟦#1,#3:TODO⟧ handle the error

warnings left out: binary_skipped (1)
```

## E143.3 A label counts nothing

Fixture: `omit`

1. a bracket group with a label around one condition

```json request
{"query": "(text has 'unwrap(') @u"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
⇥4
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warnings left out: binary_skipped (1)
```

2. one condition with a label

```json request
{"query": "text has 'unwrap(' @u", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
```

3. a bracket group with a label around two conditions

```json request
{"query": "(text has 'unwrap(' or text has 'TODO') @u"}
```
```text response
files: 2  lines: 4  matches: 4  result: r3
@u (text has 'unwrap(' or text has 'TODO')
#1 text has 'unwrap('
#2 text has 'TODO'

src/
main.rs  [@u #1 #2]  lines: 2  matches: 2
⇥4
2:let cfg = load().⟦@u,#1:unwrap(⟧);
3:// ⟦@u,#2:TODO⟧ handle the error
lib.rs  [@u #1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦@u,#1:unwrap(⟧);
⇥4
4:parse(&text).⟦@u,#1:unwrap(⟧)

warnings left out: binary_skipped (1)
```

## E143.4 A condition under `not` counts nothing

Fixture: `omit`

```json request
{"query": "text has 'unwrap(' and not text has 'TODO'"}
```
```text response
files: 1  lines: 2  matches: 2  result: r1

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
⇥4
4:parse(&text).⟦unwrap(⟧)

warnings left out: binary_skipped (1)
```

## E143.5 A file condition and a line group

Fixture: `omit`

`name ~ '*.rs'` leaves `data.bin` out before it is read, so there is no warning.

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(')"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap(')
#3 text has 'unwrap('

src/
lib.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#2,#3:unwrap(⟧);
⇥4
4:parse(&text).⟦#2,#3:unwrap(⟧)
main.rs  [#1 #2]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#2,#3:unwrap(⟧);
```

## E143.6 JSON

Fixture: `omit`

1. a line group around one condition

```json request
{"query": "line(text has 'unwrap(')", "output": {"format": "json", "detail": "files"}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 2},
    {"path": "src/main.rs", "lines": 1, "matches": 1}
  ],
  "left_out": {"warnings": {"binary_skipped": 1}}
}
```

2. one condition with a label

```json request
{"query": "text has 'unwrap(' @u", "output": {"format": "json", "detail": "files"}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r2",
  "counts": {"files": 2, "lines": 3, "matches": 3},
  "files": [
    {"path": "src/lib.rs", "lines": 2, "matches": 2},
    {"path": "src/main.rs", "lines": 1, "matches": 1}
  ],
  "left_out": {"warnings": {"binary_skipped": 1}}
}
```

## E143.7 The legend back with `show`

Fixture: `omit`

```json request
{"query": "line(text has 'unwrap(')", "output": {"show": ["legend"], "detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 line(text has 'unwrap(')
#2 text has 'unwrap('

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
```

## E143.8 Spec 1 is unchanged

Fixture: `omit`

1. a line group around one condition

```json request
{"version": 1, "query": "line(text has 'unwrap(')", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 line(text has 'unwrap(')
#2 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

2. a condition under `not`

```json request
{"version": 1, "query": "text has 'unwrap(' and not text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 2  matches: 2  result: r2
#1 text has 'unwrap('
#2 text has 'TODO'

src/
lib.rs  [#1]  lines: 2  matches: 2

warning: binary_skipped (1): data.bin
```
