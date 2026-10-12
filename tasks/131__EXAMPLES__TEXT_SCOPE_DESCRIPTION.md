# Examples for task 131

The `find` description of [TOOLS.md#description](../TOOLS.md#description) tells that `text` is the whole file content
outside `line()` and one line inside it, and the pair of its new line on a fixture.

## Fixtures

### text-scope

```text tree
notes.md                   2026-10-01 10:00
run.sh                     2026-10-01 10:00
```

In `tests/fixtures/text-scope/`. `notes.md` starts with the heading `# Notes` and has the `## ` headings `## Install`
(line 3) and `## Use` (line 7); `run.sh` starts with `#!/bin/sh`.

## E131.1 The `find` description

The `text` line of the cheat sheet is followed by the contrasting pair: `text starts '#!'` on the whole file,
`line(text starts '## ')` on a line. The other tools are unchanged (E117.2).

Fixture: `text-scope`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "find",
        "description": "Search files by name, path, metadata and content in one query (find + grep in one). Read-only. Spec version 1.\nCall get_config once first to learn the allowed roots and defaults.\n\nQUERY: conditions joined with not, and, xor, or (this precedence) and brackets. Empty query = all files.\n  name ~ '*.rs'                  wildcard on the file name: * ? ** [abc] [!abc] {a,b}\n  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator\n  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)\n  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it\n  size > 1M  modified > -7d  created  accessed  type = dir  empty\n  lang = rust   kind in (source, test)   git = changed|branch|staged|untracked\n  name ~~ 'cfgldr'               fuzzy file name (also path ~~)\n  line(text has 'a' and not text has 'b')    conditions on the same line; len > 120 works inside line()\n  near(text has 'lock', text has 'await', 3[, ordered][, closest|first])   lines within 3 of each other\nModifiers after a text value: /i ignore case, /w whole word, /m multiline regex, /c skip comments,\n  /n any naming style ('parse query' = parseQuery, parse_query, PARSE_QUERY ...),\n  /e(backslash|quote_doubling|xml|url, 0..2|any) also match the escaped text (JSON inside JSON etc.)\nLabels: text has 'x' @name — the output shows which condition matched.\nStrings: '...' or \"...\" with \\ escapes; `...` raw, best for regex.\nDates: 2026-10-01, -30m -2h -7d -2w. Sizes: 512 10K 5M 1G.\n\nOUTPUT: output.detail = exists | count | facets | files | lines (default); output.context = N lines around.\noutput.budget = N tokens: the server picks the richest detail which fits. By default files are sorted by relevance.\nText response with the default options: \"42:\" matched line, \"41-\" context line, \"--\" gap, ⟦ids:text⟧ a match,\n[#1 @x] matched conditions, ⇥N removed indentation; empty or huge results come with a funnel and hints how to fix\nthe query.\nIf the response has next_cursor, pass it back as output.cursor with the same request to get the next page.\nResponses with count detail or higher have a result id (r12), unless the result cache is off: within = \"r12\"\nsearches only its files, since = \"r12\" returns only the changes.\nqueries = [{label, query}, ...] runs several queries in one call.\noutput.file = 'x.txt' writes the response to a file under the output dir and returns its path; for big results.\n\nEXAMPLES\n  name ~ '*.py' and text has 'import requests'\n  not text has 'license' /i                                   files without a match\n  name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')\n  text has `\"role\": \"admin\"` /e(backslash, any)               also inside stringified JSON\n  kind = source and name has 'config loader' /n               a file by a vague name\n  git = changed and text has 'dbg!('                          leftovers in files with uncommitted changes"
      },
      { "name": "get_config" },
      { "name": "update_config" }
    ]
  }
}
```

## E131.2 `text starts` outside `line()`: the file starts with it

`notes.md` has `## ` headings, but the file starts with `# Notes`: in file scope `starts` works on the whole content,
so no file matches.

Fixture: `text-scope`

```json request
{"query": "name ~ '*.md' and text starts '## '"}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~ '*.md'                          1
  and text starts '## '                  0
```

## E131.3 `text starts` inside `line()`: a line starts with it

The same condition inside `line()` works on each line and finds both `## ` headings.

Fixture: `text-scope`

```json request
{"query": "name ~ '*.md' and line(text starts '## ')"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name ~ '*.md'
#2 line(text starts '## ')
#3 text starts '## '

./
notes.md  [#1 #2]  lines: 2  matches: 2
3:⟦#2,#3:## ⟧Install
--
7:## Use
```

## E131.4 `text starts '#!'`: the file starts with it

The first half of the pair: a script starts with a shebang.

Fixture: `text-scope`

```json request
{"query": "text starts '#!'", "output": {"detail": "files"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text starts '#!'

./
run.sh  [#1]  lines: 1  matches: 1
```
