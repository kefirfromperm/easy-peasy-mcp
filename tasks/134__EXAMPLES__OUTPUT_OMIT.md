# Examples for task 134

Spec: `1`

`output.omit` leaves parts of the response out (OUTPUT.md#omit).

## Fixtures

### omit

```text tree
data.bin           2026-10-01 10:00
src/lib.rs         2026-10-01 10:00
src/main.rs        2026-10-01 10:00
```

File contents are in `tests/fixtures/omit/`: `src/main.rs` a `main` with `load().unwrap()` and a `// TODO` comment,
`src/lib.rs` a `load` with two `unwrap()`; `data.bin` is binary, so a content condition gives the `binary_skipped`
warning.

## E134.1 The bare output against the default one

Fixture: `omit`

`"all"` leaves out every part: the header, the legend, the conditions and the counts of the files, the ids of the
matches, the line numbers, the gaps, the indentation lines and the warnings.

1. the default

```json request
{"query": "text has 'unwrap('"}
```
```text response
files: 2  lines: 3  matches: 3  result: r1
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

2. all

```json request
{"query": "text has 'unwrap('", "output": {"omit": "all"}}
```
```text response
src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
main.rs
let cfg = load().⟦unwrap(⟧);
```

3. all in JSON

```json request
{"query": "text has 'unwrap('", "output": {"omit": "all", "format": "json"}}
```
```json response
{"version":1,"exists":true,"files":[{"path":"src/lib.rs","blocks":[{"lines":[{"text":"let raw = read().unwrap();","fragments":[{"col":[22,29],"text":"unwrap("}]}]},{"lines":[{"text":"parse(&text).unwrap()","fragments":[{"col":[18,25],"text":"unwrap("}]}]}]},{"path":"src/main.rs","blocks":[{"lines":[{"text":"let cfg = load().unwrap();","fragments":[{"col":[22,29],"text":"unwrap("}]}]}]}]}
```

## E134.2 Every part alone

Fixture: `omit`

Each request leaves out one part of the default response of E134.1.

1. `counts`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["counts"]}}
```
```text response
result: r1
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

2. `result`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["result"]}}
```
```text response
files: 2  lines: 3  matches: 3
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

3. `legend`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["legend"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r3

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

4. `conditions`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["conditions"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r4
#1 text has 'unwrap('

src/
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

5. `file_counts`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["file_counts"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r5
#1 text has 'unwrap('

src/
lib.rs  [#1]
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

6. `match_ids`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["match_ids"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r6
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
--
⇥4
4:parse(&text).⟦unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warning: binary_skipped (1): data.bin
```

7. `line_numbers`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["line_numbers"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r7
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
let raw = read().⟦#1:unwrap(⟧);
--
⇥4
parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

8. `gaps`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["gaps"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r8
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

9. `indent`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["indent"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r9
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
2:let raw = read().⟦#1:unwrap(⟧);
--
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
2:let cfg = load().⟦#1:unwrap(⟧);

warning: binary_skipped (1): data.bin
```

10. `warnings`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["warnings"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r10
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);
```

11. `match_ids` with `highlight = columns`

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["match_ids"], "highlight": "columns"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r11
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().unwrap();
  @ 22..29
--
⇥4
4:parse(&text).unwrap()
  @ 18..25
main.rs  [#1]  lines: 1  matches: 1
⇥4
2:let cfg = load().unwrap();
  @ 22..29

warning: binary_skipped (1): data.bin
```

## E134.3 Every part alone in JSON

Fixture: `omit`

`gaps` has nothing to leave out in JSON.

1. `counts`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["counts"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r1","legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

2. `result`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["result"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

3. `legend`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["legend"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r3","counts":{"files":1,"lines":1,"matches":1},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

4. `conditions`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["conditions"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r4","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

5. `file_counts`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["file_counts"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r5","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

6. `match_ids`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["match_ids"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r6","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO"}]}]}]}]}
```

7. `line_numbers`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["line_numbers"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r7","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

8. `indent`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["indent"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r8","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}],"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

9. `warnings`

```json request
{"query": "text has 'TODO'", "output": {"omit": ["warnings"], "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r9","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"text has 'TODO'"},"files":[{"path":"src/main.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":3,"text":"// TODO handle the error","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

## E134.4 The funnel and the hints

Fixture: `omit`

`hints` leaves out the hints, `funnel` the funnel with its hints, as `funnel = never` does.

1. the default

```json request
{"query": "text has 'todo'"}
```
```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  text has 'todo'                        0
hints:
  text has 'todo' /i                     1 file: TODO

warning: binary_skipped (1): data.bin
```

2. `hints`

```json request
{"query": "text has 'todo'", "output": {"omit": ["hints"]}}
```
```text response
files: 0  lines: 0  matches: 0  result: r2
funnel:
  text has 'todo'                        0

warning: binary_skipped (1): data.bin
```

3. `funnel`

```json request
{"query": "text has 'todo'", "output": {"omit": ["funnel"]}}
```
```text response
files: 0  lines: 0  matches: 0  result: r3

warning: binary_skipped (1): data.bin
```

4. `hints` in JSON

```json request
{"query": "text has 'todo'", "output": {"omit": ["hints"], "format": "json"}}
```
```json response
{"version":1,"exists":false,"result":"r4","funnel":[{"step":"text has 'todo'","files":0}],"counts":{"files":0,"lines":0,"matches":0},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}]}
```

5. `funnel` in JSON

```json request
{"query": "text has 'todo'", "output": {"omit": ["funnel"], "format": "json"}}
```
```json response
{"version":1,"exists":false,"result":"r5","counts":{"files":0,"lines":0,"matches":0},"warnings":[{"code":"binary_skipped","count":1,"paths":["data.bin"]}]}
```

## E134.5 Line numbers left out with context lines

Fixture: `omit`

A matched line and a context line look the same.

```json request
{"query": "text has 'unwrap(' or text has 'TODO'", "output": {"context": 2, "omit": ["line_numbers"]}}
```
```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text has 'unwrap('
#2 text has 'TODO'

src/
main.rs  [#1 #2]  lines: 2  matches: 2
fn main() {
    let cfg = load().⟦#1:unwrap(⟧);
    // ⟦#2:TODO⟧ handle the error
    run(cfg);
}
lib.rs  [#1]  lines: 2  matches: 2
pub fn load() -> Config {
    let raw = read().⟦#1:unwrap(⟧);
    let text = decode(&raw);
    parse(&text).⟦#1:unwrap(⟧)
}

warning: binary_skipped (1): data.bin
```

## E134.6 The truncation and the frame are never left out

Fixture: `omit`

`truncated: yes (...)` and the cursor stay with `"all"`, and `exists: yes`; a count without its counts and its
result id is empty.

1. truncated

```json request
{"query": "text has 'unwrap('", "output": {"max_files": 1, "omit": "all"}}
```
```text response
truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMDk1NTFmMjciLCJmIjoxLCJ0IjpbMiwzLDNdfQ

src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
```

2. truncated in JSON

```json request
{"query": "text has 'unwrap('", "output": {"max_files": 1, "omit": "all", "format": "json"}}
```
```json response
{"version":1,"exists":true,"truncated":true,"truncated_by":["max_files"],"next_cursor":"eyJ2IjoxLCJoIjoiNzJhODk0MTQiLCJmIjoxLCJ0IjpbMiwzLDNdfQ","files":[{"path":"src/lib.rs","blocks":[{"lines":[{"text":"let raw = read().unwrap();","fragments":[{"col":[22,29],"text":"unwrap("}]}]},{"lines":[{"text":"parse(&text).unwrap()","fragments":[{"col":[18,25],"text":"unwrap("}]}]}]}]}
```

3. exists

```json request
{"query": "text has 'unwrap('", "output": {"detail": "exists", "omit": "all"}}
```
```text response
exists: yes
```

4. count

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "omit": "all"}}
```
```text response
```

## E134.7 `omit` of the session, of the request and of a query

Fixture: `omit`

The list of a request replaces the one of the session; every query of a batch has its own.

1. the session

```json request update_config
{"set": "output.omit = ['legend', 'conditions']"}
```
```text response
[output]
omit = ['legend', 'conditions']    # was []
```

2. a request

```json request
{"query": "text has 'TODO'"}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
main.rs  lines: 1  matches: 1
⇥4
3:// ⟦#1:TODO⟧ handle the error

warning: binary_skipped (1): data.bin
```

3. a request with its own list

```json request
{"query": "text has 'TODO'", "output": {"omit": []}}
```
```text response
files: 1  lines: 1  matches: 1  result: r2
#1 text has 'TODO'

src/
main.rs  [#1]  lines: 1  matches: 1
⇥4
3:// ⟦#1:TODO⟧ handle the error

warning: binary_skipped (1): data.bin
```

4. a batch

```json request
{"queries": [{"label": "bare", "query": "text has 'TODO'", "output": {"omit": "all"}}, {"label": "session", "query": "text has 'TODO'"}]}
```
```text response
== bare ==  src/
main.rs
// ⟦TODO⟧ handle the error
== session ==  files: 1  lines: 1  matches: 1  result: r4

src/
main.rs  lines: 1  matches: 1
⇥4
3:// ⟦#1:TODO⟧ handle the error

warning: binary_skipped (1): data.bin
```

## E134.8 More fits into the budget

Fixture: `omit`

The default response with two context lines doesn't fit into 100 tokens, the budget leaves the context out; the bare
one fits with its context.

1. the default

```json request
{"query": "text has 'unwrap(' or text has 'TODO'", "output": {"context": 2, "budget": 100}}
```
```text response
files: 2  lines: 4  matches: 4  result: r1  truncated: yes (budget)
#1 text has 'unwrap('
#2 text has 'TODO'

src/
main.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let cfg = load().⟦#1:unwrap(⟧);
3:// ⟦#2:TODO⟧ handle the error
lib.rs  [#1]  lines: 2  matches: 2
⇥4
2:let raw = read().⟦#1:unwrap(⟧);
--
⇥4
4:parse(&text).⟦#1:unwrap(⟧)

warning: binary_skipped (1): data.bin
```

2. all

```json request
{"query": "text has 'unwrap(' or text has 'TODO'", "output": {"context": 2, "budget": 100, "omit": "all"}}
```
```text response
src/
main.rs
fn main() {
    let cfg = load().⟦unwrap(⟧);
    // ⟦TODO⟧ handle the error
    run(cfg);
}
lib.rs
pub fn load() -> Config {
    let raw = read().⟦unwrap(⟧);
    let text = decode(&raw);
    parse(&text).⟦unwrap(⟧)
}
```

## E134.9 Wrong values

Fixture: `omit`

1. an unknown part

```json request
{"query": "text has 'unwrap('", "output": {"omit": ["legend", "colour"]}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings, left_out)
```

2. another string

```json request
{"query": "text has 'unwrap('", "output": {"omit": "none"}}
```
```text error
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings, left_out)
```

3. in update_config

```json request update_config
{"set": "output.omit = 'none'"}
```
```text error
error[invalid_value]: invalid value for 'output.omit'
  hint: "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings, left_out) is expected
```
