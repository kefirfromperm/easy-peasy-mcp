# Output

What the `find` command returns and how to control it. The query itself is described in [QUERY.md](QUERY.md), the
defaults of the options in [CONFIG.md](CONFIG.md#output).

To stay short, most examples leave out the result id and show `dedent = false` and `group_dirs = false`. The
[Text format](#text-format) and [JSON format](#json-format) sections show the output with the default options in
full.

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

The `output` field of the `find` request. Every option is optional; missing ones are taken from the configuration,
except `cursor`, which has no default. The time limit is the `timeout` field of the request, see
[TOOLS.md](TOOLS.md#timeout).

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
| `chars_per_token`      | number                                          | `4`        | how the budget converts characters to tokens, see [Budget](#budget) |
| `funnel`               | `auto`, `always`, `never`                       | `auto`     | explain empty and huge results, see [Funnel](#funnel)     |
| `facets`               | list of `dir`, `ext`, `lang`, `kind`, `git`, `condition` | `["dir", "lang", "kind", "condition"]` | facets, see [Facets](#facets) |
| `facet_depth`          | number                                          | `2`        | directory depth of the `dir` facet                        |
| `facet_limit`          | number                                          | `8`        | max values per facet                                      |
| `meta`                 | `true`, `false`                                 | `false`    | add type, size, modification date, language, kind and, with `sort = relevance`, the score to every file |
| `context`              | number                                          | `0`        | lines before and after every matched line (`grep -C`)     |
| `before`               | number                                          | `context`  | lines before (`grep -B`)                                  |
| `after`                | number                                          | `context`  | lines after (`grep -A`)                                   |
| `path_fragments`       | `true`, `false`                                 | `false`    | return fragments of `path` and `name` conditions          |
| `highlight`            | `auto`, `none`, `columns`, `markers`            | `auto`     | how matches are marked, see [Highlight](#highlight)       |
| `markers`              | two strings                                     | `["⟦", "⟧"]` | opening and closing marker of `highlight = markers`     |
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
| `result`        | `count` and higher | id of this result, see [Result ids](#result-ids); absent if `server.result_cache = 0` |
| `detail_used`   | `count` and higher | only if the [budget](#budget) lowered the detail level              |
| `funnel`        | `count` and higher | see [Funnel](#funnel)                                              |
| `hints`         | `count` and higher | see [Funnel](#funnel)                                              |
| `facets`        | `facets`           | see [Facets](#facets)                                              |
| `repeated`      | `lines`            | matched lines repeated in many files, see [Compact output](#compact-output) |
| `since`         | with `since`       | the result id the differences are to, see [Result ids](#result-ids) |
| `unchanged`     | with `since`       | number of files which match as before, see [Result ids](#result-ids) |
| `counts.files`  | `count` and higher | number of matched files                                            |
| `counts.lines`  | `count` and higher | number of matched lines                                            |
| `counts.matches`| `count` and higher | number of content fragments                                        |
| `counts_exact`  | `count` and higher | `false` if the search was stopped by the timeout, counts are lower bounds |
| `truncated`     | all                | `true` if any limit or the timeout was hit, see [Limits](#limits-and-truncation) |
| `truncated_by`  | all                | which limits were hit, e.g. `["max_files"]`                        |
| `next_cursor`   | `files`, `lines`   | cursor for the next page, only if truncated                        |
| `legend`        | `facets`, `files`, `lines` | condition id → condition text, if at least one file matches, see [Condition ids](#condition-ids) |
| `warnings`      | all                | problems which didn't make the request fail, see [Warnings](#warnings) |
| `files`         | `files`, `lines`   | the matched files                                                  |

Per file

| Field            | Levels           | Meaning                                                             |
|------------------|------------------|---------------------------------------------------------------------|
| `path`           | `files`, `lines` | file path, see below                                                |
| `type`           | with `meta`      | the entry: `file`, `dir`, `link`; a symlink is `link`               |
| `size`           | with `meta`      | size in bytes                                                       |
| `lang`, `kind`   | with `meta`      | language and kind of the file                                       |
| `modified`       | with `meta`      | modification date, ISO 8601 with the offset                         |
| `encoding`       | `files`, `lines` | only if the file is not UTF-8, e.g. `utf-16le`, `windows-1251`      |
| `score`          | with `meta`      | relevance score, 0–1, only with `sort = relevance`                  |
| `change`         | with `since`     | `added`, `removed`, `changed`, see [Result ids](#result-ids)        |
| `truncated`      | `files`, `lines` | `true` if a per file limit cut the file's lines                     |
| `lines`          | `files`, `lines` | number of matched lines in the file                                 |
| `matches`        | `files`, `lines` | number of fragments in the file                                     |
| `conditions`     | `files`, `lines` | ids of the conditions which made the file match, see [File conditions](#file-conditions) |
| `path_fragments` | with `path_fragments` | fragments of `path` / `name` conditions                        |
| `blocks`         | `lines`          | matched lines with context, see [Fragments](#fragments)             |

#### Paths

- `relative`: relative to the working directory. With several working directories the directory is written before
  the path as it was given in the request: `dirs: ["../api", "../web"]` → `../api/src/main.rs`, `../web/src/app.ts`.
  The working directory `.` is omitted: `src/main.rs`.
- `absolute`: the full canonical path: the canonical working directory (symlinks resolved, without the `\\?\`
  prefix on Windows) and the path inside it, a symlink inside it not resolved. The paths of the warnings and the
  directory headers of the text format are absolute too.
- Paths are written in the native form of the platform: `src/main.rs` on Linux and macOS, `src\main.rs` on Windows.
- A directory is written with a trailing separator: `src/parser/`; in a directory group its line is `parser/`. A
  symlink is written as the link itself, its target is not shown.
- A member of an archive: `<archive path>!/<member path>`, e.g. `logs/2026-10.zip!/app/server.log`; the part after
  `!/` always uses `/`.
- With [`within`](#refine-within) the paths are written as in the referred result.

#### Duplicates

Every file is returned once, even if it is reachable several times: through overlapping working directories
(`dirs: [".", "src"]`), symlinks or hard links. Files are identified by the file system (device and inode on Linux
and macOS, volume and file index on Windows). A file is reported under the path found first: working directories in
the order of the request, inside a directory in path order.

#### Counts

- a **matched line** is a line with at least one fragment, or a line matched by a `line( ... )` group
- a **match** is a content fragment; `path` / `name` fragments are not counted
- counts are always the real totals, even if the response is truncated (unless `counts_exact` is `false`)

#### Meta

With `meta` every file gets its metadata:

- `type`: `file`, `dir` or `link` (a symlink, followed or not)
- `size`: of a file, or of the file a followed symlink points to, in bytes; in the text format `512B` under 1K,
  otherwise with one decimal in the largest unit it is at least 1 of: `2.1K`, `48.2K`, `1.5M`, `3.0G` (powers of 1024)
- `lang`, `kind`: of a file, see [QUERY.md](QUERY.md#lang); `unknown` when no language is known
- `modified`: in JSON ISO 8601 in the time zone of the server, whole seconds: `2026-10-08T14:03:00+00:00`; in the
  text format `2026-10-08 14:03`
- a directory and a symlink which is not followed have only `type` and `modified`; a member of an archive is a
  `file` with the date stored in the archive, if there is one
- in the text format the values follow the counts and the encoding, separated by spaces, the missing ones left out:
  `file 2.1K 2026-10-08 14:03 rust source`, `dir 2026-10-03 12:00`

### JSON format

The response to this request; all other options are the defaults:

```json
{ "query": "name = 'config.rs' and text has 'unwrap(' /c", "output": { "context": 1, "format": "json" } }
```

```json
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": {
    "#1": "name = 'config.rs'",
    "#2": "text has 'unwrap(' /c"
  },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        {
          "indent": 4,
          "lines": [
            { "n": 41, "text": "let raw = fs::read_to_string(path)?;" },
            { "n": 42, "text": "let cfg: Config = toml::from_str(&raw).unwrap();",
              "fragments": [ { "col": [44, 51], "text": "unwrap(", "ids": ["#2"] } ] },
            { "n": 43, "text": "let port = cfg.port.expect(\"port is required\");" }
          ]
        }
      ]
    }
  ]
}
```

Empty and default values are omitted: no `truncated`, `truncated_by` and `next_cursor` when nothing was cut, no
`counts_exact` when the counts are exact, no `warnings` when there are none, no `files` and `legend` when nothing
matches, no `lines`, `matches` and `conditions` of a file when they are 0 or empty, etc. `indent` is the indentation
removed by [dedent](#dedent); `col` refers to the original line. The document is written without whitespace between
its tokens (the examples are indented for reading); `legend` lists the ids in the order of the query. A warning is
`{ "code", "count", "paths" }`, a warning which is not about entries only `{ "code" }`. With `format = "json"` an
error is the JSON document of [Errors](#errors); `position.col` is 1-based.

### Text format

The same data, compact, in the style of `grep`. Uses about 2–3 times fewer tokens than JSON. The same request without
`"format": "json"`:

```
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap(' /c

src/
config.rs  [#1 #2]  lines: 1  matches: 1
⇥4
41-let raw = fs::read_to_string(path)?;
42:let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
43-let port = cfg.port.expect("port is required");
```

- the header line: totals, `result: r1`, `detail: files (budget 2000)` when the [budget](#budget) lowered the
  detail, `since: r12  unchanged: 10` with [`since`](#changes-since), and `truncated: yes (...)`, `next cursor: ...`
  when something was cut; with `detail = exists` the header is `exists: yes` or `exists: no`, after a timeout also
  `truncated: yes (timeout)`
- the legend: one condition per line
- `funnel:` and `hints:`, when present, after the legend, see [Funnel](#funnel)
- `src/` — a directory header, the files under it are written relative to it, see
  [Directory groups](#directory-groups)
- per file: the path, the matched conditions in `[ ]` (left out when no condition made the file match: the empty
  query, or a match only through `not`), counts (left out when they are 0), `(cut)` if a per file
  limit was hit, the encoding if it is not UTF-8; with `meta` also type, size, date, language, kind and, with
  `sort = relevance`, the score:
  `file 2.1K 2026-10-08 14:03 rust source 0.82`
- `⇥4` — the indentation removed from the block, see [Dedent](#dedent)
- `42:` — a matched line, `41-` — a context line, `--` — a gap between blocks, like in `grep`
- warnings at the end, prefixed with `warning:`

## Condition ids

Every condition of the query has an id: `#1`, `#2`, ... in the order of appearance (only conditions without a label
are numbered), or a label `@name` given in the query, see [QUERY.md](QUERY.md#labels-and-condition-ids).

The `legend` maps the ids of all conditions of the query to their text as written in the query, without labels: the
tokens of the condition with their original spelling (keyword case, quotes, escapes), where any whitespace, line
breaks or comments between two tokens are written as one space and tokens written without a gap stay without one.
`NAME   ~\n'*.rs'` is in the legend as `NAME ~ '*.rs'`, `line( text has 'a' @x )` as `line( text has 'a' )`. The
legend lists the conditions in the order of appearance in the query. It is a part of the `facets`, `files` and `lines` details when at least one file matches; an empty result has no legend,
there are no ids to explain.

### File conditions

`conditions` of a file lists the ids of the conditions which **made the file match**, at the file level: simple file
scope conditions, `line( ... )` groups and `near( ... )`. It shows which branch of `or` / `xor` has matched:

```
name ~ '*.rs' and (text has '.unwrap()' @unwrap or text has '.expect(' @expect)
```

```
...
src/a.rs  [#1 @unwrap]  lines: 1  matches: 1
src/b.rs  [#1 @expect]  lines: 2  matches: 2
src/c.rs  [#1 @unwrap @expect]  lines: 2  matches: 2
```

Which conditions made the file match, from the top of the query down:

- the whole query is true, so it contributes
- a true `and` which contributes — all its parts contribute
- a true `or` / `xor` which contributes — only its true parts contribute
- `not` — nothing under it contributes: for `not text has 'license' @lic` the file is returned because `@lic` is
  false, so `@lic` is not listed; a file which matches only through `not` has an empty list

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

A fragment is a matched part of a line, or of several lines for a multiline match (see `to`).

| Field    | Meaning                                                                                  |
|----------|------------------------------------------------------------------------------------------|
| `col`    | `[start, end]`: 1-based, in Unicode characters, `end` is exclusive                       |
| `text`   | the matched text as it is in the file                                                    |
| `ids`    | the conditions which produced it                                                         |
| `escape` | the escape level (`0` — unescaped), only for conditions with `/e`                       |
| `to`     | only for multiline matches: `{ "n": 58, "col": 12 }` — the line and the exclusive end column; `col` of the fragment then ends at the end of its first line, the fragment is in the first line |

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
- `size`, dates, `type`, `empty`, `lang`, `kind`, `git`, `len` never produce fragments
- `path` and `name` conditions produce `path_fragments` only with the `path_fragments` option

Several conditions can match the same text. Overlapping fragments are all reported; identical ones (the same text and
escape level) are merged into one fragment with all ids.

### Path fragments

With `path_fragments` every listed file (`files` and `lines` detail) gets the fragments of the `path` and `name`
conditions which made it match. In JSON they are `path_fragments`: `[{ "col": [5, 11], "text": "config", "ids":
["#1"] }]`, `col` in the characters of `path`. In the text format a line after the file line shows the full path in
the [highlight](#highlight) style: `  path: src/⟦#1:config⟧.rs` with markers, `  path @ 5..11 #1` with `columns`, no
line with `none`.

- `=` and `~` match the whole name or path, so the fragment is all of it; `has`, `starts`, `ends`, `=~` and `/n` give
  every match; `~~` gives every run of matched characters
- the fragment of a `name` condition is in the name at the end of the path
- path fragments are not counted as matches; identical ones are merged into one with all ids, overlapping ones are
  listed one by one in JSON and marked as one part in the text

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

Overlapping fragments are merged into one marked part with all ids. The escape level of the fragments of a condition
with `/e` is written after the ids, level `0` too: `⟦#1/e1:\"type\": \"admin\"⟧`, `⟦#1/e0:"type": "admin"⟧`; a part
merged from fragments of several levels lists them in ascending order: `⟦#1,#2/e0/e1:…⟧`. A multiline fragment opens on its first line and closes on its last one.

The markers are `⟦` and `⟧` by default and can be changed in the [configuration](CONFIG.md#output). If a file contains
the marker characters, the response has a warning and `columns` should be used: in `lines` detail with `markers` a
file with a matched line which contains the opening or the closing marker is listed in `marker_in_content`.

`columns` in the text format: a matched line is followed by one line with the fragments which start in it, every
fragment as `@ <start>..<end> <ids>` — the columns of `col`, the ids separated by `,` with the escape levels as in the
markers (`#1/e0`) — and several fragments separated by two spaces: `  @ 5..9 #1  @ 20..24 #2,#3`. A multiline
fragment ends with the line and the column of its end: `  @ 12..58:3 #1`. Overlapping fragments are listed one by one,
as in JSON; the columns refer to the original line, also after [dedent](#dedent). A repeated line is followed by the
columns of its fragments in its text.

In JSON `"match": true` comes after the `text` of a line; with `markers` a repeated line has the markers in its `text`
and no `fragments`, with `none` neither.

## Limits and truncation

| Limit                  | Counts                                       | When hit                                         |
|------------------------|----------------------------------------------|--------------------------------------------------|
| `max_files`            | files in the response                        | next files are not listed                        |
| `max_lines`            | matched lines in the response, all files     | next lines are not listed                        |
| `max_matches`          | fragments in the response, all files         | next fragments are not listed                    |
| `max_matches_per_file` | fragments of one file                        | next fragments of this file are not listed, the next file is processed |
| `max_output_per_file`  | characters of lines and context of one file  | next lines of this file are not listed, the next file is processed |
| `max_output`           | characters of the whole response             | next lines and files are not listed              |
| `budget`               | estimated tokens of the whole response       | the detail is lowered, then the file list is cut, see [Budget](#budget) |
| `timeout`              | request field or [configuration](CONFIG.md#server) | the search stops, the results found so far are returned, `counts_exact: false` |

When a limit is hit:

- `truncated` is `true`, `truncated_by` names the limits
- `counts` still contain the real totals, so the agent knows how much is missing; after the `timeout` they are only
  lower bounds, `counts_exact: false`
- a file cut by `max_matches_per_file` or `max_output_per_file` gets `"truncated": true`, in the text format `(cut)`
  after its counts
- `next_cursor` is given to get the rest, except for the per file limits, which cut by design, and for `timeout`,
  where the unsearched part is unknown

The files are sorted before the limits are applied, so `sort: "modified desc", max_files: 10` returns the 10 most
recently changed matching files.

With `path`, `modified`, `size` and `matches` the files are ordered by the value, `asc` from the lowest, `desc` from
the highest; `matches` counts the content fragments. Files with equal values are ordered by the full path, in
ascending order also with `desc`; files without the value (the size of a directory, the date of a member of an
archive which has none) come last.

## Pagination

To get the next page, repeat the same request and add `output.cursor` from `next_cursor`:

```json
{ "dirs": ["."], "query": "text has 'TODO'", "output": { "max_files": 20, "cursor": "eyJ2IjoxLCJoIjoiYjI3ZSIsImYiOjIwLCJ0IjpbNDUsODAsOTZdfQ" } }
```

- the cursor is opaque, it encodes the request hash, the position (file index and fragment index inside the file)
  and the totals
- pages are not served from the [result cache](#result-ids): every page runs the search again
- a cursor with a different request is rejected with `cursor_mismatch`
- if files change between pages, results can shift; the response has a `results_changed` warning when the totals
  differ from the ones in the cursor

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
| kind                           | `source` > `test` > `config` > `doc` > `generated` > `other`, a light weight |
| recency                        | files modified closer to the newest matched file get a light bonus         |
| depth                          | shallower paths get a light bonus                                          |

- the score is deterministic for the same files and query: recency is measured from the newest matched file, not
  from the clock, so the pages of a [cursor](#pagination) keep their order; equal scores are ordered by the full path
- with `meta` every file shows its `score`; with another `sort` there is no score
- the weights are internal and may be tuned without a new spec version: the order is a ranking, not a contract

## Funnel

When a query returns nothing or far too much, the agent usually guesses and retries several times. The funnel
explains the result, so it can be fixed in one step.

`funnel = auto` adds it when the result is empty, or when the number of matched files is more than 10 times
`max_files`. `always` adds it to every response with `count` detail or higher, `never` — never. The
[hints](#hints) come with the funnel: `funnel = never` turns them off too.

### Steps

For a query of conditions joined with `and`, every step shows how many files are left after the condition. A part in
brackets or a `line( ... )` / `near( ... )` group is one step. The steps are in the order of evaluation: first the
metadata steps (`path`, `name`, `type`, `size`, dates, `empty`, `lang`, `kind`, `git`), then the steps with content
conditions, each group in the order of the query:

```
files: 0  lines: 0  matches: 0
funnel:
  lang = rust                          312
  and modified > -7d                    41
  and text has 'parseQuery'              0
```

For a query joined with `or` every branch shows how many files it matched; a file which matches several branches is
counted only in the first one. The steps are counted during the normal search, so they cost almost nothing. A file
for which a step is unknown (see [QUERY.md](QUERY.md#files-whose-content-is-not-searched)) doesn't pass the step; the
hints count it as skipped.

### Hints

For an empty result the server tries cheap variations of the condition where the count dropped to 0 on the files which
passed the steps before it and reports the ones which match; it also counts the skipped files which could match and
looks for similar paths and names:

| Hint                               | Example                                                         |
|------------------------------------|-----------------------------------------------------------------|
| other case                         | `text has 'parseQuery' /i` → 2 files (`ParseQuery`)             |
| other naming style                 | `text has 'parseQuery' /n` → 4 files (`parse_query`, `ParseQuery`) |
| without `/w`, `has` instead of `=` | `name has 'config'` → 3 files                                   |
| skipped files                      | 5 matching files ignored by `.gitignore`, 2 generated, 1 too big |
| similar paths                      | `path starts 'src/parsers/'`: no such directory, similar: `src/parser/` |
| similar names                      | `name = 'config.yml'`: similar: `config.yaml`                   |

```
files: 0  lines: 0  matches: 0
funnel:
  lang = rust                          312
  and modified > -7d                    41
  and text has 'parseQuery'              0
hints:
  text has 'parseQuery' /i             2 files: ParseQuery
  text has 'parseQuery' /n             4 files: parse_query, ParseQuery
  skipped: 5 ignored by .gitignore (traversal.ignore_files), 2 generated (traversal.skip_generated),
           1 too big (traversal.max_file_size)
```

The hints use at most 20% of the timeout, or 5 seconds without a timeout. For a huge result the hints show where the
matches are — the top values of the `dir` and `lang` facets — and a few ways to narrow the query:

```
files: 1840  lines: 9211  matches: 10380  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNGE3ZCIsImYiOjEwMCwidCI6WzE4NDAsOTIxMSwxMDM4MF19
#1 text has 'error'
funnel:
  text has 'error'                    1840
hints:
  dir: vendor/tokio/ 610 · vendor/hyper/ 600 · src/parser/ 402 · tests/integration/ 228
  lang: rust 1790 · c 50
  narrow with: not path ~ 'vendor/**', kind = source, /w

... the first 100 files
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

```json
{ "query": "kind in (source, test) and (text has 'unwrap(' @unwrap or text has 'expect(' @expect)", "output": { "detail": "facets" } }
```

```
files: 162  lines: 410  matches: 455
#1 kind in (source, test)
@unwrap text has 'unwrap('
@expect text has 'expect('
dir:        tests/integration/ 88 · src/parser/ 41 · tests/unit/ 24 · src/eval/ 9
lang:       rust 158 · python 4
kind:       test 112 · source 50
condition:  #1 162 · @unwrap 120 · @expect 61
```

| Facet       | Values                                                                        |
|-------------|-------------------------------------------------------------------------------|
| `dir`       | directories at `facet_depth` levels below the working directory; files higher up are counted in their own directory |
| `ext`       | file extensions                                                               |
| `lang`      | languages                                                                     |
| `kind`      | kinds                                                                         |
| `git`       | git statuses                                                                  |
| `condition` | condition ids: in how many files the condition made the file match; for a condition inside `line( ... )` / `near( ... )` — in how many files it produced a fragment |

Values are sorted by count, more than `facet_limit` values end with `… 5 more (21)`. In JSON:
`"facets": { "dir": { "tests/integration/": 88, ... } }`.

## Compact output

Options which save tokens. `dedent`, `blank_context` and `group_dirs` lose no information; `group_repeated` shows
only a few locations of a repeated line. `group_dirs` changes only the text format, the others change both formats.

### Dedent

`dedent = true` removes the common leading whitespace (spaces and tabs) of every block. The number of removed characters
is written once at the start of the block, `col` still refers to the original line. Empty and whitespace-only lines
don't count for the common whitespace, and it is never removed into a match: a fragment which starts in it keeps the
whitespace before it. A block without common whitespace has no `⇥` line:

```
...
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

- a header is written whenever the directory differs from the one of the previous file, so with `sort = relevance`
  the same directory can get several headers
- files directly in a working directory are under the header of that directory as written in the request: `./` for
  `.`, `../api/` for `../api`
- on Windows the header ends with `\`, like the paths

JSON paths are always full.

### Repeated lines

A matched line which is identical (ignoring leading and trailing whitespace, with the same condition ids) in at least
`group_repeated` files is printed once in a `repeated` section before the files, with the number of files and a few
locations:

```
...
repeated:
  ×37 use anyhow::⟦#1:Result⟧;    src/main.rs:3, src/config.rs:5, src/eval/mod.rs:2, … 34 more
```

In the files below these lines are not repeated. A file which has no other matched lines is not printed in `lines`
detail at all; `detail = files` still lists such a file. The counts are not changed.

- the section comes after the legend (and the funnel) and its empty line, only in `lines` detail
- a line is `  ×N <line>    <locations>`: `N` the number of files, the line without its leading and trailing
  whitespace and with its markers, after four spaces `path:line` of the first three files in the order of the files
  and `, … M more`; the lines are ordered by `N`, more first, then by their first file
- a line touched by a multiline match is not grouped; the context of a repeated line goes with it
- JSON: `"repeated": [{ "count": 37, "text": "use anyhow::Result;", "fragments": [...], "locations":
  ["src/main.rs:3", "src/config.rs:5", "src/eval/mod.rs:2"] }]` — `col` of the fragments counts in `text`, the
  locations are the first three

## Result ids

Every response with `count` detail or higher has a result id, e.g. `result: r12` in the header, unless
`server.result_cache = 0`. The server keeps the last [`server.result_cache`](CONFIG.md#server) results in memory
(default 50, until the server stops): the request, all matched files (not only the shown ones) and a fingerprint of
their matched lines. The ids count the kept results: a response without an id doesn't use one.

### Refine: `within`

```json
{ "within": "r12", "query": "text has 'retry'" }
```

Searches only the files of result `r12`: "of those, which also have X?". There is no directory walk, so it is fast,
and the agent doesn't have to repeat the first query. `dirs` and the walk options of `traversal` (depth, symlinks,
hidden files, ignore files, `exclude`) are ignored; the file and matching options (`binary`, `max_file_size`,
`skip_generated`, `encoding`, `fallback_encoding`, `archives`, `max_archive_size`, `path_case`) apply. The files are
read again, so changes are seen. Files deleted since `r12` are skipped. The paths are the ones of `r12`: they are
written as in `r12`, and `path` conditions see them relative to the working directories of `r12`.

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
#1 lang = rust
#2 text has 'unwrap(' /c

+ src/cache.rs  [#1 #2]  lines: 1  matches: 1
63:        let v = map.get(k).⟦#2:unwrap(⟧);
~ src/config.rs  [#1 #2]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
- src/main.rs
```

Changed files show their current matched lines, not only the changed ones; removed files show only the path. The
counts are of the current result.

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
  goes to the next ones; a query which sets its own `output.budget` or `output.max_output` gets exactly that, the
  rest is split between the others
- result ids, legends and funnels are per query, each where it applies (a `count` query has no legend)
- in the text format every section starts with `== label ==` (the number of the query if it has no label) on its
  header line
- an invalid query has an error in its section, the others still run; `isError` is true only if all of them fail

```
== defs ==  files: 1  lines: 1  matches: 1  result: r20
#1 lang = rust
#2 text =~ `fn\s+load_config`

src/config.rs  [#1 #2]  lines: 1  matches: 1
== calls ==  files: 3  lines: 4  matches: 4  result: r21
#1 lang = rust
#2 text has 'load_config('
#3 text =~ `fn\s+load_config`

src/main.rs  [#1 #2]  lines: 1  matches: 1
src/cli.rs  [#1 #2]  lines: 1  matches: 1
src/server/mod.rs  [#1 #2]  lines: 2  matches: 2
== tests ==  files: 2  lines: 5  matches: 5  result: r22
```

## Warnings

Problems which don't make the request fail: the response is returned and lists them in `warnings`:

| Code                   | Meaning                                                                    |
|------------------------|----------------------------------------------------------------------------|
| `permission_denied`    | a file or directory can't be read, or a symlink points outside of the allowed roots; it is skipped |
| `too_big`              | a file is bigger than `max_file_size`, content conditions are unknown for it |
| `binary_skipped`       | a binary file is not searched by content (`binary = skip`), content conditions are unknown for it |
| `decode_errors`        | a file has bytes invalid in its encoding, they are replaced with `�`       |
| `archive_error`        | an archive can't be opened (corrupted, encrypted, unsupported): its members are not searched, the archive file itself is still an entry |
| `archive_too_big`      | decompressed content exceeds `max_archive_size`, the rest is skipped       |
| `date_unsupported`     | the file system has no `created` / `accessed` dates, the condition is unknown |
| `marker_in_content`    | a line contains the highlight markers                                      |
| `timeout`              | the search was stopped by the timeout                                      |
| `results_changed`      | the totals differ from the ones in the cursor, files changed between pages |
| `not_a_git_repo`       | the query has `git` conditions, but a file is outside of a repository      |
| `git_base_not_found`   | `git.base` doesn't exist in the repository of a file: `git = branch` is `changed` there |

Unreadable entries (`permission_denied`): a directory which can't be listed is skipped with everything below it, also
by `type = dir`; a file which can't be opened is skipped when the server opens it — for its content when the other
conditions don't decide, and always before it is returned. A file which doesn't match by its other conditions is not
opened and gets no warning.

Warnings of the same code are grouped, with the number of entries and at most 3 example paths in path order, in the
order of the table above. In the text format every group is one line at the end of the response, after an empty
line: `warning: <code> (<count>): <path>, <path>, <path>`, and `, … N more` when more entries have the warning;
directories end with the separator. A warning which is not about entries (`timeout`) is just `warning: <code>`. A
response without files has the empty line and the warnings after its header and legend.

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

If the request can't be executed, the tool returns a result with `isError: true` and the error, see
[TOOLS.md](TOOLS.md#errors); in a [batch](#batch) every invalid query has its own error in its section. A cancelled
request returns nothing, as MCP requires.

```json
{
  "error": {
    "code": "unknown_operator",
    "message": "unknown operator 'contains'",
    "query": "name ~ '*.rs' and text contains 'x'",
    "position": { "col": 24, "len": 8 },
    "hint": "did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~"
  }
}
```

In the text format:

```
error[unknown_operator]: unknown operator 'contains'
  name ~ '*.rs' and text contains 'x'
                         ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

- the first line: `error[<code>]: <message>`
- for an error in the query: the query line where the error starts, indented by two spaces, and under it `^` markers
  for the error position; a query with several lines shows only that line, and `col` in JSON counts from the start of
  the whole query (a line break is one character)
- a position at the end of the query (`unexpected_end`) is one `^` right after the last character
- `  hint: ...` when there is a hint
- every line ends with a line break; a request error without a query position has only the first line and the hint

Query errors are listed in [QUERY.md](QUERY.md#errors). Request errors:

| Code                | Meaning                                                    |
|---------------------|------------------------------------------------------------|
| `dir_not_found`     | a working directory doesn't exist                          |
| `dir_not_allowed`   | a working directory is outside of the allowed roots        |
| `invalid_option`    | an unknown field or option, a wrong value, or both `query` and `queries` |
| `cursor_mismatch`   | the cursor belongs to another request                      |
| `cursor_invalid`    | the cursor can't be decoded                                |
| `result_expired`    | `within` / `since` refers to a result which is unknown or was evicted      |
| `unsupported_version` | the requested `version` is not supported, see [TOOLS.md](TOOLS.md#versioning) |
| `not_implemented`   | a valid feature of the spec which this server version doesn't implement yet: `error[not_implemented]: <feature> is not implemented yet`, with the position when the feature is in the query |

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

src/parser.rs  [#1 #2]  file 48.2K 2026-10-07 18:22 rust source
src/eval.rs  [#1 #2]  file 23.9K 2026-10-02 09:10 rust source
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
files: 2  lines: 2  matches: 2
@unwrap text has 'unwrap('

src/config.rs  [@unwrap]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).unwrap();
  @ 44..51 @unwrap
src/main.rs  [@unwrap]  lines: 1  matches: 1
18:    let cfg = config::load(&args.config).unwrap();
  @ 42..49 @unwrap
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
--
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
57:        let mut guard = self.inner.⟦#1,@lock:.lock()⟧;
--
59:        let value = fetch(key)⟦#1,@await:.await⟧?;
```

### Escaped JSON

```json
{ "query": "text has `\"role\": \"admin\"` /e(backslash, any)", "output": { "format": "json" } }
```

```json
{
  "version": 1,
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
files: 7  lines: 12  matches: 12  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiOWMxZiIsImYiOjIsInQiOls3LDEyLDEyXX0
#1 text has 'TODO'

src/parser.rs  [#1]  lines: 3  matches: 3
src/main.rs  [#1]  lines: 1  matches: 1
```

The next page:

```json
{
  "query": "text has 'TODO'",
  "output": { "detail": "files", "max_files": 2, "sort": "modified desc", "cursor": "eyJ2IjoxLCJoIjoiOWMxZiIsImYiOjIsInQiOls3LDEyLDEyXX0" }
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
