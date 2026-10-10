# Examples for task 1

## Fixtures

### mvp

```text tree
.aiignore                  2026-10-01 10:00
.config/x.rs               2026-10-01 10:00
.gitignore                 2026-10-01 10:00
.hidden.rs                 2026-10-01 10:00
.ignore                    2026-10-01 10:00
Cargo.lock                 2026-10-01 10:00
Cargo.toml                 2026-10-01 10:00
README.md                  2026-10-01 10:00
api/handlers.rs            2026-10-01 10:00
api/v1/routes.py           2026-10-01 10:00
api/v2/routes.py           2026-10-01 10:00
app.log                    2026-10-01 10:00
data/v1.json               2026-10-01 10:00
data/v10.json              2026-10-01 10:00
data/v2.json               2026-10-01 10:00
data/v3.json               2026-10-01 10:00
docs/a.md                  2026-10-01 10:00
docs/b.md                  2026-10-01 10:00
keep.log                   2026-10-01 10:00
notes/[draft].txt          2026-10-01 10:00
notes/it's.txt             2026-10-01 10:00
notes/plain.txt            2026-10-01 10:00
notes/{wip}.txt            2026-10-01 10:00
scripts.rs/notes.txt       2026-10-01 10:00
src/config.rs              2026-10-01 10:00
src/gen.rs                 2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/schema.generated.rs    2026-10-01 10:00
src/secret.rs              2026-10-01 10:00
target/debug/out.rs        2026-10-01 10:00
tmp/x.rs                   2026-10-01 10:00
web/app.js                 2026-10-01 10:00
web/app.ts                 2026-10-01 10:00
web/app.tsx                2026-10-01 10:00
```

File contents are in `tests/fixtures/mvp/`. Times are UTC. What the defaults skip:

- hidden: `.config/`, `.hidden.rs` and the ignore files themselves
- `.gitignore` (`target/`, `*.log`): `target/debug/out.rs`, `app.log`; `keep.log` is re-included by `!keep.log` in
  `.aiignore`, which wins over `.gitignore` at the same level
- `.ignore` (`tmp/`): `tmp/x.rs`
- `.aiignore` (`src/secret.rs`): `src/secret.rs`
- generated: `Cargo.lock` (`**/*.lock`), `src/schema.generated.rs` (`**/*.generated.*`), `src/gen.rs` (the marker
  `@generated` in the first line)
- `scripts.rs/` is a directory, it is not returned by `name ~ '*.rs'`

## E1.1 Exact name

Fixture: `mvp`

```json request
{ "query": "name = 'Cargo.toml'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'Cargo.toml'

./
Cargo.toml  [#1]
```

## E1.2 Wildcard `*`, files in several directories

Hidden, ignored and generated files and the directory `scripts.rs/` are not returned.

Fixture: `mvp`

```json request
{ "query": "name ~ '*.rs'" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

api/
handlers.rs  [#1]
src/
config.rs  [#1]
lib.rs  [#1]
main.rs  [#1]
```

## E1.3 Wildcard `?`

Fixture: `mvp`

```json request
{ "query": "name ~ 'v?.json'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~ 'v?.json'

data/
v1.json  [#1]
v2.json  [#1]
v3.json  [#1]
```

## E1.4 Character set `[...]`

Fixture: `mvp`

```json request
{ "query": "name ~ 'v[13].json'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'v[13].json'

data/
v1.json  [#1]
v3.json  [#1]
```

## E1.5 Character range `[a-z]`

Fixture: `mvp`

```json request
{ "query": "name ~ 'v[1-2].json'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'v[1-2].json'

data/
v1.json  [#1]
v2.json  [#1]
```

## E1.6 Negated set `[!...]`

Fixture: `mvp`

```json request
{ "query": "name ~ 'v[!1].json'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'v[!1].json'

data/
v2.json  [#1]
v3.json  [#1]
```

## E1.7 Alternatives `{a,b}`

Fixture: `mvp`

```json request
{ "query": "name ~ 'app.{ts,tsx}'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'app.{ts,tsx}'

web/
app.ts  [#1]
app.tsx  [#1]
```

## E1.8 Bracket escape `[[]`

Fixture: `mvp`

```json request
{ "query": "name ~ '[[]*'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '[[]*'

notes/
[draft].txt  [#1]
```

