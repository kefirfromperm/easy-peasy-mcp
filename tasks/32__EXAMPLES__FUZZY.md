# Examples for task 32

## Fixtures

### fuzzy

```text tree
services/http/handler.go   2026-10-01 10:00
src/config-loader.rs       2026-10-01 10:00
src/config_loader.rs       2026-10-01 10:00
src/ConfigLoader.java      2026-10-01 10:00
src/scaffold_bigolder.ts   2026-10-01 10:00
src/server/handlers.rs     2026-10-01 10:00
```

File contents are in `tests/fixtures/fuzzy/`; every file has one line of code. Times are UTC.

The matched files of an example have the same fuzzy score, so the order doesn't depend on it.

## E32.1 Characters in order, at word starts

`scaffold_bigolder.ts` has the characters in order too, but mostly in the middle of words: its score is below 0.5.

Fixture: `fuzzy`

```json request
{ "query": "name ~~ 'cfgldr' and name ~ '*.{rs,ts}'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'
#2 name ~ '*.{rs,ts}'

src/
config-loader.rs  [#1 #2]
config_loader.rs  [#1 #2]
```

## E32.2 Case changes start words, any case in the value

Fixture: `fuzzy`

```json request
{ "query": "name ~~ 'CfgLdr' and name ~ '*.java'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~~ 'CfgLdr'
#2 name ~ '*.java'

src/
ConfigLoader.java  [#1 #2]
```

## E32.3 All terms must match

Fixture: `fuzzy`

```json request
{ "query": "name ~~ 'cfg java'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfg java'

src/
ConfigLoader.java  [#1]
```

## E32.4 A path, terms in directories and in the file name

Fixture: `fuzzy`

```json request
{ "query": "path ~~ 'srv hndl' and name ~ '*.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~~ 'srv hndl'
#2 name ~ '*.rs'

src/server/
handlers.rs  [#1 #2]
```

## E32.5 Another path of the spec example

Fixture: `fuzzy`

```json request
{ "query": "path ~~ 'srv hndl' and name ~ '*.go'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~~ 'srv hndl'
#2 name ~ '*.go'

services/http/
handler.go  [#1 #2]
```

## E32.6 `~~` on `text`

Fixture: `fuzzy`

```json request
{ "query": "text ~~ 'cfg'" }
```

```text error
error[operator_not_allowed]: operator '~~' is not allowed for 'text'
  text ~~ 'cfg'
       ^^
  hint: fuzzy matching works on 'path' and 'name'
```
