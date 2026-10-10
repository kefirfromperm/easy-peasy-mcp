# Examples for task 21

## Fixtures

### scripts

```text tree
bin/VERSION                2026-10-01 10:00
bin/build                  2026-10-01 10:00
bin/clean                  2026-10-01 10:00
bin/deploy                 2026-10-01 10:00
```

File contents are in `tests/fixtures/scripts/`; `bin/build` has no line break at its end. Times are UTC.

## E21.1 `starts`: a shebang

Fixture: `scripts`

```json request
{ "query": "text starts '#!/bin/bash'" }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text starts '#!/bin/bash'

bin/
build  [#1]  lines: 1  matches: 1
1:⟦#1:#!/bin/bash⟧
deploy  [#1]  lines: 1  matches: 1
1:⟦#1:#!/bin/bash⟧
```

## E21.2 `ends` with a line break

The fragment ends with the line break of line 2, so it closes at the end of that line.

Fixture: `scripts`

```json request
{ "query": "text ends 'clean\\n'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text ends 'clean\n'

bin/
clean  [#1]  lines: 1  matches: 1
2:echo ⟦#1:clean⟧
```

## E21.3 Files without a line break at the end

Fixture: `scripts`

```json request
{ "query": "not text ends '\\n'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 text ends '\n'

bin/
build
```

## E21.4 `=`: the whole content

Fixture: `scripts`

```json request
{ "query": "text = '1.0.0\\n'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text = '1.0.0\n'

bin/
VERSION  [#1]  lines: 1  matches: 1
1:⟦#1:1.0.0⟧
```

## E21.5 `starts` with `/i`

Fixture: `scripts`

```json request
{ "query": "text starts '#!/BIN/SH' /i" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text starts '#!/BIN/SH' /i

bin/
clean  [#1]  lines: 1  matches: 1
1:⟦#1:#!/bin/sh⟧
```

## E21.6 Wildcards don't work on `text`

Fixture: `scripts`

```json request
{ "query": "text ~ '*foo*'" }
```

```text error
error[operator_not_allowed]: operator '~' is not allowed for 'text'
  text ~ '*foo*'
       ^
  hint: wildcards work on 'path' and 'name'; use 'has'
```
