# Examples for task 4

Spec: `1`

## Fixtures

### paths

```text tree
docs/guide.md              2026-10-01 10:00
docs/intro.md              2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/parser/ast.rs          2026-10-01 10:00
src/parser/lexer.rs        2026-10-01 10:00
src/parser/sub/tokens.rs   2026-10-01 10:00
tests/basic.rs             2026-10-01 10:00
tests/suite/flow.rs        2026-10-01 10:00
web/tests/ui.rs            2026-10-01 10:00
```

File contents are in `tests/fixtures/paths/`. Times are UTC. Where an example matches files at several depths,
the shallower files also come first in path order, so the order is the same with relevance sorting.

## E4.1 `*` doesn't cross directories

Fixture: `paths`

```json request
{ "query": "path ~ 'src/*.rs'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path ~ 'src/*.rs'

src/
lib.rs  [#1]
main.rs  [#1]
```

## E4.2 `**` crosses directories

`**/` also matches no directory, so the files directly in `src/` match too.

Fixture: `paths`

```json request
{ "query": "path ~ 'src/**/*.rs'" }
```

```text response
files: 5  lines: 0  matches: 0  result: r1
#1 path ~ 'src/**/*.rs'

src/
lib.rs  [#1]
main.rs  [#1]
src/parser/
ast.rs  [#1]
lexer.rs  [#1]
src/parser/sub/
tokens.rs  [#1]
```

## E4.3 `**/tests/**` includes the top level directory

Fixture: `paths`

```json request
{ "query": "path ~ '**/tests/**'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 path ~ '**/tests/**'

tests/
basic.rs  [#1]
tests/suite/
flow.rs  [#1]
web/tests/
ui.rs  [#1]
```

## E4.4 `?` in a path

Fixture: `paths`

```json request
{ "query": "path ~ 'src/?ib.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ 'src/?ib.rs'

src/
lib.rs  [#1]
```

## E4.5 `=` on the whole path

Fixture: `paths`

```json request
{ "query": "path = 'src/parser/lexer.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path = 'src/parser/lexer.rs'

src/parser/
lexer.rs  [#1]
```

## E4.6 `has`

Fixture: `paths`

```json request
{ "query": "path has 'parser/'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 path has 'parser/'

src/parser/
ast.rs  [#1]
lexer.rs  [#1]
src/parser/sub/
tokens.rs  [#1]
```

## E4.7 `starts`

Fixture: `paths`

```json request
{ "query": "path starts 'docs/'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path starts 'docs/'

docs/
guide.md  [#1]
intro.md  [#1]
```

## E4.8 `ends`

Fixture: `paths`

```json request
{ "query": "path ends '/flow.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ends '/flow.rs'

tests/suite/
flow.rs  [#1]
```
