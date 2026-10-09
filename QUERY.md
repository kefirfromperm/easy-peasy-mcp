# Query Language

The query describes which files the `find` command returns. It is a compact text expression: conditions combined with
logical operators.

```
name ~ '*.rs' and modified > -7d and not path ~ '**/tests/**'
and line(text has 'unwrap' /i and not text starts '//')
```

Rust files changed during the last 7 days, outside of `tests` directories, with at least one line which contains
`unwrap` in any case and doesn't start with `//`. The examples assume the default configuration unless they say
otherwise.

- [Basics](#basics)
- [Condition scope](#condition-scope)
- [Targets](#targets)
- [Operators](#operators)
- [Values](#values)
- [Modifiers](#modifiers)
- [Comments](#comments)
- [Escaping](#escaping)
- [Proximity](#proximity)
- [Identifiers and fuzzy names](#identifiers-and-fuzzy-names)
- [Labels and condition ids](#labels-and-condition-ids)
- [Grammar](#grammar)
- [Errors](#errors)
- [Recipes](#recipes)
- [Gotchas](#gotchas)

## Basics

A condition is `<target> <operator> <value>` (only `empty` stands alone), optionally followed by
[modifiers](#modifiers) and a [label](#labels-and-condition-ids):

```
text has 'todo' /i/w @todo
^^^^ ^^^ ^^^^^^ ^^^^ ^^^^^
 |    |    |      |     label
 |    |    |      modifiers: case-insensitive, whole word
 |    |    value
 |    operator: contains
 target: file content
```

Conditions are combined with `not`, `and`, `xor`, `or` and grouped with brackets `( )`.

| Operator | Meaning                                      | Precedence  |
|----------|----------------------------------------------|-------------|
| `not`    | negation                                     | 1 (highest) |
| `and`    | both are true                                | 2           |
| `xor`    | exactly one is true; chained — odd number    | 3           |
| `or`     | at least one is true                         | 4 (lowest)  |

```
name ~ '*.rs' and text has 'tokio' or text has 'async-std'
```

is `(name ~ '*.rs' and text has 'tokio') or text has 'async-std'`. Use brackets when in doubt:

```
name ~ '*.rs' and (text has 'tokio' or text has 'async-std')
```

Keywords (targets, operators, `and`, `line`, ...) and the values of `type`, `lang`, `kind` and `git` are
case-insensitive. Text values are case-sensitive unless `/i` is used (`~~` and `/n` are always case-insensitive):
`TEXT HAS 'TODO'` matches `TODO`, but not `todo`. For `path` and `name` the default depends on the platform, see
[Targets](#targets).

An empty query matches every file which the [traversal options](CONFIG.md#traversal) let through.

### Files and directories

If the query contains no `type` condition, an implicit `type = file` is added, so only files are returned, like
`grep -r` does. To get directories or symlinks, ask for them explicitly:

```
type = dir and name = 'migrations'
```

| Value  | Matches                                                                                  |
|--------|------------------------------------------------------------------------------------------|
| `file` | regular files; with [`follow_symlinks`](CONFIG.md#traversal) also symlinks to files      |
| `dir`  | directories; with `follow_symlinks` also symlinks to directories                         |
| `link` | symlinks (on Windows also junctions), whether `follow_symlinks` is on or not             |

With `follow_symlinks` a symlink to a file is both `file` and `link`, its target is read. Without it a symlink is
only `link` and its target is not read.

Content conditions (`text`, `len`, `line(...)`, `near(...)`) are false for directories: a directory has no content.

### Generated files

Generated files (`kind = generated`: lock files, `*.min.js`, files with an `@generated` or `DO NOT EDIT` marker in
the first lines, see [kinds](CONFIG.md#kinds)) are skipped by default, they waste tokens and must not be edited
anyway. If the query contains a `kind` condition, nothing is skipped implicitly:

```
kind = generated and text has 'version'
```

The default is [`traversal.skip_generated`](CONFIG.md#traversal).

### Files whose content is not searched

The content of some files is not searched: binary files (unless [`traversal.binary`](CONFIG.md#traversal) is
`search`), files bigger than [`traversal.max_file_size`](CONFIG.md#traversal), archives which are not opened and
archive members over the limits. For such a file every content condition (`text`, `line( ... )`, `near( ... )`) is
**unknown**: neither true nor false. The same holds for a date which the file system doesn't provide (`created`,
`accessed`).

Unknown spreads like in SQL:

| Expression          | Result  |
|---------------------|---------|
| `not unknown`       | unknown |
| `true and unknown`  | unknown |
| `false and unknown` | false   |
| `true or unknown`   | true    |
| `false or unknown`  | unknown |
| `xor` with unknown  | unknown |

A file is returned only if the whole query is true. So such a file is never returned because of its content —
neither by `text has 'x'` nor by `not text has 'x'` — but it can be returned by the other conditions:

```
name ~ '*.png' or text has 'logo'        # PNG files by the name, text files by the content
```

For an empty result the [hints](OUTPUT.md#hints) (on by default) show how many files were skipped this way.

## Condition scope

Every condition works either on the whole file (**file scope**) or on one line (**line scope**).

### File scope

The default. Metadata conditions and `text` on the whole content.

```
text has 'serde' and text has 'tokio'
```

Files which contain both `serde` and `tokio`, possibly on different lines.

```
name ~ '*.rs' and not text has 'SPDX-License-Identifier'
```

Rust files without `SPDX-License-Identifier` anywhere in the file (like `grep -L`).

### Line scope

A `line( ... )` group. The expression inside it must be true for **the same line**. The file matches the group if at
least one of its lines matches it.

```
line(text has 'serde' and text has 'tokio')
```

Files with a line which contains both `serde` and `tokio`.

`not` inside the group negates a condition for this line (like `grep -v`):

```
name = 'nginx.conf' and line(not text starts '#' and len > 0)
```

Lines of `nginx.conf` which don't start with `#` and are not empty.

`not` before the group means that no line of the file matches it:

```
name ~ '*.py' and not line(len > 120)
```

Python files where every line fits into 120 characters.

Rules

- only `text` and `len` can be used inside `line( ... )`
- `line( ... )` can't be nested and can't contain `near( ... )`
- several groups in one query are independent, each of them may match a different line:

  ```
  line(text has 'BEGIN') and line(text has 'COMMIT')
  ```

## Targets

| Target     | Scope | Value type | Meaning                                                        |
|------------|-------|------------|----------------------------------------------------------------|
| `path`     | file  | text       | path relative to the working directory, `/` as separator       |
| `name`     | file  | text       | file name with extension                                       |
| `type`     | file  | type       | `file`, `dir`, `link`                                          |
| `size`     | file  | size       | file size in bytes                                             |
| `modified` | file  | date       | last modification date                                         |
| `created`  | file  | date       | creation date (if the file system supports it)                 |
| `accessed` | file  | date       | last access date (if the file system tracks it)                |
| `empty`    | file  | —          | empty file (size 0) or directory without entries               |
| `lang`     | file  | language   | language or format of the file, see below                      |
| `kind`     | file  | kind       | `source`, `test`, `config`, `doc`, `generated`, `other`        |
| `git`      | file  | git status | `modified`, `staged`, `untracked`, `conflicted`, `changed`, `branch`, `clean` |
| `text`     | both  | text       | whole file content in file scope, line content in line scope   |
| `len`      | line  | number     | line length in characters, without the line break              |

Notes

- `path` doesn't include the working directory itself. Searching in `~/project`, the file `~/project/src/main.rs` has
  `path` = `src/main.rs` and `name` = `main.rs`.
- `/` is the path separator in queries on every platform. On Windows `\` is accepted as a separator too. In
  wildcards a special character is escaped with brackets on every platform: `[*]`, `[?]`, `[[]`, `[{]`.
- Case sensitivity of `path` and `name` follows the platform by default: sensitive on Linux, insensitive on macOS and
  Windows, see [`traversal.path_case`](CONFIG.md#traversal). `/i` always makes a condition case-insensitive; `~~` and
  `/n` are always case-insensitive.
- `path` and `name` are compared in Unicode NFC form, so a name stored decomposed (as macOS may do) matches the
  composed form written in the query.
- Inside archives (see [`traversal.archives`](CONFIG.md#archives)) a member's path is
  `<archive path>!/<member path>`, e.g. `logs/2026-10.zip!/app/server.log`, its `name` is `server.log`. A compressed
  single file (`app.log.gz`) keeps its own path and name, only its content is decompressed.
- `text` and `len` work on the decoded text, whatever the file encoding is, see
  [Encoding](CONFIG.md#encoding).
- If the file system doesn't provide `created` or `accessed`, the condition is unknown (see
  [Files whose content is not searched](#files-whose-content-is-not-searched)) and the output has a
  `date_unsupported` warning.
- Lines are split by `\n`, a trailing `\r` is removed, so files with `\r\n` line breaks work the same way.
- `len` counts Unicode characters, a tab is one character.

### lang

The language of a file, by the [language table](CONFIG.md#languages): by the extension, by the full file name
(`Dockerfile`, `Makefile`), and for files without an extension by the shebang (`#!/usr/bin/env python3` → `python`).
Unknown files have `lang = unknown`.

```
lang = rust
lang in (typescript, javascript)
lang != markdown
```

`lang = rust` is shorter than `name ~ '*.rs'` and also covers languages with many extensions: `lang = cpp` is `.cpp`,
`.cc`, `.cxx`, `.hpp`, `.hh`, ...

### kind

What the file is for. Every file has exactly one kind, the first which matches in this order:

| Kind        | Detected by                                                                          |
|-------------|--------------------------------------------------------------------------------------|
| `generated` | a marker in the first 5 lines (`@generated`, `DO NOT EDIT`, `Code generated by`, `auto-generated`), lock files, `*.min.js`, `*.min.css`, `*.pb.go`, `*_pb2.py`, `*.g.dart`, ... |
| `test`      | test directories and names: `**/test/**`, `**/tests/**`, `**/__tests__/**`, `*_test.go`, `test_*.py`, `*_test.py`, `*.test.ts`, `*.spec.ts`, `*Test.java`, ... |
| `doc`       | `*.md`, `*.rst`, `*.adoc`, `README*`, `LICENSE*`, `CHANGELOG*`, files in `docs` directories, ... |
| `config`    | `*.toml`, `*.yaml`, `*.yml`, `*.ini`, `*.cfg`, `*.conf`, `*.properties`, `*.env`, `package.json`, `tsconfig*.json`, `*.config.js`, `Dockerfile`, `Makefile`, `requirements*.txt`, CI files, ... |
| `source`    | code: a [language](CONFIG.md#languages) with `source = true`; data and document formats like JSON, XML, YAML and Markdown are not source |
| `other`     | everything else: data, images, binaries                                              |

The table summarizes the default rules: wildcards and markers in the [configuration](CONFIG.md#kinds), which can be
changed. There every wildcard starts with `**/` (`*_test.go` is matched by `**/*_test.*`), so the kind of a file
doesn't depend on the working directory.

```
kind = source and text has 'TODO'          # TODOs in the code, not in docs and tests
kind = test and text has 'sleep('          # tests which mention sleep(
kind in (source, test) and lang = python
```

### git

The git status of a file in its repository. The repository is found for every file separately, so several working
directories, nested repositories and monorepos work.

| Value        | The file                                                                       |
|--------------|--------------------------------------------------------------------------------|
| `modified`   | has changes in the working tree which are not staged                           |
| `staged`     | has staged changes                                                             |
| `untracked`  | is not tracked                                                                 |
| `conflicted` | has merge conflicts                                                            |
| `changed`    | any of the above                                                               |
| `branch`     | `changed`, or changed by the commits of the current branch since the [base](CONFIG.md#git) |
| `clean`      | is tracked and has no changes                                                  |

A file can have several statuses at once (`modified` and `staged`); `git = x` is true if the file has the status `x`.
Outside of a repository and inside archives every `git` condition is false. For files outside of a repository the
response has a `not_a_git_repo` warning.

```
git = changed and text has 'dbg!('              # debug leftovers in files with uncommitted changes
git = branch and kind = source                  # the code changed in this branch
git = conflicted                                # files with conflicts
```

## Operators

### Text operators

For `path`, `name`, `text`; `~` and `~~` only for `path` and `name`.

| Operator | Meaning      | Example                          |
|----------|--------------|----------------------------------|
| `=`      | exact match  | `name = 'Cargo.toml'`            |
| `has`    | contains     | `text has 'TODO'`                |
| `starts` | starts with  | `path starts 'src/'`             |
| `ends`   | ends with    | `name ends '.test.ts'`           |
| `~`      | wildcard     | `name ~ '*.{ts,tsx}'`            |
| `~~`     | fuzzy        | `name ~~ 'cfgldr'`               |
| `=~`     | regex        | ``text =~ `fn \w+_test\(` ``     |

For `text` in file scope `=`, `has`, `starts` and `ends` work on the whole file content, so a value with `'\n'` can
match across lines: `text starts '#!/bin/bash'` is a file whose first line starts with the shebang.

#### Wildcards

Only for `path` and `name`. The pattern must match the whole value.

| Pattern  | Matches                                            |
|----------|----------------------------------------------------|
| `*`      | any characters except `/`                          |
| `**`     | any characters including `/`, i.e. any directories |
| `?`      | one character except `/`                           |
| `[abc]`  | one of the characters, `[a-z]` — range             |
| `[!abc]` | any character except these                         |
| `{a,b}`  | one of the alternatives                            |

```
name ~ '*.rs'                  # any Rust file
name ~ '*.{yml,yaml}'          # YAML files
name ~ '[Dd]ockerfile*'        # Dockerfile, dockerfile.dev, ...
path ~ 'src/*.rs'              # Rust files directly in src
path ~ 'src/**/*.rs'           # Rust files in src at any depth
path ~ '**/tests/**'           # anything inside any tests directory, including the top level one
```

#### Regex

Rust [`regex`](https://docs.rs/regex) crate syntax. It guarantees linear time, so there are **no look-arounds and no
backreferences**.

- In line scope and in file scope without `/m` the regex is applied to every line separately, `^` and `$` are the line
  start and end.
- With `/m` (file scope only) the regex is applied to the whole content and can match across lines: `\n` matches a line
  break, `^` and `$` match at line starts and ends, `.` doesn't match `\n` unless `(?s)` is used.

Write regexes in raw strings `` `...` ``, so backslashes don't have to be doubled.

### Comparison operators

For `size`, `modified`, `created`, `accessed`, `len`.

| Operator          | Meaning                               | Example                       |
|-------------------|---------------------------------------|-------------------------------|
| `=` `!=`          | equal, not equal                      | `size = 0`                    |
| `<` `<=` `>` `>=` | less, less or equal, greater, ...     | `size > 1M`                   |
| `in a..b`         | between, both ends included           | `modified in -7d..-1d`        |

### Value operators

For `type`, `lang`, `kind`, `git`: `=`, `!=` and `in ( ... )` with a list of values.

```
type = dir
lang != markdown
kind in (source, test)
git in (modified, untracked)
```

## Values

### Strings

| Form      | Escapes                                       | Use for                                       |
|-----------|-----------------------------------------------|-----------------------------------------------|
| `'...'`   | `\'`, `\"`, `\\`, `\n`, `\t`                  | most values                                   |
| `"..."`   | `\'`, `\"`, `\\`, `\n`, `\t`                  | values with `'`                               |
| `` `...` `` | none, everything until the closing backtick | regexes, values with quotes and backslashes   |

The query is sent inside a JSON tool argument, where every `"` becomes `\"` and every `\` becomes `\\`. To keep it
readable prefer `'...'` and `` `...` ``:

```json
{ "query": "text =~ `\\bfoo\\(` and name ~ '*.rs'" }
```

is the query ``text =~ `\bfoo\(` and name ~ '*.rs'``. With a quoted string it would need four backslashes in JSON.

### Sizes

A number of bytes with an optional unit: `B`, `K`, `M`, `G` (powers of 1024), e.g. `0`, `512`, `10K`, `1.5M`, `2G`.

### Dates

Absolute, ISO 8601, server local time unless an offset is given

```
2026-10-01
2026-10-01T14:30
2026-10-01T14:30:00Z
2026-10-01T14:30:00+02:00
```

Relative to the moment the search starts: `-` + number + unit, units `s`, `m` (minutes), `h`, `d`, `w`

```
-30m   -2h   -7d   -2w
```

A date without a time means the whole day:

| Condition                    | Means                                          |
|------------------------------|------------------------------------------------|
| `modified = 2026-10-01`      | during October 1                               |
| `modified > 2026-10-01`      | after October 1 (from October 2, 00:00)        |
| `modified >= 2026-10-01`     | from October 1, 00:00                          |
| `modified < 2026-10-01`      | before October 1                               |
| `modified > -7d`             | during the last 7 days                         |
| `modified in -7d..-1d`       | from 7 days ago to 1 day ago                   |

### Numbers

`len` takes a non-negative integer: `len > 120`, `len in 1..10`.

## Modifiers

Written after a text value, can be combined in any order: `/i/w`, `/w /i`.

| Modifier            | Meaning                                       | Allowed with                                  |
|---------------------|-----------------------------------------------|-----------------------------------------------|
| `/i`                | case-insensitive (Unicode simple case folding) | all text operators                           |
| `/w`                | whole word: no letter, digit or `_` right before and after the match | `=`, `has`, `starts`, `ends`, `=~` on `text` |
| `/m`                | multiline regex                               | `=~` on `text` in file scope                  |
| `/c`                | ignore matches inside comments                | all operators on `text`                       |
| `/e(style, levels)` | match escaped text                            | `=`, `has`, `starts`, `ends` on `text`        |
| `/n`                | identifier: any naming style, see [Identifiers](#identifiers-and-fuzzy-names) | `=`, `has`, `starts`, `ends` |

```
text has 'error' /i            # error, Error, ERROR
text has 'id' /w               # id, but not valid or identity
text has 'unsafe' /c           # unsafe in code, not in comments
name = 'readme.md' /i          # README.md, Readme.md, ...
text has 'parse query' /n      # parseQuery, parse_query, ParseQuery, PARSE_QUERY, ...
```

## Comments

`/c` ignores matches which are inside comments. Comment syntax is taken from the [language](CONFIG.md#languages) of
the file, see [comment syntax](CONFIG.md#comment-syntax). Files of an unknown language are searched as is.

```
// let x = foo.unwrap();     <- ignored with /c
let y = bar.unwrap(); // ok  <- matches, the match is before the comment
/* .unwrap() */              <- ignored with /c
let s = "// .unwrap()";      <- matches, the comment marker is inside a string literal
```

With `not` the meaning is "no match outside of comments":

```
name ~ '*.rs' and not text has 'unsafe' /c
```

Rust files where `unsafe` appears only in comments, if at all.

## Escaping

`/e(style, levels)` finds text which is stored escaped in the file. The agent writes the value as it is (unescaped),
the server builds the escaped variants and matches all of them.

Typical case: JSON inside a string in JSON. The object `{"type": "admin"}` stored as a string value looks like
`"{\"type\": \"admin\"}"`, and one more level deep like `"{\\\"type\\\": \\\"admin\\\"}"`.

```
text has `"type": "admin"` /e(backslash, 0..2)
```

matches all of these:

```
{"type": "admin"}
"{\"type\": \"admin\"}"
"{\\\"type\\\": \\\"admin\\\"}"
```

### Styles

| Style            | Formats                          | Escapes                                                         |
|------------------|----------------------------------|-----------------------------------------------------------------|
| `backslash`      | JSON, C, Java, JS, Rust, Python  | `\` → `\\`, `"` → `\"`, `'` → `\'`, new line → `\n`, tab → `\t` |
| `quote_doubling` | SQL, CSV                         | `'` → `''`, `"` → `""`                                          |
| `xml`            | XML, HTML                        | `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`, `"` → `&quot;`, `'` → `&apos;` or `&#39;` |
| `url`            | URLs, query strings              | reserved characters → `%XX`, space → `%20` or `+`               |

`backslash`: JSON doesn't escape `'`, so both `'` and `\'` are accepted for it.

### Levels

Level `n` means the escaping is applied `n` times. For `"`:

| Level | `backslash` | `quote_doubling` | `xml`        | `url`   |
|-------|-------------|------------------|--------------|---------|
| 0     | `"`         | `"`              | `"`          | `"`     |
| 1     | `\"`        | `""`             | `&quot;`     | `%22`   |
| 2     | `\\\"`      | `""""`           | `&amp;quot;` | `%2522` |

- a number: `/e(backslash, 1)` — only escaped once
- a range: `/e(backslash, 0..2)` — raw, once and twice
- `any`: `/e(backslash, any)` — from 0 to [`escape.max_level`](CONFIG.md#escape) (default 3)

The output says on which level each fragment was found, see [OUTPUT.md](OUTPUT.md#fragments).

### Examples

```
name ~ '*.json' and text has `"type": "admin"` /e(backslash, any)
```

`"type": "admin"` in JSON files, also inside stringified JSON nested up to `escape.max_level` levels deep.

```
name ~ '*.sql' and text has `O'Brien` /e(quote_doubling, 1)
```

The name inside SQL string literals: `'O''Brien'`.

```
name ~ '*.html' and text has '<script>' /e(xml, 1)
```

An escaped tag in HTML: `&lt;script&gt;`.

```
text has 'q=hello world' /e(url, 1)
```

`q%3Dhello%20world` or `q%3Dhello+world`.

Escaping combines with `/i`, `/w` and `/c`: `/i` is applied to the escaped variants, `/w` checks the boundaries
around the escaped match. It can't be combined with `/n`.

## Proximity

```
near(<A>, <B>, N [, order] [, report])
```

A file scope condition: true if a line matching `A` and a line matching `B` are at most `N` lines apart. `A` and `B`
are line scope expressions, like the inside of `line( ... )`. `N = 0` means the same line.

Options, keywords in any order after `N`:

| Option     | Values                  | Default | Meaning                                                              |
|------------|-------------------------|---------|----------------------------------------------------------------------|
| order      | `any`, `ordered`        | `any`   | `ordered`: the `B` line must be after the `A` line; on the same line the `B` match must start after the `A` match |
| report     | `all`, `closest`, `first` | `all` | which lines are reported in the output, see below                    |

`report` doesn't change whether the file matches, only which lines and fragments are returned and counted:

- `all` — every `A` line which has a `B` line within `N`, and every such `B` line
- `closest` — for every `A` line only the nearest `B` line; on a tie the earlier one
- `first` — only the first pair in the file; the fastest, enough to know that the file matches

```
near(text has 'open(', text has 'close(', 20, ordered, closest)
```

Every `open(` with the nearest `close(` after it within 20 lines.

```
near(text has '.lock()', text has '.await', 3)
```

A lock and an `await` close to each other — a possible lock held across an await point.

```
near(text =~ `^\s*#\[test\]`, text has '#[ignore]', 1)
```

Ignored tests.

```
name ~ '*.go' and near(text has 'err != nil', text has 'return nil', 2)
```

An error check which returns `nil` — a possibly swallowed error.

## Identifiers and fuzzy names

An agent often knows the concept, but not the exact spelling of a name. Two tools for that.

### `/n` — any naming style

The value is split into words and matches the words joined in any common style, case-insensitive.

```
text has 'parse query' /n
```

matches

| Style            | Example        |
|------------------|----------------|
| camelCase        | `parseQuery`   |
| PascalCase       | `ParseQuery`   |
| snake_case       | `parse_query`  |
| SCREAMING_SNAKE  | `PARSE_QUERY`  |
| kebab-case       | `parse-query`  |
| dot.case         | `parse.query`  |
| words            | `parse query`  |
| flat             | `parsequery`   |

- The value can be written in any of these styles too: `'parse query'`, `'parseQuery'` and `'parse_query'` are the
  same. Words are split at spaces, `_`, `-`, `.` and case changes; an uppercase run is one word: `HTTPServer` →
  `http`, `server`. Digits stay with the preceding word: `utf8Decode` → `utf8`, `decode`.
- With `/w` the whole identifier must match: `text has 'user id' /n/w` matches `userId` and `user_id`, but not
  `userIdentity`.
- Works on `path` and `name` too: `name has 'config loader' /n` finds `config_loader.rs`, `ConfigLoader.java`,
  `config-loader.ts`.
- `/n` can't be combined with `/e` and is not allowed with `~`, `~~`, `=~`.

### `~~` — fuzzy

For `path` and `name`. Like fzf: the characters of the value must appear in this order, not necessarily next to each
other, case-insensitive. Several space separated terms must all match.

```
name ~~ 'cfgldr'               # config_loader.rs, ConfigLoader.java
path ~~ 'srv hndl'             # src/server/handlers.rs, services/http/handler.go
```

Every match gets a score from 0 to 1. It is higher when the characters are at word starts (after `/`, `_`, `-`, `.`
and case changes), consecutive, in the file name rather than in directories, and when the value is shorter. Matches
below [`query.fuzzy_min_score`](CONFIG.md#query) (default 0.5) are dropped. The score is one of the
[relevance](OUTPUT.md#relevance) signals, so with the default `sort = relevance` the best matches tend to come first.

## Labels and condition ids

Every condition gets an id, so the output can tell which condition matched a file or a fragment, see
[OUTPUT.md](OUTPUT.md#condition-ids).

- By default the ids are `#1`, `#2`, ... in the order of appearance in the query. Simple conditions, `line( ... )`
  groups and `near( ... )` get ids, logical operators don't. Only conditions without a label are numbered, so the
  numbers have no gaps: in `text has 'a' @x or text has 'b'` the second condition is `#1`.
- A label `@name` after a condition (after its modifiers) or after a `line( ... )` / `near( ... )` replaces the number.
  A label starts with a letter or `_` and contains letters, digits, `_` and `-`. Labels must be unique.

```
name ~ '*.rs' and line(text has 'unwrap(' @unwrap or text has 'expect(' @expect) @panics
```

| Id        | Condition                                                |
|-----------|----------------------------------------------------------|
| `#1`      | `name ~ '*.rs'`                                          |
| `@panics` | `line(text has 'unwrap(' or text has 'expect(')`         |
| `@unwrap` | `text has 'unwrap('`                                     |
| `@expect` | `text has 'expect('`                                     |

Labels are useful with `or`: the output shows which branch has matched.

## Grammar

```
query       = [ or_expr ]
or_expr     = xor_expr { "or" xor_expr }
xor_expr    = and_expr { "xor" and_expr }
and_expr    = unary { "and" unary }
unary       = "not" unary | primary
primary     = "(" or_expr ")"
            | ( line | near | condition ) [ label ]
line        = "line" "(" or_expr ")"
near        = "near" "(" or_expr "," or_expr "," integer { "," near_option } ")"
near_option = "any" | "ordered" | "all" | "closest" | "first"
condition   = "empty"
            | enum_target ( "=" | "!=" ) word
            | enum_target "in" "(" word { "," word } ")"
            | text_target text_op string { modifier }
            | cmp_target cmp_op value
            | cmp_target "in" value ".." value
text_target = "path" | "name" | "text"
enum_target = "type" | "lang" | "kind" | "git"
cmp_target  = "size" | "modified" | "created" | "accessed" | "len"
text_op     = "=" | "has" | "starts" | "ends" | "~" | "~~" | "=~"
cmp_op      = "=" | "!=" | "<" | "<=" | ">" | ">="
modifier    = "/i" | "/w" | "/m" | "/c" | "/n" | "/e" "(" style "," levels ")"
style       = "backslash" | "quote_doubling" | "xml" | "url"
levels      = integer | integer ".." integer | "any"
label       = "@" ( letter | "_" ) { letter | digit | "_" | "-" }
string      = "'" ... "'" | '"' ... '"' | "`" ... "`"
value       = size | date | integer
word        = letter { letter | digit | "_" | "-" }
integer     = digit { digit }
size        = integer [ "." integer ] [ "B" | "K" | "M" | "G" ]
date        = ISO 8601 date or date-time | "-" integer ( "s" | "m" | "h" | "d" | "w" )
```

Whitespace between tokens is free, line breaks are allowed. `#` outside of strings starts a comment until the end of
the line, so long queries can be documented:

```
name ~ '*.rs'                      # Rust only
and not path ~ '**/tests/**'       # skip tests
and text has 'unwrap(' /c          # outside of comments
```

## Errors

The query is checked before any file is read. An error has a code, the position, a message and a hint, so the agent
can fix the query itself. The response format is described in [OUTPUT.md](OUTPUT.md#errors).

```
error[unknown_operator]: unknown operator 'contains'
  name ~ '*.rs' and text contains 'x'
                         ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

| Code                     | Example                                    | Hint                                             |
|--------------------------|--------------------------------------------|--------------------------------------------------|
| `unexpected_token`       | `name ~ '*.rs' text has 'x'`               | missing `and` / `or` between conditions?         |
| `unexpected_end`         | `name ~ '*.rs' and`                        | a condition is expected after `and`              |
| `unknown_target`         | `content has 'x'`                          | did you mean `text`?                             |
| `unknown_operator`       | `text contains 'x'`                        | did you mean `has`? expected one of: =, has, starts, ends, ~, ~~, =~ |
| `operator_not_allowed`   | `text ~ '*foo*'`                           | wildcards work on `path` and `name`; use `has`   |
| `modifier_not_allowed`   | `name = 'a' /m`                            | `/m` works with `=~` on `text` in file scope     |
| `unterminated_string`    | `text has 'abc`                            | closing `'` is missing                           |
| `invalid_regex`          | ``text =~ `(?<=a)b` ``                     | look-arounds are not supported                   |
| `invalid_wildcard`       | `name ~ '[a-'`                             | unclosed `[`                                     |
| `invalid_size`           | `size > 10MB`                              | units: B, K, M, G                                |
| `invalid_date`           | `modified > -1y`                           | units: s, m, h, d, w                             |
| `invalid_range`          | `size in 10M..1M`                          | the start must not be greater than the end       |
| `line_target_outside`    | `len > 100`                                | `len` works only inside `line( ... )` or `near( ... )` |
| `file_target_inside`     | `line(name ~ '*.rs')`                      | move `name` out of `line( ... )`                 |
| `nested_line`            | `line(line(text has 'a'))`                 | `line( ... )` can't be nested                    |
| `unknown_value`          | `lang = rusty`                             | did you mean `rust`?                             |
| `invalid_near_option`    | `near(text has 'a', text has 'b', 3, nearest)` | options: any, ordered, all, closest, first   |
| `conflicting_options`    | `near(text has 'a', text has 'b', 3, any, ordered)` | choose one of: any, ordered             |
| `invalid_escape_level`   | `text has 'a' /e(backslash, 9)`            | max level is 3 (configuration `escape.max_level`) |
| `duplicate_label`        | `text has 'a' @x or text has 'b' @x`       | labels must be unique                            |

## Recipes

### Like `find`

| Task                                     | `find`                                  | Query                                          |
|------------------------------------------|-----------------------------------------|------------------------------------------------|
| Files by extension                       | `find . -name '*.rs'`                   | `name ~ '*.rs'`                                |
| Several extensions                       | `find . -name '*.ts' -o -name '*.tsx'`  | `name ~ '*.{ts,tsx}'`                          |
| Case-insensitive name                    | `find . -iname 'readme*'`               | `name ~ 'readme*' /i`                          |
| By path                                  | `find . -path '*/migrations/*.sql'`     | `path ~ '**/migrations/*.sql'`                 |
| Empty directories                        | `find . -type d -empty`                 | `type = dir and empty`                         |
| Big files                                | `find . -size +100M`                    | `size > 100M`                                  |
| Changed in the last 24 hours             | `find . -mtime -1`                      | `modified > -1d`                               |
| Not changed for a year                   | `find . -mtime +365`                    | `modified < -365d`                             |
| Symlinks                                 | `find . -type l`                        | `type = link`                                  |
| Everything except a directory            | `find . -not -path './vendor/*'`        | `not path starts 'vendor/'`                    |

### Like `grep`

| Task                                     | `grep`                                  | Query                                          |
|------------------------------------------|-----------------------------------------|------------------------------------------------|
| Fixed string                             | `grep -rn 'TODO' .`                     | `text has 'TODO'`                              |
| Case-insensitive whole word              | `grep -rniw 'error' .`                  | `text has 'error' /i/w`                        |
| Regex                                    | `grep -rnE 'fn \w+_test' .`             | ``text =~ `fn \w+_test` ``                     |
| Files without a match                    | `grep -rLi 'license' .`                 | `not text has 'license' /i`                    |
| Lines without a match                    | `grep -v '^#' app.conf`                 | `name = 'app.conf' and line(not text starts '#')` |
| Only in some files                       | `grep -rn --include='*.py' 'import os'` | `name ~ '*.py' and text has 'import os'`       |

`-q`, `-c` and `-l` are [detail levels](OUTPUT.md#detail-levels), `-A/-B/-C` and `-m` are
[output options](OUTPUT.md#options), and `-o` is the `text` of the [fragments](OUTPUT.md#fragments).

### Beyond `grep` and `find`

Both words on the same line:

```
line(text has 'Mutex' and text has 'async')
```

Both words in the file, any lines:

```
text has 'tokio' and text has 'rayon'
```

Rust files which mention exactly one of two logging libraries — files mentioning both are excluded:

```
name ~ '*.rs' and (text has 'log::' xor text has 'tracing::')
```

JavaScript with very long lines, usually minified code. Generated files are skipped by default, the `kind` condition
brings them back (see [Generated files](#generated-files)):

```
lang = javascript and kind in (source, generated) and line(len > 500)
```

Rust files which don't end with a line break (a quoted string, because `` `\n` `` would be two characters):

```
name ~ '*.rs' and not text ends '\n'
```

A test function whose name ends with `_slow`, after `#[test]` with only whitespace between, multiline regex:

```
name ~ '*.rs' and text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m
```

`.unwrap()` and `.expect(` outside of comments, in Rust files outside of `tests` directories changed during the last 7
days, with labels to see which pattern matched:

```
name ~ '*.rs'
and modified > -7d
and not path ~ '**/tests/**'
and (text has '.unwrap()' /c @unwrap or text has '.expect(' /c @expect)
```

YAML, `.env` and `.properties` files with a line where a password key has a non-empty value, except lines starting
with `#`:

```
name ~ '*.{yml,yaml,env,properties}' and line(text =~ `(?i)password\s*[:=]\s*\S+` and not text starts '#')
```

The admin role in test fixtures, also inside stringified JSON:

```
path ~ '**/fixtures/**' and text has `"role": "admin"` /e(backslash, any)
```

Big log files that mention a request id. Files bigger than [`traversal.max_file_size`](CONFIG.md#traversal) (default
10M) are not searched by content, so the request raises it, e.g. `"traversal": { "max_file_size": "1G" }`:

```
name ~ '*.log' and size > 10M and text has 'req-7f3a9c'
```

Log files that mention the request id, also compressed (`.log.gz`) and archived ones, with
[`traversal.archives`](CONFIG.md#archives) = `all`; big ones only with the raised `max_file_size`, as above. `*.log`
also matches `.log` members inside `.zip` and `.tar.gz` archives, because `name` of a member is its own name:

```
name ~ '*.{log,log.gz}' and text has 'req-7f3a9c'
```

Only inside archives:

```
path ~ '**/*!/**' and text has 'req-7f3a9c'
```

### Finding a place by a vague idea

The file of a class which loads the configuration, exact name unknown:

```
kind = source and (name has 'config loader' /n or name ~~ 'cfgload')
```

Where a function is defined, in languages with an `fn`, `def`, `func` or `function` keyword, any naming style:

```
kind = source and line(text has 'parse query' /n/w and text =~ `\b(fn|def|func|function)\b`)
```

Source and test files changed in this branch which still have a TODO:

```
git = branch and kind in (source, test) and text has 'TODO'
```

## Gotchas

- `path ~ 'tests/**'` matches only the top level `tests` directory; `path ~ '**/tests/**'` matches any of them.
- `path has '/tests/'` doesn't match `tests/a.rs`: the path is relative and has no leading `/`.
- `name ~ '*.rs' and text has 'a' or text has 'b'` — `and` binds tighter than `or`, use brackets.
- `text has 'a' and text has 'b'` means anywhere in the file, `line(text has 'a' and text has 'b')` means the same
  line.
- Inside raw strings `` `...` `` there are no escapes: `` `\n` `` is two characters. For a line break in `=`, `has`,
  `starts`, `ends` use `'\n'`; in a regex `` `\n` `` works, because the regex itself understands `\n`.
- `.` in a regex matches any character; ``name =~ `.rs$` `` also matches `furs`. Use `name ~ '*.rs'` or
  ``name =~ `\.rs$` ``.
- Regex without anchors finds a match anywhere: ``name =~ `test` `` matches `contest.rs`.
- Files bigger than [`traversal.max_file_size`](CONFIG.md#traversal) and, with the default `traversal.binary = skip`,
  binary files are not searched by content: both `text has 'x'` and `not text has 'x'` are unknown for them, so they
  are never returned because of their content, see
  [Files whose content is not searched](#files-whose-content-is-not-searched).
