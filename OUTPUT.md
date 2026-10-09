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
- [Relevance](#relevance)
- [Funnel](#funnel)
- [Budget](#budget)
- [Facets](#facets)
- [Compact output](#compact-output)
- [Result ids](#result-ids)
- [Batch](#batch)
- [Warnings](#warnings)
- [Errors](#errors)
- [Examples](#examples)

## Options

The `output` field of the `find` request. Every option is optional, missing ones are taken from the configuration.
The time limit is the `timeout` field of the request, see [TOOLS.md](TOOLS.md#find).

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
| `detail`               | `exists`, `count`, `facets`, `files`, `lines`   | `lines`    | how much to return, see [Detail levels](#detail-levels)   |
| `budget`               | number of tokens                                | not set    | fit the response into the budget, see [Budget](#budget)   |
| `funnel`               | `auto`, `always`, `never`                       | `auto`     | explain empty and huge results, see [Funnel](#funnel)     |
| `facets`               | list of `dir`, `ext`, `lang`, `kind`, `git`, `condition` | `[dir, lang, kind, condition]` | facets, see [Facets](#facets) |
| `facet_depth`          | number                                          | `2`        | directory depth of the `dir` facet                        |
| `facet_limit`          | number                                          | `8`        | max values per facet                                      |
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
| `max_output_per_file`  | number                                          | `4000`     | max characters of matched lines and context of one file   |
| `max_output`           | number                                          | `40000`    | max characters of the whole response                      |
| `dedent`               | `true`, `false`                                 | `true`     | remove the common indentation of every block, see [Compact output](#compact-output) |
| `blank_context`        | `true`, `false`                                 | `false`    | keep empty context lines                                  |
| `group_dirs`           | `true`, `false`                                 | `true`     | text format: group files under directory headers          |
| `group_repeated`       | number                                          | `3`        | a matched line repeated in this many files is printed once; `0` — off |
| `sort`                 | `relevance`, or `path`, `modified`, `size`, `matches` + `asc` / `desc` | `relevance` | order of files, see [Relevance](#relevance); equal values are ordered by the full path |
| `path_style`           | `relative`, `absolute`                          | `relative` | how paths are written                                     |
| `cursor`               | string                                          | —          | continue a truncated response, see [Pagination](#pagination) |

## Detail levels

| Level    | Returns                                                          | Like                    |
|----------|------------------------------------------------------------------|-------------------------|
| `exists` | only whether at least one file matches; stops at the first match | `grep -q`               |
| `count`  | totals: files, lines, matches                                    | `grep -c`, `find \| wc` |
| `facets` | totals + counts of files per directory, language, kind, condition | —                      |
| `files`  | totals + the list of files with their counts and conditions      | `find`, `grep -l`       |
| `lines`  | totals + files + matched lines with fragments and context        | `grep -n -C`            |

Use the smallest level which answers the question: `exists` and `count` are cheap in tokens, `exists` is also the
fastest. A typical way to the exact place: `facets` to see where the matches are, then `files` in the right
directory, then `lines` for a few files. Or just set a [budget](#budget) and let the server choose.

## Response

### Fields

Top level

| Field           | Levels             | Meaning                                                            |
|-----------------|--------------------|--------------------------------------------------------------------|
| `version`       | all, JSON only     | version of the response format, see [TOOLS.md](TOOLS.md#versioning) |
| `exists`        | all                | `true` if at least one file matches                                |
| `result`        | `count` and higher | id of this result, see [Result ids](#result-ids)                   |
| `detail_used`   | `count` and higher | only if the [budget](#budget) lowered the detail level              |
| `funnel`        | `count` and higher | see [Funnel](#funnel)                                              |
| `hints`         | `count` and higher | see [Funnel](#funnel)                                              |
| `facets`        | `facets`           | see [Facets](#facets)                                              |
| `repeated`      | `lines`            | matched lines repeated in many files, see [Compact output](#compact-output) |
| `unchanged`     | with `since`       | number of files which match as before, see [Result ids](#result-ids) |
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
| `lang`, `kind`   | with `meta`      | language and kind of the file                                       |
| `modified`       | with `meta`      | modification date, ISO 8601 with the offset                         |
| `encoding`       | `files`, `lines` | only if the file is not UTF-8, e.g. `utf-16le`, `windows-1251`      |
| `score`          | with `meta`      | relevance score, 0–1                                                |
| `change`         | with `since`     | `added`, `removed`, `changed`, see [Result ids](#result-ids)        |
| `truncated`      | `files`, `lines` | `true` if a per file limit cut the file's lines                     |
| `lines`          | `files`, `lines` | number of matched lines in the file                                 |
| `matches`        | `files`, `lines` | number of fragments in the file                                     |
| `conditions`     | `files`, `lines` | ids of the conditions which are true for this file                  |
| `path_fragments` | with `path_fragments` | fragments of `path` / `name` conditions                        |
| `blocks`         | `lines`          | matched lines with context, see [Fragments](#fragments)             |

#### Paths

- `relative`: relative to the working directory. With several working directories the directory is written before
  the path as it was given in the request: `dirs: ["../api", "../web"]` → `../api/src/main.rs`, `../web/src/app.ts`.
  The working directory `.` is omitted: `src/main.rs`.
- `absolute`: the full canonical path.
- Paths are written in the native form of the platform: `src/main.rs` on Linux and macOS, `src\main.rs` on Windows.
- A member of an archive: `<archive path>!/<member path>`, e.g. `logs/2026-10.zip!/app/server.log`; the part after
  `!/` always uses `/`.

#### Duplicates

Every file is returned once, even if it is reachable several times: through overlapping working directories
(`dirs: [".", "src"]`), symlinks or hard links. Files are identified by the file system (device and inode on Linux
and macOS, volume and file index on Windows). A file is reported under the path found first: working directories in
the order of the request, inside a directory in path order.

#### Counts

- a **matched line** is a line with at least one fragment, or a line matched by a `line( ... )` group
- a **match** is a content fragment; `path` / `name` fragments are not counted
- counts are always the real totals, even if the response is truncated (unless `counts_exact` is `false`)

### JSON format

```json
{
  "version": 1,
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

- the header line: totals, `result: r12`, and `truncated: yes`, `next cursor: ...` when truncated
- the legend: one condition per line
- per file: the path, the matched conditions in `[ ]`, counts, `(cut)` if a per file limit was hit, the encoding if
  it is not UTF-8; with `meta` also `file 2.1K 2026-10-08 14:03`
- `42:` — a matched line, `41-` — a context line, `--` — a gap between blocks, like in `grep`
- warnings at the end, prefixed with `warning:`

## Condition ids

Every condition of the query has an id: `#1`, `#2`, ... in the order of appearance, or a label `@name` given in the
query, see [QUERY.md](QUERY.md#labels-and-condition-ids).

The `legend` maps the ids to the condition text as written in the query, without labels.

### File conditions

`conditions` of a file lists the ids of the conditions which **made the file match**, at the file level: simple file
scope conditions, `line( ... )` groups and `near( ... )`. It shows which branch of `or` / `xor` has matched:

```
name ~ '*.rs' and (text has '.unwrap()' @unwrap or text has '.expect(' @expect)
```

```
src/a.rs  [#1 @unwrap]
src/b.rs  [#1 @expect]
src/c.rs  [#1 @unwrap @expect]
```

Which conditions made the file match, from the top of the query down:

- the whole query is true, so it contributes
- a true `and` which contributes — all its parts contribute
- a true `or` / `xor` which contributes — only its true parts contribute
- `not` — nothing under it contributes: for `not text has 'license' @lic` the file is returned because `@lic` is
  false, so `@lic` is not listed

A condition which is true, but in a branch which didn't make the file match, is not listed:

```
(text has 'TODO' @todo and size > 1M @big) or name ~ '*.md' @md
```

A small `README.md` with `TODO` inside is listed with `[@md]` only: `@todo` is true, but its branch is false because
of `@big`. The same rule decides which conditions produce fragments.

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

- only the conditions which made the file match, see [File conditions](#file-conditions); a true condition in a
  false branch produces nothing
- `text` conditions with `=`, `has`, `starts`, `ends`, `=~` — the matched text
- inside `line( ... )`: the fragments get the id of the condition and the id of the group, e.g. `["#2", "#3"]`
- a `line( ... )` group without positive text conditions (e.g. `line(len > 200)` or `line(not text starts '#')`)
  produces one fragment which is the whole line, with the id of the group
- inside `near( ... )`: the fragments get the id of the inner condition and the id of `near`; which lines are
  reported depends on the `report` option of `near`, see [QUERY.md](QUERY.md#proximity)
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
| `max_output_per_file`  | characters of lines and context of one file  | next lines of this file are not listed, the next file is processed |
| `max_output`           | characters of the whole response             | next lines and files are not listed              |
| `timeout`              | request field or [configuration](CONFIG.md#server) | the search stops, the results found so far are returned, `counts_exact: false` |

When a limit is hit:

- `truncated` is `true`, `truncated_by` names the limits
- `counts` still contain the real totals, so the agent knows how much is missing
- a file cut by `max_matches_per_file` or `max_output_per_file` gets `"truncated": true`, in the text format `(cut)`
  after its counts
- `next_cursor` is given to get the rest, except for the per file limits, which cut by design, and for `timeout`,
  where the unsearched part is unknown

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

## Relevance

`sort = relevance` (the default) puts the files most likely to be the one the agent is looking for first, so a small
`max_files` returns the right files, not the first ones in alphabetical order.

The score of a file, from 0 to 1, combines:

| Signal                         | Effect                                                                     |
|--------------------------------|----------------------------------------------------------------------------|
| fuzzy score                    | the best score of the `~~` conditions                                      |
| query words in the path        | words of the `text` values found in the file name, then in the path: searching `'parse query' /n` ranks `parse_query.rs` high |
| match density                  | matches per line of the file, log-scaled: a small file full of matches beats a huge file with a few |
| matched conditions             | more contributing conditions of an `or` / `xor` — higher                   |
| kind                           | `source` > `test` > `config` > `doc` > `other`, a light weight             |
| recency                        | recently modified files get a light bonus                                  |
| depth                          | shallower paths get a light bonus                                          |

- the score is deterministic for the same files and query, equal scores are ordered by the full path
- with `meta` every file shows its `score`
- the weights are internal and may be tuned without a new spec version: the order is a ranking, not a contract

## Funnel

When a query returns nothing or far too much, the agent usually guesses and retries several times. The funnel
explains the result, so it can be fixed in one step.

`funnel = auto` adds it when the result is empty, or when the number of matched files is more than 10 times
`max_files`. `always` adds it to every response with `count` detail or higher, `never` — never.

### Steps

For a query of conditions joined with `and`, every step shows how many files are left after the condition, in the
order of the query. A part in brackets or a `line( ... )` / `near( ... )` group is one step:

```
files: 0
funnel:
  lang = rust                          312
  and modified > -7d                    41
  and text has 'parseQuery'              0
```

For a query joined with `or` every branch shows its own count. The steps are counted during the normal search, they
cost nothing: the evaluation of a file stops at the first false condition anyway.

### Hints

For an empty result the server tries cheap variations of the condition where the count dropped to 0, only on the
files which passed the steps before it, and reports the ones which match:

| Hint                               | Example                                                         |
|------------------------------------|-----------------------------------------------------------------|
| other case                         | `text has 'parseQuery' /i` → 2 files                            |
| other naming style                 | `text has 'parseQuery' /n` → 4 files (`parse_query`)            |
| without `/w`, `has` instead of `=` | `name has 'config'` → 3 files                                   |
| skipped files                      | 5 matching files ignored by `.gitignore`, 2 generated, 1 too big |
| similar paths                      | `path starts 'src/parsers/'`: no such directory, similar: `src/parser/` |
| similar names                      | `name = 'config.yml'`: similar: `config.yaml`                   |

```
files: 0
funnel:
  lang = rust                          312
  and modified > -7d                    41
  and text has 'parseQuery'              0
hints:
  text has 'parseQuery' /n             4 files: parse_query
  skipped: 5 ignored by .gitignore (traversal.ignore_files)
```

The hints use at most 20% of the timeout. For a huge result the hints are the top values of the `dir` and `lang`
facets, to show where to narrow:

```
files: 1840  lines: 9211  matches: 10380  truncated: yes (max_files)
funnel:
  text has 'error'                    1840
hints:
  most in: vendor/ 1210 · src/ 402 · tests/ 228
  narrow with: not path ~ 'vendor/**', kind = source, /w
```

## Budget

`budget` is the max size of the response in tokens. The server estimates tokens as characters divided by
[`output.chars_per_token`](CONFIG.md#output) (default 4) and lowers the detail step by step until the response fits:

1. as requested
2. without context lines
3. lines cut to 120 characters
4. `detail = files`, the list truncated to the budget with a cursor, if at least 10 files (or all of them) fit
5. `detail = facets`
6. `detail = count`

- the response says which level was used: `detail: files (budget 2000)`, in JSON `"detail_used": "files"`
- the budget never raises the detail above the requested one
- the `max_*` limits still apply; the budget is a limit on top of them

```json
{ "query": "lang = rust and text has 'unwrap(' /c", "output": { "budget": 1500 } }
```

The agent gets the most useful answer which costs at most about 1500 tokens, without guessing `max_files` and
`context`.

## Facets

`detail = facets` returns the totals and the number of matched files per value of every facet: a cheap map of where
the matches are.

```
files: 162  lines: 410  matches: 455
dir:   tests/integration/ 88 · src/parser/ 41 · tests/unit/ 24 · src/eval/ 9
lang:  rust 150 · toml 12
kind:  test 112 · source 38 · config 12
cond:  #1 162 · @unwrap 120 · @expect 61
```

| Facet       | Values                                                                        |
|-------------|-------------------------------------------------------------------------------|
| `dir`       | directories at `facet_depth` levels below the working directory; files higher up are counted in their own directory |
| `ext`       | file extensions                                                               |
| `lang`      | languages                                                                     |
| `kind`      | kinds                                                                         |
| `git`       | git statuses                                                                  |
| `condition` | condition ids, the files where the condition contributed                     |

Values are sorted by count, more than `facet_limit` values end with `… 5 more (21)`. In JSON:
`"facets": { "dir": { "tests/integration/": 88, ... } }`.

## Compact output

Text format options which save tokens without losing information.

### Dedent

`dedent = true` removes the common leading whitespace of every block. The number of removed characters is written
once at the start of the block, `col` still refers to the original line:

```
src/server/handlers.rs  [#1]  lines: 1  matches: 1
⇥12
140-let user = session.user()?;
141:let id = user.⟦#1:unwrap(⟧).id;
142-audit(id);
```

In JSON the block has `"indent": 12`.

### Blank context lines

With `blank_context = false` empty and whitespace-only context lines are skipped; the line numbers show the gap.

### Directory groups

With `group_dirs = true` files are written under a directory header, with paths relative to it:

```
src/parser/
lexer.rs  [#1]  lines: 2  matches: 2
...
ast.rs  [#1]  lines: 1  matches: 1
...
src/eval/
mod.rs  [#1]  lines: 1  matches: 1
...
```

JSON paths are always full.

### Repeated lines

A matched line which is identical (ignoring leading and trailing whitespace, with the same condition ids) in at least
`group_repeated` files is printed once in a `repeated` section before the files, with the number of files and a few
locations:

```
repeated:
  ×37 use anyhow::⟦#1:Result⟧;    src/main.rs:3, src/config.rs:5, src/eval/mod.rs:2, … 34 more
```

In the files below these lines are not repeated. A file which has no other matched lines is not printed in `lines`
detail at all; `detail = files` still lists every file. The counts are not changed.

## Result ids

Every response with `count` detail or higher has a result id, e.g. `result: r12` in the header. The server keeps the
last [`server.result_cache`](CONFIG.md#server) results in memory (default 50, until the server stops): the request,
all matched files (not only the shown ones) and a fingerprint of their matched lines.

### Refine: `within`

```json
{ "within": "r12", "query": "text has 'retry'" }
```

Searches only the files of result `r12`: "of those, which also have X?". There is no directory walk, so it is fast,
and the agent doesn't have to repeat the first query. `dirs` and `traversal` of the request are ignored, the files are
read again, so changes are seen. Files deleted since `r12` are skipped.

### Changes: `since`

```json
{ "since": "r12", "query": "lang = rust and text has 'unwrap(' /c" }
```

Runs the request and returns only the differences to `r12`: useful while the agent edits files and checks its
progress.

| Mark | `change`  | The file                                                     |
|------|-----------|--------------------------------------------------------------|
| `+`  | `added`   | matches now, didn't match in `r12`                           |
| `-`  | `removed` | matched in `r12`, doesn't match now (or is deleted)          |
| `~`  | `changed` | matches in both, but the matched lines are different         |

```
files: 12  lines: 30  matches: 31  result: r15  since: r12  unchanged: 10
+ src/cache.rs  [#1]  lines: 1  matches: 1
57:let v = map.get(k).⟦#1:unwrap(⟧);
~ src/config.rs  [#1]  lines: 1  matches: 1
42:let cfg: Config = toml::from_str(&raw).⟦#1:unwrap(⟧);
- src/main.rs
```

Changed files show all their current matched lines, removed files only the path. The counts are of the current
result.

## Batch

Several queries in one call: one round trip, one response.

```json
{
  "dirs": ["."],
  "output": { "detail": "files", "budget": 3000 },
  "queries": [
    { "label": "defs", "query": "lang = rust and text =~ `fn\\s+load_config`" },
    { "label": "calls", "query": "lang = rust and text has 'load_config(' and not text =~ `fn\\s+load_config`" },
    { "label": "tests", "query": "kind = test and text has 'load_config' /n", "output": { "detail": "count" } }
  ]
}
```

- `queries` replaces `query`; every item has a `query`, an optional `label` and may override `dirs`, `timeout`,
  `traversal` and `output`
- the other fields of the request are shared by all queries
- the queries run in parallel; queries with the same `dirs` and `traversal` share one directory walk
- `budget` and `max_output` are for the whole response, split equally between the queries; what a query doesn't use
  goes to the next ones
- every query has its own result id, legend and funnel
- an invalid query has an error in its section, the others still run; `isError` is true only if all of them fail

```
== defs ==  files: 1  result: r20
src/config.rs  [#1]  lines: 1  matches: 1
== calls ==  files: 3  result: r21
src/main.rs  [#1]  lines: 1  matches: 1
src/cli.rs  [#1]  lines: 1  matches: 1
src/server/mod.rs  [#1]  lines: 2  matches: 2
== tests ==  files: 2  lines: 5  matches: 5  result: r22
```

## Warnings

Problems which don't stop the search. The search continues, the response has them in `warnings`:

| Code                   | Meaning                                                                    |
|------------------------|----------------------------------------------------------------------------|
| `permission_denied`    | a file or directory can't be read, it is skipped                           |
| `too_big`              | a file is bigger than `max_file_size`, content conditions are false for it |
| `binary_skipped`       | a binary file is skipped by content conditions                             |
| `decode_errors`        | a file has bytes invalid in its encoding, they are replaced with `�`       |
| `archive_error`        | an archive can't be read (corrupted, encrypted, unsupported), it is skipped |
| `archive_too_big`      | decompressed content exceeds `max_archive_size`, the rest is skipped       |
| `date_unsupported`     | the file system has no `created` / `accessed` dates                        |
| `marker_in_content`    | a line contains the highlight markers                                      |
| `timeout`              | the search was stopped by the timeout                                      |
| `results_changed`      | totals differ from the previous page                                       |
| `not_a_git_repo`       | the query has `git` conditions, but a file is outside of a repository      |

Warnings of the same code are grouped, with a few example paths:

```json
{
  "warnings": [
    {
      "code": "permission_denied",
      "count": 3,
      "paths": [
        "secrets/a.key",
        "secrets/b.key",
        "root-only/"
      ]
    }
  ]
}
```

## Errors

If the request can't be executed, the tool returns a result with `isError: true` and one error, see
[TOOLS.md](TOOLS.md#errors). A cancelled request returns nothing, as MCP requires.

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
| `result_expired`    | `within` / `since` refers to a result which is unknown or was evicted      |
| `unsupported_version` | the requested `version` is not supported, see [TOOLS.md](TOOLS.md#versioning) |

## Examples

All examples search in a small Rust project. To keep them focused, they leave out the result id and use
`dedent = false` and `group_dirs = false`; see [Compact output](#compact-output) for how these look.

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
