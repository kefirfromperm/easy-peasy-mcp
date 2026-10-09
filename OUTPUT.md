# Output

What the `find` command returns and how to control it. The query itself is described in [QUERY.md](QUERY.md), the
defaults of all options in [CONFIG.md](CONFIG.md#output).

- [Options](#options)
- [Detail levels](#detail-levels)
- [Response](#response)
- [Condition ids](#condition-ids)
- [Fragments](#fragments)
- [Highlight](#highlight)
- [Limits and truncation](#limits-and-truncation)
- [Pagination](#pagination)
- [Warnings](#warnings)
- [Errors](#errors)
- [Examples](#examples)

## Options

The `output` field of the `find` request. Every option is optional, missing ones are taken from the configuration.

```json
{
  "dirs": ["."],
  "query": "name ~ '*.rs' and text has 'unwrap(' /c",
  "output": {
    "format": "text",
    "detail": "lines",
    "context": 2,
    "max_files": 20
  }
}
```

| Option                 | Values                                          | Default    | Meaning                                                   |
|------------------------|-------------------------------------------------|------------|-----------------------------------------------------------|
| `format`               | `text`, `json`                                  | `text`     | response format                                           |
| `detail`               | `exists`, `count`, `files`, `lines`             | `lines`    | how much to return, see [Detail levels](#detail-levels)   |
| `meta`                 | `true`, `false`                                 | `false`    | add file type, size and modification date to every file   |
| `context`              | number                                          | `0`        | lines before and after every matched line (`grep -C`)     |
| `before`               | number                                          | `context`  | lines before (`grep -B`)                                  |
| `after`                | number                                          | `context`  | lines after (`grep -A`)                                   |
| `path_fragments`       | `true`, `false`                                 | `false`    | return fragments of `path` and `name` conditions          |
| `highlight`            | `auto`, `none`, `columns`, `markers`            | `auto`     | how matches are marked, see [Highlight](#highlight)       |
| `max_files`            | number                                          | `100`      | max files in the response                                 |
| `max_lines`            | number                                          | `500`      | max matched lines in the response, context not counted    |
| `max_matches`          | number                                          | `1000`     | max fragments in the response                             |
| `max_matches_per_file` | number                                          | `50`       | max fragments per file (`grep -m`)                        |
| `max_line_length`      | number                                          | `300`      | longer lines are cut around the matches                   |
| `sort`                 | `path`, `modified`, `size`, `matches` + `asc` / `desc` | `path asc` | order of files                                 |
| `path_style`           | `relative`, `absolute`                          | `relative` | how paths are written                                     |
| `cursor`               | string                                          | —          | continue a truncated response, see [Pagination](#pagination) |

## Detail levels

| Level    | Returns                                                          | Like                    |
|----------|------------------------------------------------------------------|-------------------------|
| `exists` | only whether at least one file matches; stops at the first match | `grep -q`               |
| `count`  | totals: files, lines, matches                                    | `grep -c`, `find \| wc` |
| `files`  | totals + the list of files with their counts and conditions      | `find`, `grep -l`       |
| `lines`  | totals + files + matched lines with fragments and context        | `grep -n -C`            |

Use the smallest level which answers the question: `exists` and `count` are cheap in tokens, `exists` is also the
fastest.

## Response

### Fields

Top level

| Field           | Levels             | Meaning                                                            |
|-----------------|--------------------|--------------------------------------------------------------------|
| `exists`        | all                | `true` if at least one file matches                                |
| `counts.files`  | `count` and higher | number of matched files                                            |
| `counts.lines`  | `count` and higher | number of matched lines                                            |
| `counts.matches`| `count` and higher | number of content fragments                                        |
| `counts_exact`  | `count` and higher | `false` if the search was stopped by the timeout, counts are lower bounds |
| `truncated`     | `files`, `lines`   | `true` if any limit was hit, see [Limits](#limits-and-truncation)  |
| `truncated_by`  | `files`, `lines`   | which limits were hit, e.g. `["max_files"]`                        |
| `next_cursor`   | `files`, `lines`   | cursor for the next page, only if truncated                        |
| `legend`        | `files`, `lines`   | condition id → condition text, see [Condition ids](#condition-ids) |
| `warnings`      | all                | problems which didn't stop the search, see [Warnings](#warnings)   |
| `files`         | `files`, `lines`   | the matched files                                                  |

Per file

| Field            | Levels           | Meaning                                                             |
|------------------|------------------|---------------------------------------------------------------------|
| `path`           | `files`, `lines` | file path, see below                                                |
| `type`           | with `meta`      | `file`, `dir`, `link`                                               |
| `size`           | with `meta`      | size in bytes                                                       |
| `modified`       | with `meta`      | modification date, ISO 8601 with the offset                         |
| `lines`          | `files`, `lines` | number of matched lines in the file                                 |
| `matches`        | `files`, `lines` | number of fragments in the file                                     |
| `conditions`     | `files`, `lines` | ids of the conditions which are true for this file                  |
| `path_fragments` | with `path_fragments` | fragments of `path` / `name` conditions                        |
| `blocks`         | `lines`          | matched lines with context, see [Fragments](#fragments)             |

Paths

- `relative`: relative to the working directory. With several working directories the directory is written before
  the path as it was given in the request: `dirs: ["../api", "../web"]` → `../api/src/main.rs`, `../web/src/app.ts`.
  The working directory `.` is omitted: `src/main.rs`.
- `absolute`: the full canonical path.

Counts

- a **matched line** is a line with at least one fragment, or a line matched by a `line( ... )` group
- a **match** is a content fragment; `path` / `name` fragments are not counted
- counts are always the real totals, even if the response is truncated (unless `counts_exact` is `false`)

### JSON format

```json
{
  "exists": true,
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "counts_exact": true,
  "truncated": false,
  "legend": {
    "#1": "name ~ '*.rs'",
    "#2": "text has 'unwrap(' /c"
  },
  "warnings": [],
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        {
          "lines": [
            { "n": 41, "text": "    let raw = fs::read_to_string(path)?;" },
            { "n": 42, "text": "    let cfg: Config = toml::from_str(&raw).unwrap();",
              "fragments": [ { "col": [44, 51], "text": "unwrap(", "ids": ["#2"] } ] },
            { "n": 43, "text": "    Ok(cfg)" }
          ]
        }
      ]
    }
  ]
}
```

Empty and default values are omitted: no `next_cursor` when not truncated, no `warnings` when there are none, etc.

### Text format

The same data, compact, in the style of `grep`. Uses about 2–3 times fewer tokens than JSON.

```
files: 1  lines: 1  matches: 1
#1 name ~ '*.rs'
#2 text has 'unwrap(' /c

src/config.rs  [#1 #2]  lines: 1  matches: 1
41-    let raw = fs::read_to_string(path)?;
42:    let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
43-    Ok(cfg)
```

- the header line: totals and `truncated: yes`, `next cursor: ...` when truncated
- the legend: one condition per line
- per file: the path, the matched conditions in `[ ]`, counts; with `meta` also `file 2.1K 2026-10-08 14:03`
- `42:` — a matched line, `41-` — a context line, `--` — a gap between blocks, like in `grep`
- warnings at the end, prefixed with `warning:`

## Condition ids

Every condition of the query has an id: `#1`, `#2`, ... in the order of appearance, or a label `@name` given in the
query, see [QUERY.md](QUERY.md#labels-and-condition-ids).

The `legend` maps the ids to the condition text as written in the query, without labels.

### File conditions

`conditions` of a file lists the ids of all conditions which are true for this file, at the file level: simple file
scope conditions, `line( ... )` groups and `near( ... )`. It shows which branch of `or` / `xor` has matched:

```
name ~ '*.rs' and (text has '.unwrap()' @unwrap or text has '.expect(' @expect)
```

```
src/a.rs  [#1 @unwrap]
src/b.rs  [#1 @expect]
src/c.rs  [#1 @unwrap @expect]
```

A condition is listed only if it is true itself. For `not text has 'license' @lic` the file is returned because `@lic`
is false, so `@lic` is not listed.

Conditions inside `line( ... )` and `near( ... )` are not listed per file — they are true or false for a line, see
fragments.

### Fragment ids

Every fragment lists the ids of the conditions which produced it, see [Fragments](#fragments).

## Fragments

A fragment is a matched part of a line.

| Field    | Meaning                                                                                  |
|----------|------------------------------------------------------------------------------------------|
| `col`    | `[start, end]`: 1-based, in Unicode characters, `end` is exclusive                       |
| `text`   | the matched text as it is in the file                                                    |
| `ids`    | the conditions which produced it                                                         |
| `escape` | the escape level, only for conditions with `/e`                                          |
| `to`     | only for multiline matches: `{ "n": 58, "col": 12 }` — the line and the exclusive end column |

Which conditions produce fragments

- `text` conditions with `=`, `has`, `starts`, `ends`, `=~` — the matched text
- inside `line( ... )`: the fragments get the id of the condition and the id of the group, e.g. `["#2", "#3"]`
- a `line( ... )` group without positive text conditions (e.g. `line(len > 200)` or `line(not text starts '#')`)
  produces one fragment which is the whole line, with the id of the group
- inside `near( ... )`: the fragments get the id of the inner condition and the id of `near`; both lines are shown
- conditions under `not` never produce fragments: there is nothing to show for a missing match
- `size`, dates, `type`, `empty`, `len` never produce fragments
- `path` and `name` conditions produce `path_fragments` only with the `path_fragments` option

Several conditions can match the same text. Overlapping fragments are all reported; identical ones are merged into one
fragment with all ids.

### Blocks

Matched lines and their context are grouped into blocks. If the context of two matched lines touches or overlaps, they
are in the same block, so no line is repeated. Between blocks the text format prints `--`.

### Long lines

A line longer than `max_line_length` is cut to windows around its fragments; the cuts are marked with `…` and the line
gets `"cut": true`. `col` always refers to the original line.

```json
{ "n": 1, "cut": true, "text": "…var a=require(\"./x\");var b=a.unwrap();var c=…",
  "fragments": [ { "col": [18342, 18348], "text": "unwrap", "ids": ["#1"] } ] }
```

## Highlight

How the fragments are shown in the lines.

| Style     | JSON                                         | Text                                              |
|-----------|----------------------------------------------|---------------------------------------------------|
| `none`    | no `fragments`, matched lines get `"match": true` | lines as they are                            |
| `columns` | `fragments` with `col`, `text`, `ids`        | after the line: `  @ 44..51 #2`                   |
| `markers` | markers inside `text`, no `fragments`        | markers inside the line                           |
| `auto`    | `columns`                                    | `markers`                                         |

Markers wrap a fragment with its ids: `⟦ids:text⟧`, ids separated by `,`.

```
42:    let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
```

Overlapping fragments are merged into one marked part with all ids. The escape level is written after the ids:
`⟦#1/e1:\"type\": \"admin\"⟧`. A multiline fragment opens on its first line and closes on its last one.

The markers are `⟦` and `⟧` by default and can be changed in the [configuration](CONFIG.md#output). If a file contains
the marker characters, the response has a warning and `columns` should be used.

## Limits and truncation

| Limit                  | Counts                                       | When hit                                         |
|------------------------|----------------------------------------------|--------------------------------------------------|
| `max_files`            | files in the response                        | next files are not listed                        |
| `max_lines`            | matched lines in the response, all files     | next lines are not listed                        |
| `max_matches`          | fragments in the response, all files         | next fragments are not listed                    |
| `max_matches_per_file` | fragments of one file                        | next fragments of this file are not listed, the next file is processed |
| timeout                | [configuration](CONFIG.md#server)            | the search stops, `counts_exact: false`          |

When a limit is hit:

- `truncated` is `true`, `truncated_by` names the limits
- `counts` still contain the real totals, so the agent knows how much is missing
- `next_cursor` is given to get the rest, except for `max_matches_per_file`, which is a per file cut by design

The files are sorted before the limits are applied, so `sort: "modified desc", max_files: 10` returns the 10 most
recently changed matching files.

## Pagination

To get the next page, repeat the request with the same `dirs`, `query`, `traversal` and `output`, and add `cursor`
from `next_cursor`:

```json
{ "dirs": ["."], "query": "text has 'TODO'", "output": { "max_files": 20, "cursor": "eyJ2IjoxLCJoIjoiOWMxZiIsImYiOjIwfQ" } }
```

- the cursor is opaque, it encodes the request hash and the position (file index and fragment index inside the file)
- the server is stateless, every page runs the search again
- a cursor with a different request is rejected with `cursor_mismatch`
- if files change between pages, results can shift; the response has a warning when the totals differ from the
  previous page

## Warnings

Problems which don't stop the search. The search continues, the response has them in `warnings`:

| Code                   | Meaning                                                                    |
|------------------------|----------------------------------------------------------------------------|
| `permission_denied`    | a file or directory can't be read, it is skipped                           |
| `too_big`              | a file is bigger than `max_file_size`, content conditions are false for it |
| `binary_skipped`       | a binary file is skipped by content conditions                             |
| `invalid_utf8`         | a file has invalid UTF-8, the bad bytes are replaced with `�`              |
| `date_unsupported`     | the file system has no `created` / `accessed` dates                        |
| `marker_in_content`    | a line contains the highlight markers                                      |
| `timeout`              | the search was stopped by the timeout                                      |
| `results_changed`      | totals differ from the previous page                                       |

Warnings of the same code are grouped, with a few example paths:

```json
"warnings": [
  { "code": "permission_denied", "count": 3, "paths": ["secrets/a.key", "secrets/b.key", "root-only/"] }
]
```

## Errors

If the request can't be executed, the tool returns an MCP error (`isError: true`) with one error.

```json
{
  "error": {
    "code": "unknown_operator",
    "message": "unknown operator 'contains'",
    "query": "name ~ '*.rs' and text contains 'x'",
    "position": { "col": 24, "len": 8 },
    "hint": "expected one of: =, has, starts, ends, ~, =~"
  }
}
```

In the text format:

```
error[unknown_operator]: unknown operator 'contains'
  name ~ '*.rs' and text contains 'x'
                         ^^^^^^^^
  hint: expected one of: =, has, starts, ends, ~, =~
```

Query errors are listed in [QUERY.md](QUERY.md#errors). Request errors:

| Code                | Meaning                                                    |
|---------------------|------------------------------------------------------------|
| `dir_not_found`     | a working directory doesn't exist                          |
| `dir_not_allowed`   | a working directory is outside of the allowed roots        |
| `invalid_option`    | an unknown output / traversal option or a wrong value      |
| `cursor_mismatch`   | the cursor belongs to another request                      |
| `cursor_invalid`    | the cursor can't be decoded                                |

## Examples

All examples search in a small Rust project.

### Does anything match?

```json
{ "query": "text has 'dbg!(' /c", "output": { "detail": "exists" } }
```

```
exists: no
```

### How many?

```json
{ "query": "name ~ '*.rs' and text has 'TODO'", "output": { "detail": "count" } }
```

```
files: 7  lines: 12  matches: 12
```

### Which files (like `find` / `grep -l`)

```json
{ "query": "name ~ '*.rs' and size > 20K", "output": { "detail": "files", "meta": true, "sort": "size desc" } }
```

```
files: 2  lines: 0  matches: 0
#1 name ~ '*.rs'
#2 size > 20K

src/parser.rs  [#1 #2]  file 48.2K 2026-10-07 18:22
src/eval.rs  [#1 #2]  file 23.9K 2026-10-02 09:10
```

### Lines with context (like `grep -n -C1`)

```json
{
  "query": "name ~ '*.rs' and line(text has 'unwrap(' @unwrap or text has 'expect(' @expect)",
  "output": { "context": 1 }
}
```

```
files: 2  lines: 3  matches: 3
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' or text has 'expect(')
@unwrap text has 'unwrap('
@expect text has 'expect('

src/config.rs  [#1 #2]  lines: 2  matches: 2
41-    let raw = fs::read_to_string(path)?;
42:    let cfg: Config = toml::from_str(&raw).⟦#2,@unwrap:unwrap(⟧);
43:    let port = cfg.port.⟦#2,@expect:expect(⟧"port is required");
44-    Ok(cfg)

src/main.rs  [#1 #2]  lines: 1  matches: 1
17-    let args = Args::parse();
18:    let cfg = config::load(&args.config).⟦#2,@unwrap:unwrap(⟧);
19-    run(cfg)
```

### Columns instead of markers

```json
{ "query": "text has 'unwrap(' @unwrap", "output": { "highlight": "columns" } }
```

```
src/config.rs  [@unwrap]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).unwrap();
  @ 44..51 @unwrap
```

### Lines without a match (like `grep -v`)

```json
{ "query": "name = 'app.conf' and line(not text starts '#' and len > 0)" }
```

The group has no positive text conditions, so the whole line is the fragment:

```
files: 1  lines: 2  matches: 2
#1 name = 'app.conf'
#2 line(not text starts '#' and len > 0)
#3 text starts '#'
#4 len > 0

config/app.conf  [#1 #2]  lines: 2  matches: 2
4:⟦#2:listen = 8080⟧
9:⟦#2:log_level = debug⟧
```

### Proximity

```json
{ "query": "near(text has '.lock()' @lock, text has '.await' @await, 3)", "output": { "context": 0 } }
```

```
files: 1  lines: 2  matches: 2
#1 near(text has '.lock()', text has '.await', 3)
@lock text has '.lock()'
@await text has '.await'

src/cache.rs  [#1]  lines: 2  matches: 2
57:        let mut guard = self.inner.⟦#1,@lock:.lock()⟧.unwrap();
--
59:        let value = fetch(key)⟦#1,@await:.await⟧?;
```

### Escaped JSON

```json
{ "query": "text has `\"role\": \"admin\"` /e(backslash, any)", "output": { "format": "json" } }
```

```json
{
  "exists": true,
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has `\"role\": \"admin\"` /e(backslash, any)" },
  "files": [
    {
      "path": "tests/fixtures/users.json",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 3, "text": "  { \"name\": \"root\", \"role\": \"admin\" },",
            "fragments": [ { "col": [21, 36], "text": "\"role\": \"admin\"", "ids": ["#1"], "escape": 0 } ] }
        ] },
        { "lines": [
          { "n": 8, "text": "  \"payload\": \"{\\\"role\\\": \\\"admin\\\"}\"",
            "fragments": [ { "col": [16, 35], "text": "\\\"role\\\": \\\"admin\\\"", "ids": ["#1"], "escape": 1 } ] }
        ] }
      ]
    }
  ]
}
```

### Multiline match

```json
{ "query": "text =~ `#\\[test\\]\\s*\\n\\s*fn \\w+_slow` /m" }
```

```
files: 1  lines: 2  matches: 1
#1 text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m

src/eval.rs  [#1]  lines: 2  matches: 1
210:    ⟦#1:#[test]
211:    fn eval_big_input_slow⟧() {
```

### Truncated

```json
{ "query": "text has 'TODO'", "output": { "detail": "files", "max_files": 2, "sort": "modified desc" } }
```

```
files: 7  lines: 12  matches: 12  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiOWMxZiIsImYiOjJ9
#1 text has 'TODO'

src/parser.rs  [#1]  lines: 3  matches: 3
src/main.rs  [#1]  lines: 1  matches: 1
```

The next page:

```json
{
  "query": "text has 'TODO'",
  "output": { "detail": "files", "max_files": 2, "sort": "modified desc", "cursor": "eyJ2IjoxLCJoIjoiOWMxZiIsImYiOjJ9" }
}
```

### With warnings

```
files: 1  lines: 1  matches: 1
#1 text has 'password'

config/dev.env  [#1]  lines: 1  matches: 1
3:DB_⟦#1:password⟧=changeme

warning: permission_denied (2): secrets/prod.env, secrets/stage.env
warning: too_big (1): data/dump.sql
```
