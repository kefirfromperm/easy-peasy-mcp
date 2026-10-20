# Examples for task 154

In spec 2 a written legend lists only the conditions whose ids the response writes: the conditions of the files, the
ids of the matches in a highlight which writes them, the `condition` facet, and every labelled condition
(OUTPUT.md#the-legend-of-the-written-ids). A legend left with no entry is not written. The examples are written for
the current spec version 2.

## Fixtures

The examples use the fixture `omit` of task 134: `src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs` with
`unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`. `name ~ '*.rs'` leaves
`data.bin` out before it is read, so there is no warning.

## E154.1 `line(a or b)` with `highlight = "none"`

Fixture: `omit`

The conditions of the files are the same and the lines are written without ids: no id is written, no legend.

1. text

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"highlight": "none"}}
```
```text response
files: 2  lines: 4  result: r1

src/
lib.rs  lines: 2
⇥4
2:let raw = read().unwrap();
4:parse(&text).unwrap()
main.rs  lines: 2
⇥4
2:let cfg = load().unwrap();
3:// TODO handle the error
```

2. JSON: the lines have `"match": true`, no fragments and no ids

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"highlight": "none", "format": "json"}}
```
```json response
{"version":2,"exists":true,"result":"r2","counts":{"files":2,"lines":4,"matches":4},"files":[{"path":"src/lib.rs","lines":2,"matches":2,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let raw = read().unwrap();","match":true}]},{"indent":4,"lines":[{"n":4,"text":"parse(&text).unwrap()","match":true}]}]},{"path":"src/main.rs","lines":2,"matches":2,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let cfg = load().unwrap();","match":true},{"n":3,"text":"// TODO handle the error","match":true}]}]}]}
```

## E154.2 The ids that are written stay

Fixture: `omit`

The matches have `#2,#3` and `#2,#4`: these stay in the legend; `#1 name ~ '*.rs'` only selects the files and is
written nowhere.

1. markers (`auto`)

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')"}
```
```text response
files: 2  lines: 4  result: r1
#2 line(text has 'unwrap(' or text has 'TODO')
#3 text has 'unwrap('
#4 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦#2,#3:unwrap(⟧);
4:parse(&text).⟦#2,#3:unwrap(⟧)
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#2,#3:unwrap(⟧);
3:// ⟦#2,#4:TODO⟧ handle the error
```

2. columns

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"highlight": "columns"}}
```
```text response
files: 2  lines: 4  result: r2
#2 line(text has 'unwrap(' or text has 'TODO')
#3 text has 'unwrap('
#4 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().unwrap();
  @ 22..29 #2,#3
4:parse(&text).unwrap()
  @ 18..25 #2,#3
main.rs  lines: 2
⇥4
2:let cfg = load().unwrap();
  @ 22..29 #2,#3
3:// TODO handle the error
  @ 8..12 #2,#4
```

