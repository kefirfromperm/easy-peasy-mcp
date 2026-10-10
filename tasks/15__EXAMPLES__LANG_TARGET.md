# Examples for task 15

## Fixtures

### langs

```text tree
build/Dockerfile           2026-10-01 10:00
src/app.js                 2026-10-01 10:00
src/app.ts                 2026-10-01 10:00
src/data.csv               2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/notes.txt              2026-10-01 10:00
src/run                    2026-10-01 10:00
src/tool                   2026-10-01 10:00
src/util.py                2026-10-01 10:00
```

File contents are in `tests/fixtures/langs/`. `src/run` starts with `#!/usr/bin/env python3`, `src/tool` with
`#!/bin/bash`. Times are UTC. The files of every example have the same kind.

## E15.1 By extension

Fixture: `langs`

```json request
{ "query": "lang = rust" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = rust

src/
lib.rs  [#1]
main.rs  [#1]
```

## E15.2 `in ( ... )`

Fixture: `langs`

```json request
{ "query": "lang in (typescript, javascript)" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang in (typescript, javascript)

src/
app.js  [#1]
app.ts  [#1]
```

## E15.3 By the full file name

Fixture: `langs`

```json request
{ "query": "lang = dockerfile" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = dockerfile

build/
Dockerfile  [#1]
```

## E15.4 By extension and by shebang

`src/run` has no extension; its shebang `#!/usr/bin/env python3` names `python3`.

Fixture: `langs`

```json request
{ "query": "lang = python" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = python

src/
run  [#1]
util.py  [#1]
```

## E15.5 A shebang without `env`

Fixture: `langs`

```json request
{ "query": "lang = shell" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = shell

src/
tool  [#1]
```

## E15.6 `unknown`

Fixture: `langs`

```json request
{ "query": "lang = unknown" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = unknown

src/
data.csv  [#1]
notes.txt  [#1]
```

## E15.7 `!=`

Fixture: `langs`

```json request
{ "query": "lang != rust and name starts 'app'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang != rust
#2 name starts 'app'

src/
app.js  [#1 #2]
app.ts  [#1 #2]
```

## E15.8 Values in any case

Fixture: `langs`

```json request
{ "query": "LANG = Rust" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 LANG = Rust

src/
lib.rs  [#1]
main.rs  [#1]
```

## E15.9 Unknown value

Fixture: `langs`

```json request
{ "query": "lang = rusty" }
```

```text error
error[unknown_value]: unknown value 'rusty'
  lang = rusty
         ^^^^^
  hint: did you mean 'rust'? expected one of: c, cpp, csharp, java, kotlin, scala, go, swift, dart, groovy, javascript, typescript, rust, php, css, scss, python, ruby, shell, perl, r, powershell, batch, sql, lua, haskell, clojure, lisp, html, xml, vue, markdown, yaml, toml, ini, properties, dotenv, json, jsonc, dockerfile, makefile, unknown
```
