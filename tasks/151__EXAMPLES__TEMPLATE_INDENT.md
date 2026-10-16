# Examples for task 151

The placeholder `{indent}` of a `replace` template ([TOOLS.md](../TOOLS.md#templates)): the spaces and tabs at the
start of the line where the match starts. The examples are written for spec 2. They are startup examples with
`--allow-write`, like the examples of `replace` of task 129; the dry run of each shows the new lines.

## Fixtures

### template-indent

```text tree
data/four.json       2026-10-01 10:00
data/tabs.json       2026-10-01 10:00
data/two.json        2026-10-01 10:00
src/lib.rs           2026-10-01 10:00
```

In `tests/fixtures/template-indent/`, the same JSON list in three indentations:

- `data/two.json`: `{`, `  "omit": [`, `    "left_out",` (line 3), `    "warnings"`, `  ]`, `}`: two spaces
- `data/four.json`: `{`, `    "show": [`, `        "left_out",` (line 3), `        "funnel"`, `    ]`, `}`: four spaces
- `data/tabs.json`: `{`, `\t"omit": [`, `\t\t"left_out",` (line 3), `\t\t"legend"`, `\t]`, `}`: tabs
- `src/lib.rs`: `mod math {`, `    pub fn add(a: i32, b: i32) -> i32 {` (line 2), `        a + b`, `    }`, `}`, an empty
  line, `fn main() {`, `    let total = math::add(1, 2);` (line 8), `}`

## E151.1 Insert a line in files of different indentation: the dry run

One template adds `"more_lines",` under `"left_out",` in all three files, indented like the matched line with spaces
or tabs.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has '\"left_out\",' @item", "replace": {"@item": "{@item}\n{indent}\"more_lines\","}, "dry_run": true}
```
```text response
dry run  would replace: 3  matches: 3

data/
four.json  matches: 1
-3:        "left_out",
+3:        "left_out",
+4:        "more_lines",
tabs.json  matches: 1
-3:		"left_out",
+3:		"left_out",
+4:		"more_lines",
two.json  matches: 1
-3:    "left_out",
+3:    "left_out",
+4:    "more_lines",
```

## E151.2 Insert a line in files of different indentation: the real call

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

1. replace

```json request replace
{"query": "text has '\"left_out\",' @item", "replace": {"@item": "{@item}\n{indent}\"more_lines\","}, "expect_matches": 3}
```
```text response
replaced: 3  matches: 3

data/
four.json  matches: 1
-3:        "left_out",
+3:        "left_out",
+4:        "more_lines",
tabs.json  matches: 1
-3:		"left_out",
+3:		"left_out",
+4:		"more_lines",
two.json  matches: 1
-3:    "left_out",
+3:    "left_out",
+4:    "more_lines",
```

2. the files after it

```json request
{"query": "text has 'more_lines'", "output": {"sort": "path asc", "highlight": "none", "dedent": false, "context": 1}}
```
```text response
files: 3  lines: 3  result: r1

data/
four.json  lines: 1
3-        "left_out",
4:        "more_lines",
5-        "funnel"
tabs.json  lines: 1
3-		"left_out",
4:		"more_lines",
5-		"legend"
two.json  lines: 1
3-    "left_out",
4:    "more_lines",
5-    "warnings"
```

## E151.3 A match in the middle of a line

`{indent}` is the indentation of the whole line, not the text before the match.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'math::add(1, 2);' @call", "replace": {"@call": "{@call}\n{indent}println!(\"{{total}}\");"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

src/
lib.rs  matches: 1
-8:    let total = math::add(1, 2);
+8:    let total = math::add(1, 2);
+9:    println!("{total}");
```

## E151.4 A match which takes the indentation

The regex takes `^\s*` into the match; `{indent}` writes it back, here before a new line inserted above.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'tabs.json' and line(text =~ `^\\s*\"left_out\",`) @item", "replace": {"@item": "{indent}\"more_lines\",\n{indent}\"left_out\","}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

data/
tabs.json  matches: 1
-3:		"left_out",
+3:		"more_lines",
+4:		"left_out",
```

## E151.5 A line break without `{indent}`

A line break of the template is not followed by the indentation on its own: the new line starts at the first column.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'two.json' and text has '\"left_out\",' @item", "replace": {"@item": "{@item}\n\"more_lines\","}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

data/
two.json  matches: 1
-3:    "left_out",
+3:    "left_out",
+4:"more_lines",
```

## E151.6 A multiline match

The match of `add` covers lines 2-4; `{indent}` is the indentation of its first line, four spaces, not the eight of
line 3.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text =~ `pub fn add[^}]*\\}` /m @add", "replace": {"@add": "{@add}\n\n{indent}pub fn sub(a: i32, b: i32) -> i32 {{\n{indent}    a - b\n{indent}}}"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

src/
lib.rs  matches: 1
-2:    pub fn add(a: i32, b: i32) -> i32 {
-3:        a + b
-4:    }
+2:    pub fn add(a: i32, b: i32) -> i32 {
+3:        a + b
+4:    }
+5:
+6:    pub fn sub(a: i32, b: i32) -> i32 {
+7:        a - b
+8:    }
```

## E151.7 An unknown placeholder: the hint lists `{indent}`

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'left_out' @item", "replace": {"@item": "{indnet}{@item}"}}
```
```text error
error[invalid_template]: unknown placeholder 'indnet'
  {indnet}{@item}
  ^^^^^^^^
  hint: the placeholders of '@item' are {@item}, {indent}
```

## E151.8 An empty placeholder with the groups of a regex

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text =~ `fn (\\w+)\\((?<args>[^)]*)\\)` @def", "replace": {"@def": "{}"}}
```
```text error
error[invalid_template]: empty placeholder
  {}
  ^^
  hint: the placeholders of '@def' are {@def}, {@def.1}, {@def.2}, {@def.args}, {indent}
```

## E151.9 A reference to another group: the hint lists only the group

`{indent}` is not a group, so the hint of a wrong group reference lists only the placeholders of the group.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'left_out' @item", "replace": {"@item": "{@other}"}}
```
```text error
error[invalid_template]: '{@other}' is not '@item' or its sub-group
  {@other}
  ^^^^^^^^
  hint: the placeholders of '@item' are {@item}
```

## E151.10 `/n` leaves the indentation out

`/n` writes the new text in the naming style of the match, an identifier without spaces or tabs: `{indent}` is empty.

Fixture: `template-indent`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'tabs.json' and text has 'left out' /n @item", "replace": {"@item": "{indent}more lines"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

data/
tabs.json  matches: 1
-3:		"left_out",
+3:		"more_lines",
```