3. JSON

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"format": "json"}}
```
```json response
{"version":2,"exists":true,"result":"r3","counts":{"files":2,"lines":4,"matches":4},"legend":{"#2":"line(text has 'unwrap(' or text has 'TODO')","#3":"text has 'unwrap('","#4":"text has 'TODO'"},"files":[{"path":"src/lib.rs","lines":2,"matches":2,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let raw = read().unwrap();","fragments":[{"col":[22,29],"text":"unwrap(","ids":["#2","#3"]}]}]},{"indent":4,"lines":[{"n":4,"text":"parse(&text).unwrap()","fragments":[{"col":[18,25],"text":"unwrap(","ids":["#2","#3"]}]}]}]},{"path":"src/main.rs","lines":2,"matches":2,"blocks":[{"indent":4,"lines":[{"n":2,"text":"let cfg = load().unwrap();","fragments":[{"col":[22,29],"text":"unwrap(","ids":["#2","#3"]}]},{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#2","#4"]}]}]}]}]}
```

4. `groups`: the ids of a shown match are its references

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"groups": ["#3", "#4"]}}
```
```text response
files: 2  lines: 4  result: r4
#3 text has 'unwrap('
#4 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦#3:unwrap(⟧);
4:parse(&text).⟦#3:unwrap(⟧)
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#3:unwrap(⟧);
3:// ⟦#4:TODO⟧ handle the error
```

## E154.3 `near(...)`

Fixture: `omit`

```json request
{"query": "name ~ '*.rs' and near(text has 'unwrap(', text has 'TODO', 1)"}
```
```text response
files: 1  lines: 2  result: r1
#2 near(text has 'unwrap(', text has 'TODO', 1)
#3 text has 'unwrap('
#4 text has 'TODO'

src/
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#2,#3:unwrap(⟧);
3:// ⟦#2,#4:TODO⟧ handle the error
```

## E154.4 A bracket group

Fixture: `omit`

1. the files differ: their conditions write `#1` too, so the whole legend stays

```json request
{"query": "name ~ '*.rs' and (text has 'unwrap(' or text has 'TODO')"}
```
```text response
files: 2  lines: 4  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap('
#3 text has 'TODO'

src/
main.rs  [#1 #2 #3]  lines: 2
⇥4
2:let cfg = load().⟦#2:unwrap(⟧);
3:// ⟦#3:TODO⟧ handle the error
lib.rs  [#1 #2]  lines: 2
⇥4
2:let raw = read().⟦#2:unwrap(⟧);
4:parse(&text).⟦#2:unwrap(⟧)
```

2. the `condition` facet writes every id

```json request
{"query": "name ~ '*.rs' and (name = 'lib.rs' or text has 'TODO')", "output": {"detail": "facets", "facets": ["condition"]}}
```
```text response
files: 2  lines: 1  result: r2
#1 name ~ '*.rs'
#2 name = 'lib.rs'
#3 text has 'TODO'
condition:  #1 2 · #2 1 · #3 1
```

3. other facets write no id: no legend

```json request
{"query": "name ~ '*.rs' and (name = 'lib.rs' or text has 'TODO')", "output": {"detail": "facets", "facets": ["ext"]}}
```
```text response
files: 2  lines: 1  result: r3
ext:        rs 2
```

## E154.5 A labelled condition stays

Fixture: `omit`

1. `@rs` is written nowhere and stays, `#1`, `@unwrap` and `#2` are written in the markers

```json request
{"query": "name ~ '*.rs' @rs and line(text has 'unwrap(' @unwrap or text has 'TODO')"}
```
```text response
files: 2  lines: 4  result: r1
@rs name ~ '*.rs'
#1 line(text has 'unwrap(' or text has 'TODO')
@unwrap text has 'unwrap('
#2 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦#1,@unwrap:unwrap(⟧);
4:parse(&text).⟦#1,@unwrap:unwrap(⟧)
main.rs  lines: 2
⇥4
2:let cfg = load().⟦#1,@unwrap:unwrap(⟧);
3:// ⟦#1,#2:TODO⟧ handle the error
```

2. `highlight = "none"`: only the labelled conditions

```json request
{"query": "name ~ '*.rs' @rs and line(text has 'unwrap(' @unwrap or text has 'TODO')", "output": {"highlight": "none"}}
```
```text response
files: 2  lines: 4  result: r2
@rs name ~ '*.rs'
@unwrap text has 'unwrap('

src/
lib.rs  lines: 2
⇥4
2:let raw = read().unwrap();
4:parse(&text).unwrap()
main.rs  lines: 2
⇥4
2:let cfg = load().unwrap();
3:// TODO handle the error
```

## E154.6 Every condition with `show = ["legend"]`

Fixture: `omit`

```json request
{"query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"highlight": "none", "show": ["legend"]}}
```
```text response
files: 2  lines: 4  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' or text has 'TODO')
#3 text has 'unwrap('
#4 text has 'TODO'

src/
lib.rs  lines: 2
⇥4
2:let raw = read().unwrap();
4:parse(&text).unwrap()
main.rs  lines: 2
⇥4
2:let cfg = load().unwrap();
3:// TODO handle the error
```

## E154.7 Spec 1 is unchanged

Fixture: `omit`

```json request
{"version": 1, "query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": {"highlight": "none"}}
```
```text response
files: 2  lines: 4  matches: 4  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' or text has 'TODO')
#3 text has 'unwrap('
#4 text has 'TODO'

src/
lib.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let raw = read().unwrap();
--
⇥4
4:parse(&text).unwrap()
main.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let cfg = load().unwrap();
3:// TODO handle the error
```