## E1.9 Bracket escape `[{]`

Fixture: `mvp`

```json request
{ "query": "name ~ '[{]*}.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '[{]*}.txt'

notes/
{wip}.txt  [#1]
```

## E1.10 Bracket escapes `[*]` and `[?]` match only the character itself

`docs/b.md` would match if `[*]` or `[?]` were wildcards.

Fixture: `mvp`

```json request
{ "query": "name ~ '{[*],[?],a}.md'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '{[*],[?],a}.md'

docs/
a.md  [#1]
```

## E1.11 Double quoted string with `'`

Fixture: `mvp`

```json request
{ "query": "name = \"it's.txt\"" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = "it's.txt"

notes/
it's.txt  [#1]
```

## E1.12 Escaped quote `\'`

Fixture: `mvp`

```json request
{ "query": "name = 'it\\'s.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'it\'s.txt'

notes/
it's.txt  [#1]
```

## E1.13 Raw string

Fixture: `mvp`

```json request
{ "query": "name = `it's.txt`" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = `it's.txt`

notes/
it's.txt  [#1]
```

## E1.14 All escapes of a quoted string

The alternatives `"q"`, `'q'`, `\x`, a line break and a tab match no file; the legend shows the condition as written.

Fixture: `mvp`

```json request
{ "query": "name ~ \"{\\\"q\\\",\\'q\\',\\\\x,\\n,\\t,Cargo.toml}\"" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ "{\"q\",\'q\',\\x,\n,\t,Cargo.toml}"

./
Cargo.toml  [#1]
```

## E1.15 Keywords in upper case

Fixture: `mvp`

```json request
{ "query": "NAME = 'lib.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 NAME = 'lib.rs'

src/
lib.rs  [#1]
```

## E1.16 Line breaks between tokens

Fixture: `mvp`

```json request
{ "query": "name\n  ~\n  'v1*.json'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'v1*.json'

data/
v1.json  [#1]
v10.json  [#1]
```

## E1.17 Nested directories

Fixture: `mvp`

```json request
{ "query": "name = 'routes.py'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'routes.py'

api/v1/
routes.py  [#1]
api/v2/
routes.py  [#1]
```

## E1.18 A later ignore file re-includes a file

Fixture: `mvp`

```json request
{ "query": "name ~ '*.log'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
keep.log  [#1]
```

## E1.19 Unexpected token

Fixture: `mvp`

```json request
{ "query": "name ~ '*.rs' 'x'" }
```

```text error
error[unexpected_token]: unexpected 'x'
  name ~ '*.rs' 'x'
                ^^^
  hint: missing 'and' / 'or' between conditions?
```

## E1.20 Unexpected end

Fixture: `mvp`

```json request
{ "query": "name ~" }
```

```text error
error[unexpected_end]: unexpected end of the query
  name ~
        ^
  hint: a string is expected after '~'
```

## E1.21 Unknown target

Fixture: `mvp`

```json request
{ "query": "filename = 'a.rs'" }
```

```text error
error[unknown_target]: unknown target 'filename'
  filename = 'a.rs'
  ^^^^^^^^
  hint: did you mean 'name'? expected one of: path, name, type, size, modified, created, accessed, empty, lang, kind, git, text, len
```

## E1.22 Unknown operator

Fixture: `mvp`

```json request
{ "query": "name contains 'a'" }
```

```text error
error[unknown_operator]: unknown operator 'contains'
  name contains 'a'
       ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

## E1.23 Unterminated string

Fixture: `mvp`

```json request
{ "query": "name = 'abc" }
```

```text error
error[unterminated_string]: unterminated string
  name = 'abc
         ^^^^
  hint: closing ' is missing
```

## E1.24 Invalid wildcard

Fixture: `mvp`

```json request
{ "query": "name ~ '[a-'" }
```

```text error
error[invalid_wildcard]: invalid wildcard '[a-'
  name ~ '[a-'
         ^^^^^
  hint: unclosed '['
```

## E1.25 Unknown request field

Fixture: `mvp`

```json request
{ "query": "name = 'Cargo.toml'", "colour": true }
```

```text error
error[invalid_option]: unknown field 'colour'
  hint: expected one of: version, dirs, query, queries, within, since, timeout, traversal, output, escape, query_options, kinds, git, languages
```
