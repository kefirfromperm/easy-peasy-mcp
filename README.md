# Easy-Peasy MCP Server
The main goal is to allow to an agent to find a file or files with complex criteria.

I will use RUST

Candidate crates: `ignore` (parallel directory walking, `.gitignore` support), `grep-regex` / `grep-searcher` (fast
content search, binary detection), `globset` (wildcards), `pest` or `chumsky` (query parser).

## Commands

- find
- get config
- update config

## Request
The request contains fields

- working directory
- traversal options
- search criteria
- output format

### Working directory
One or more paths to search in. Every path must be inside one of the allowed roots (see Configuration).

### Traversal options

- min depth / max depth
- follow symlinks (default: no; loops are detected, symlinks can't escape the allowed roots)
- include hidden files (default: no)
- respect ignore files: `.gitignore`, `.ignore`, global git excludes (default: yes)
- exclude paths: wildcards for directories/files that are pruned and never walked (e.g. `target/`, `node_modules/`)
- binary files: skip / search / list only (default: skip for content conditions)
- max file size to scan content (bigger files are matched only by metadata conditions)

### Search criteria

Search criteria is a query with multiply conditions which can be combined with AND, OR, XOR and NOT logical operators.

The query is a compact text expression, e.g.

```
name ~ '*.rs' and modified > -7d and not path has '/tests/'
and line(text has 'unwrap' /i and not text starts '//')
```

means: Rust files changed during the last 7 days, not in tests, with a line which contains `unwrap` (any case) and is
not a `//` comment.

#### Condition scope

Every condition has an explicit scope:

- **file scope** — conditions on file metadata and on the whole file content. `text has 'A' and text has 'B'` means A
  and B can be on different lines. `not text has 'X'` means files without a match (like `grep -L`).
- **line scope** — a `line( ... )` group inside a file-level expression. All conditions inside the group must be true
  for the same line. `not` inside the group means lines without a match (like `grep -v`). A file matches the `line`
  group if at least one of its lines matches it, so `not line( ... )` means no line of the file matches it.

#### Targets

| Target     | Scope | Value                                         |
|------------|-------|-----------------------------------------------|
| `path`     | file  | file path, text                               |
| `name`     | file  | file name with extension, text                |
| `type`     | file  | `file`, `dir`, `link`                         |
| `size`     | file  | size                                          |
| `modified` | file  | date                                          |
| `created`  | file  | date                                          |
| `accessed` | file  | date                                          |
| `empty`    | file  | no value, empty file or empty directory       |
| `text`     | both  | whole file content in file scope, line content inside `line( ... )` |
| `len`      | line  | line length in characters, number             |

Directories can be in the result only via metadata conditions (`path`, `name`, `type`, dates, `empty`). Content
conditions never match a directory.

Proximity: `near(<line expr>, <line expr>, N)` — a file scope condition, true if a line matching the first expression
is within N lines of a line matching the second one, e.g. `near(text has 'lock()', text has 'unwrap', 3)`.

#### Operators

Text operators (`path`, `name`, `text`)

| Operator | Meaning                                                                 |
|----------|-------------------------------------------------------------------------|
| `=`      | exact match                                                             |
| `has`    | contains                                                                |
| `starts` | starts with                                                             |
| `ends`   | ends with                                                               |
| `~`      | wildcard: `*`, `**`, `?`, `[abc]`, `{a,b}` (for `path` and `name` only) |
| `=~`     | regex (Rust `regex` crate syntax)                                       |

Comparison operators (`size`, dates, `len`): `=`, `!=`, `<`, `<=`, `>`, `>=`, `in <from>..<to>`

- sizes accept units: `10`, `10K`, `5M`, `1G`
- dates are absolute ISO 8601 (`2026-01-01`, `2026-01-01T10:00`) or relative to now (`-30m`, `-2h`, `-7d`, `-2w`),
  e.g. `modified > -7d` — changed during the last 7 days, `created in 2026-01-01..-7d`

Logical operators: `not`, `and`, `xor`, `or` (precedence from the highest to the lowest), brackets `( )`. Keywords are
case-insensitive.

#### Strings

- `'...'` and `"..."` — backslash escapes are supported: `\'`, `\"`, `\\`, `\n`, `\t`
- `` `...` `` — raw string, no escapes at all, everything until the closing backtick is the value

The query itself is sent inside a JSON tool argument, so every `"` in the query becomes `\"` there. Prefer `'...'`, and
`` `...` `` for regexes and texts with quotes and backslashes.

#### Modifiers

Modifiers are written after a text value and can be combined, e.g. `text has 'todo' /i/w`.

| Modifier              | Meaning                                                                     |
|-----------------------|-----------------------------------------------------------------------------|
| `/i`                  | case-insensitive (default: case-sensitive)                                  |
| `/w`                  | whole word                                                                  |
| `/m`                  | multiline, regex may match across lines (`=~` on `text` in file scope only) |
| `/c`                  | ignore matches inside comments                                              |
| `/e(style, levels)`   | escaping, see below                                                         |

Comments (`/c`): comment syntax is defined per file extension in the configuration (line comment like `//`, `#`,
block comment like `/* */`). Built-in defaults exist for common languages. Files with an unknown extension are searched
as is.

#### Escaping

Escaping allows to find a text which is stored escaped in the file, e.g. JSON inside a string in JSON, where `"` is
written as `\"` and on the next nesting level as `\\\"`.

The agent writes the pattern unescaped, the server builds the escaped variants of the pattern and matches them.

- escape style
  - `backslash` — JSON, C, Java, Rust and others: `"` → `\"`, `'` → `\'`, `\` → `\\`, new line → `\n`, tab → `\t`
  - `quote_doubling` — SQL, CSV: `'` → `''`, `"` → `""`
  - `xml` — HTML/XML entities: `"` → `&quot;`, `'` → `&apos;` / `&#39;`, `<` → `&lt;`, `>` → `&gt;`, `&` → `&amp;`
  - `url` — percent encoding: `"` → `%22`, space → `%20` / `+`
- escape levels: a number (`0` — raw, `1` — escaped once, `2` — escaped twice, ...), a range (`0..2` matches raw, once
  and twice escaped text) or `any` (from 0 to the max escape level from the configuration)

Example: find `"name": "x"` raw and inside a JSON string

```
text has `"name": "x"` /e(backslash, 0..1)
```

matches both `"name": "x"` and `\"name\": \"x\"`.

Escaping is supported for `=`, `has`, `starts` and `ends`. For regex the agent writes the escaped form itself.

#### Grammar

```
query      = or_expr
or_expr    = xor_expr { "or" xor_expr }
xor_expr   = and_expr { "xor" and_expr }
and_expr   = unary { "and" unary }
unary      = "not" unary | primary
primary    = "(" or_expr ")" | "line(" or_expr ")" | near | condition
near       = "near(" or_expr "," or_expr "," number ")"
condition  = "empty"
           | "type" ("=" | "!=") ("file" | "dir" | "link")
           | text_target text_op string { modifier }
           | cmp_target cmp_op value
           | cmp_target "in" value ".." value
text_target = "path" | "name" | "text"
cmp_target  = "size" | "modified" | "created" | "accessed" | "len"
text_op    = "=" | "has" | "starts" | "ends" | "~" | "=~"
cmp_op     = "=" | "!=" | "<" | "<=" | ">" | ">="
modifier   = "/i" | "/w" | "/m" | "/c" | "/e(" style "," levels ")"
```

#### Errors

A parse error contains the position and a hint, so the agent can fix the query itself, e.g.

```
col 23: unknown operator 'contains', expected one of: =, has, starts, ends, ~, =~
```

Semantic errors are reported the same way, e.g. `len` outside of `line( ... )`, `~` on `text`, `/m` inside
`line( ... )`.

#### Encoding

Files are read as UTF-8. Invalid byte sequences are replaced (lossy) and do not stop the search.

### Output format

Output can contain fields

- exists YES/NO (true if at least one file is found)
- files count
- line count
- matches count
- truncated YES/NO (true if any of the max limits is hit; counts above still contain the real totals)
- next cursor (to continue the search from the place where it was truncated)

Per file fields

- path (relative to the working directory or absolute)
- file type
- file size
- file modification date
- line count
- matches count
- matched fragments with
- matched fragments line numbers
- the whole lines of matched fragments with lines before and after with size how many lines before and after have
  to show

Also output format includes

- max file count
- max line count
- max matches count
- max matches per file
- sort by: path, modification date, size, matches count; ascending / descending
- path style: relative / absolute
- cursor (from the previous truncated response)

## Configuration

The configuration contains default values for

- allowed roots (the server never reads outside of them; `update config` can't widen them, they are set only in the
  config file or the command line at server start)
- working directory (by default it's only current working directory)
- traversal options
- output format
- comment syntax per file extension
- max escape level for `any` (default: 3)
- search timeout
