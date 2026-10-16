# Examples for task 146

The rule of spec 2 writes the conditions of the files, the ids of the matches and the legend only where they tell
something apart (OUTPUT.md#the-same-everywhere), decided over the whole result. The examples are written for the
current spec version 2.

## Fixtures

The examples use the fixture `omit` of task 134: `src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs` with
`unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`.

## E146.1 A file condition and a text condition

Fixture: `omit`

Every file has `[#1 #2]`, every match `#2`: no legend, no ids. `name ~ '*.rs'` leaves `data.bin` out before it is
read, so there is no warning.

1. text

```json request
{"query": "name ~ '*.rs' and text has 'unwrap('"}
```
```text response
files: 2  lines: 3  result: r1

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);
```

2. JSON

```json request
{"query": "name ~ '*.rs' and text has 'unwrap('", "output": {"format": "json"}}
```
```json response
{"version":2,"exists":true,"result":"r2","counts":{"files":2,"lines":3,"matches":3},"files":[{"path":"src/lib.rs","lines":2,"matches":2,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let raw = read().unwrap();","fragments":[{"col":[22,29],"text":"unwrap("}]}]},{"indent":4,"lines":[{"n":4,"text":"parse(&text).unwrap()","fragments":[{"col":[18,25],"text":"unwrap("}]}]}]},{"path":"src/main.rs","lines":1,"matches":1,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let cfg = load().unwrap();","fragments":[{"col":[22,29],"text":"unwrap("}]}]}]}]}
```

3. a line group: E143.5 without the legend and the ids

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(')"}
```
```text response
files: 2  lines: 3  result: r3

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);
```

## E146.2 The files differ, the matches don't

Fixture: `omit`

`lib.rs` matches by its name, `main.rs` by its text: the conditions of the files and the legend are written, the
only match has `#2` alone, so its ids are not.

```json request
{"query": "name = 'lib.rs' or text has 'TODO'"}
```
```text response
files: 2  lines: 1  result: r1
#1 name = 'lib.rs'
#2 text has 'TODO'

src/
main.rs  [#2]  lines: 1
⇥4
3:// ⟦TODO⟧ handle the error
lib.rs  [#1]

warnings left out: binary_skipped (1)
```

## E146.3 The matches differ, the files don't

Fixture: `omit`

Every file has `[#1]`, the matches `#1,#2` or `#1,#3`.

1. `lines` detail: the legend and the ids of the matches

```json request
{"query": "line(text has 'unwrap(' or text has 'TODO')"}
```
```text response
files: 2  lines: 4  result: r1
#1 line(text has 'unwrap(' or text has 'TODO')
#2 text has 'unwrap('
#3 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦#1,#2:unwrap(⟧);
4:parse(&text).⟦#1,#2:unwrap(⟧)
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#1,#2:unwrap(⟧);
3:// ⟦#1,#3:TODO⟧ handle the error

warnings left out: binary_skipped (1)
```

2. `files` detail shows no match: no legend

```json request
{"query": "line(text has 'unwrap(' or text has 'TODO')", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 4  result: r2

src/
lib.rs  lines: 2
main.rs  lines: 2

warnings left out: binary_skipped (1)
```

## E146.4 A page decides by the whole result

Fixture: `omit`

The first page has only `lib.rs`, whose matches all have `#1,#2`; `main.rs` of the second page has `#1,#3` too, so
both pages write the ids.

1. the first page

```json request
{"query": "line(text has 'unwrap(' or text has 'TODO')", "output": {"max_files": 1, "sort": "path asc"}}
```
```text response
files: 2  lines: 4  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNjUwZGI1NmEiLCJmIjoxLCJ0IjpbMiw0LDRdfQ
#1 line(text has 'unwrap(' or text has 'TODO')
#2 text has 'unwrap('
#3 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦#1,#2:unwrap(⟧);
4:parse(&text).⟦#1,#2:unwrap(⟧)

warnings left out: binary_skipped (1)
```

2. the second page

```json request
{"query": "line(text has 'unwrap(' or text has 'TODO')", "output": {"max_files": 1, "sort": "path asc", "cursor": "eyJ2IjoxLCJoIjoiNjUwZGI1NmEiLCJmIjoxLCJ0IjpbMiw0LDRdfQ"}}
```
```text response
files: 2  lines: 4  result: r2
#1 line(text has 'unwrap(' or text has 'TODO')
#2 text has 'unwrap('
#3 text has 'TODO'

src/
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#1,#2:unwrap(⟧);
3:// ⟦#1,#3:TODO⟧ handle the error

warnings left out: binary_skipped (1)
```

## E146.5 The condition facet

Fixture: `omit`

The conditions of the files are the same, so the legend is not written.

```json request
{"query": "name ~ '*.rs' and text has 'unwrap('", "output": {"detail": "facets", "facets": ["condition"]}}
```
```text response
files: 2  lines: 3  result: r1
condition:  #1 2 · #2 2
```

## E146.6 Each part back with `show`

Fixture: `omit`

1. `legend`

```json request
{"query": "name ~ '*.rs' and text has 'unwrap('", "output": {"show": ["legend"], "detail": "files"}}
```
```text response
files: 2  lines: 3  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap('

src/
lib.rs  lines: 2
main.rs  lines: 1
```

2. `conditions`

```json request
{"query": "name ~ '*.rs' and text has 'unwrap('", "output": {"show": ["conditions"], "detail": "files"}}
```
```text response
files: 2  lines: 3  result: r2

src/
lib.rs  [#1 #2]  lines: 2
main.rs  [#1 #2]  lines: 1
```

3. `match_ids`

```json request
{"query": "name ~ '*.rs' and text has 'TODO'", "output": {"show": ["match_ids"]}}
```
```text response
files: 1  lines: 1  result: r3

src/
main.rs  lines: 1
⇥4
3:// ⟦#2:TODO⟧ handle the error
```

## E146.7 Spec 1 is unchanged

Fixture: `omit`

```json request
{"version": 1, "query": "name ~ '*.rs' and text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap('

src/
lib.rs  [#1 #2]  lines: 2  matches: 2
main.rs  [#1 #2]  lines: 1  matches: 1
```
