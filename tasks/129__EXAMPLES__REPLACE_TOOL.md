# Examples for task 129

The `replace` tool ([TOOLS.md](../TOOLS.md#replace)). It is a write tool: the selection, `expect`, `dry_run`, the
skipped entries and the response follow the shared rules of task 125, so these examples are about what is replaced and
how a file is written. They are startup examples with `--allow-write`, except the one without it; a step with `find`
shows the file after the replacement.

## Fixtures

### replace

```text tree
Cargo.toml           2026-10-01 10:00
data/config.json     2026-10-01 10:00
notes/crlf.txt       2026-10-01 10:00
notes/latin1.txt     2026-10-01 10:00
notes/utf16.txt      2026-10-01 10:00
src/lib.rs           2026-10-01 10:00
src/main.rs          2026-10-01 10:00
src/naming.rs        2026-10-01 10:00
src/parse.rs         2026-10-01 10:00
src/todo.rs          2026-10-01 10:00
```

In `tests/fixtures/replace/`:

- `src/main.rs`: `use serde_json::Value;` (line 1), `    let apple = serde_json::json!({"fruit": "apple"});`
  (line 4), `    println!("{}", serde_json::to_string(&apple).unwrap());` (line 5)
- `src/lib.rs`: `pub fn to_json(v: &serde_json::Value) -> String {` (line 1), no `apple`
- `Cargo.toml`: `[dependencies]`, `serde_json = "1"`
- `src/parse.rs`: `fn parse_query(text: &str) -> Query {` (line 1), `fn parse_value(text: &str) -> Value {` (line 5)
- `src/naming.rs`: `fn parseQuery() {}`, `const PARSE_QUERY: u8 = 1;`, `struct ParseQuery;`, `mod parse_query {}`,
  `// parse-query and parse query`
- `src/todo.rs`: `// TODO: remove unwrap`, `let a = x.unwrap();`, `let b = y.unwrap(); // TODO`, `let c = z.unwrap();`
- `data/config.json`: `{"role": "admin"}`, then the same object as a string: `{"payload": "{\"role\": \"admin\"}"}`
- `notes/crlf.txt`: `one`, `two`, `three` with `\r\n` line breaks, 17 bytes
- `notes/latin1.txt`: `café au lait` and `\n` in `windows-1252`, 13 bytes
- `notes/utf16.txt`: a UTF-16LE BOM and `hello world\n`, 26 bytes

## E129.1 `--allow-write` lists `replace`

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}, {"name": "delete"}, {"name": "rename"}, {"name": "copy"}, {"name": "replace", "annotations": {"readOnlyHint": false, "destructiveHint": true, "idempotentHint": false, "openWorldHint": false}}]}}
```

## E129.2 Without `allow_write`

Fixture: `replace`

```json request replace
{"query": "text has 'serde_json' @old", "replace": {"@old": "simd_json"}}
```
```text error
error[write_disabled]: write tools are disabled
  hint: the configuration has server.allow_write = false
```

## E129.3 Only the group is replaced

The example of the task: `serde_json` is replaced only in the Rust files which also have `apple`; `apple` selects the
file and stays. The matches of lines 4 and 5 are one run of lines.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. replace

```json request replace
{"query": "lang = rust and (text has 'serde_json' /w) @old and text has 'apple'", "replace": {"@old": "simd_json"}}
```
```text response
replaced: 1  matches: 3

src/
main.rs  matches: 3
-1:use serde_json::Value;
+1:use simd_json::Value;
-4:    let apple = serde_json::json!({"fruit": "apple"});
-5:    println!("{}", serde_json::to_string(&apple).unwrap());
+4:    let apple = simd_json::json!({"fruit": "apple"});
+5:    println!("{}", simd_json::to_string(&apple).unwrap());
```

2. the files after it: `lib.rs` and `Cargo.toml` keep `serde_json`

```json request
{"query": "text has 'json'", "output": {"sort": "path asc", "highlight": "none", "dedent": false}}
```
```text response
files: 3  lines: 6  matches: 6  result: r1
#1 text has 'json'

./
Cargo.toml  lines: 1  matches: 1
2:serde_json = "1"
src/
lib.rs  lines: 1  matches: 2
1:pub fn to_json(v: &serde_json::Value) -> String {
main.rs  lines: 3  matches: 3
1:use simd_json::Value;
--
4:    let apple = simd_json::json!({"fruit": "apple"});
5:    println!("{}", simd_json::to_string(&apple).unwrap());
```

## E129.4 A regex with a named group in the template

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'parse.rs' and text =~ `fn (?<fname>\\w+)\\(` @def", "replace": {"@def": "fn {@def.fname}_v2("}}
```
```text response
replaced: 1  matches: 2

src/
parse.rs  matches: 2
-1:fn parse_query(text: &str) -> Query {
+1:fn parse_query_v2(text: &str) -> Query {
-5:fn parse_value(text: &str) -> Value {
+5:fn parse_value_v2(text: &str) -> Value {
```

## E129.5 `/n` keeps the naming style of every match

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. replace

```json request replace
{"query": "name = 'naming.rs' and text has 'parse query' /n @old", "replace": {"@old": "build request"}}
```
```text response
replaced: 1  matches: 6

src/
naming.rs  matches: 6
-1:fn parseQuery() {}
-2:const PARSE_QUERY: u8 = 1;
-3:struct ParseQuery;
-4:mod parse_query {}
-5:// parse-query and parse query
+1:fn buildRequest() {}
+2:const BUILD_REQUEST: u8 = 1;
+3:struct BuildRequest;
+4:mod build_request {}
+5:// build-request and build request
```

2. the file after it

```json request
{"query": "name = 'naming.rs' and text has 'build request' /n", "output": {"highlight": "none"}}
```
```text response
files: 1  lines: 5  matches: 6  result: r1
#1 name = 'naming.rs'
#2 text has 'build request' /n

src/
naming.rs  [#1 #2]  lines: 5  matches: 6
1:fn buildRequest() {}
2:const BUILD_REQUEST: u8 = 1;
3:struct BuildRequest;
4:mod build_request {}
5:// build-request and build request
```

## E129.6 `/i` writes the template as it is

The case of the matches is not adapted.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'naming.rs' and text has 'parsequery' /i @q", "replace": {"@q": "buildRequest"}}
```
```text response
replaced: 1  matches: 2

src/
naming.rs  matches: 2
-1:fn parseQuery() {}
+1:fn buildRequest() {}
-3:struct ParseQuery;
+3:struct buildRequest;
```

## E129.7 `/n` turned off for a group

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'naming.rs' and text has 'parse query' /n @old", "replace": {"@old": {"template": "build_request", "naming": false}}, "dry_run": true}
```
```text response
replaced: 1  matches: 6  dry run

src/
naming.rs  matches: 6
-1:fn parseQuery() {}
-2:const PARSE_QUERY: u8 = 1;
-3:struct ParseQuery;
-4:mod parse_query {}
-5:// parse-query and parse query
+1:fn build_request() {}
+2:const build_request: u8 = 1;
+3:struct build_request;
+4:mod build_request {}
+5:// build_request and build_request
```

## E129.8 Inside `line( ... )` only the matched lines

`unwrap` is replaced only in the lines which also have `TODO`: lines 1 and 3.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'todo.rs' and line(text has 'unwrap' @u and text has 'TODO')", "replace": {"@u": "expect"}}
```
```text response
replaced: 1  matches: 2

src/
todo.rs  matches: 2
-1:// TODO: remove unwrap
+1:// TODO: remove expect
-3:let b = y.unwrap(); // TODO
+3:let b = y.expect(); // TODO
```

## E129.9 `/c`: the comments stay

The `unwrap` of the comment in line 1 is not a match.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'todo.rs' and text has 'unwrap' /c @u", "replace": {"@u": "unwrap_or_default"}}
```
```text response
replaced: 1  matches: 3

src/
todo.rs  matches: 3
-2:let a = x.unwrap();
-3:let b = y.unwrap(); // TODO
-4:let c = z.unwrap();
+2:let a = x.unwrap_or_default();
+3:let b = y.unwrap_or_default(); // TODO
+4:let c = z.unwrap_or_default();
```

## E129.10 Two groups

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'Cargo.toml' and text has 'serde_json' @name and text has '\"1\"' @version", "replace": {"@name": "simd_json", "@version": "\"0.13\""}}
```
```text response
replaced: 1  matches: 2

./
Cargo.toml  matches: 2
-2:serde_json = "1"
+2:simd_json = "0.13"
```

## E129.11 Overlapping groups

`json` is inside `serde_json`: the file is skipped, nothing was done of the selection.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'main.rs' and text has 'serde_json' @a and text has 'json' @b", "replace": {"@a": "simd_json", "@b": "yaml"}}
```
```text error
replaced: 0  matches: 0  skipped: 1

warning: replace_conflict (1): src/main.rs
```

## E129.12 A template which refers to another group

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'serde_json' @old and text has 'apple' @fruit", "replace": {"@old": "{@fruit}_json"}}
```
```text error
error[invalid_template]: '{@fruit}' is not '@old' or its sub-group
  {@fruit}_json
  ^^^^^^^^
  hint: the placeholders of '@old' are {@old}
```

## E129.13 Groups which can't be replaced

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. an unknown group

```json request replace
{"query": "text has 'serde_json' @old", "replace": {"@new": "simd_json"}}
```
```text error
error[unknown_group]: unknown group '@new'
  @new
  ^^^^
  hint: the groups of the query: @old
```

2. a sub-group

```json request replace
{"query": "text =~ `fn (\\w+)` @def", "replace": {"@def.1": "f"}}
```
```text error
error[invalid_option]: '@def.1' is a sub-group; replace the whole group
  hint: e.g. {"@def": "...{@def.1}..."}
```

3. a group without text

```json request replace
{"query": "(name ~ '*.rs') @src and text has 'apple'", "replace": {"@src": "x"}}
```
```text error
error[invalid_option]: the group '@src' has no text condition
  hint: replace changes the text of files; rename changes their names
```

4. without `replace`

```json request replace
{"query": "text has 'apple'"}
```
```text error
error[invalid_option]: 'replace' is required
  hint: the groups of the query and their new text, e.g. {"@old": "new"}
```

## E129.14 The files of a result, one changed since

`find` keeps the result `r1`; `src/lib.rs` changes before the replacement and is skipped. The other files are searched
again with the query of the result.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. find

```json request
{"query": "text has 'serde_json' @old", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 3  lines: 5  matches: 5  result: r1
@old text has 'serde_json'

./
Cargo.toml  [@old]  lines: 1  matches: 1
src/
lib.rs  [@old]  lines: 1  matches: 1
main.rs  [@old]  lines: 3  matches: 3
```

```text setup
file src/lib.rs 10
```

2. replace in its files

```json request replace
{"result": "r1", "replace": {"@old": "simd_json"}}
```
```text response
replaced: 2  matches: 4  skipped: 1

./
Cargo.toml  matches: 1
-2:serde_json = "1"
+2:simd_json = "1"
src/
main.rs  matches: 3
-1:use serde_json::Value;
+1:use simd_json::Value;
-4:    let apple = serde_json::json!({"fruit": "apple"});
-5:    println!("{}", serde_json::to_string(&apple).unwrap());
+4:    let apple = simd_json::json!({"fruit": "apple"});
+5:    println!("{}", simd_json::to_string(&apple).unwrap());

warning: file_changed (1): src/lib.rs
```

## E129.15 A file selected by another branch is unchanged

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'to_json' @old or name = 'main.rs'", "replace": {"@old": "to_json_string"}}
```
```text response
replaced: 1  matches: 1  unchanged: 1

src/
lib.rs  matches: 1
-1:pub fn to_json(v: &serde_json::Value) -> String {
+1:pub fn to_json_string(v: &serde_json::Value) -> String {
main.rs  unchanged
```

## E129.16 An encoding which can't write the replacement

`windows-1252` has `è` but no Cyrillic letters.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. Cyrillic

```json request replace
{"query": "name = 'latin1.txt' and text has 'lait' @l", "traversal": {"encoding": "windows-1252"}, "replace": {"@l": "молоко"}}
```
```text error
replaced: 0  matches: 0  skipped: 1

warning: unencodable (1): notes/latin1.txt
```

2. `è`

```json request replace
{"query": "name = 'latin1.txt' and text has 'lait' @l", "traversal": {"encoding": "windows-1252"}, "replace": {"@l": "crème"}}
```
```text response
replaced: 1  matches: 1

notes/
latin1.txt  matches: 1
-1:café au lait
+1:café au crème
```

3. the file is still `windows-1252`: one byte for `è`

```json request
{"query": "name = 'latin1.txt' and size = 14 and text has 'crème'", "traversal": {"encoding": "windows-1252"}, "output": {"highlight": "none"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'latin1.txt'
#2 size = 14
#3 text has 'crème'

notes/
latin1.txt  [#1 #2 #3]  lines: 1  matches: 1  windows-1252
1:café au crème
```

## E129.17 A UTF-16 file with a BOM

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. replace

```json request replace
{"query": "text has 'world' @w", "replace": {"@w": "there"}}
```
```text response
replaced: 1  matches: 1

notes/
utf16.txt  matches: 1
-1:hello world
+1:hello there
```

2. the BOM and the encoding are kept: still 26 bytes

```json request
{"query": "size = 26 and text has 'hello there'", "output": {"highlight": "none"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1
#1 size = 26
#2 text has 'hello there'

notes/
utf16.txt  [#1 #2]  lines: 1  matches: 1  utf-16le
1:hello there
```

## E129.18 CRLF line breaks

The new line break of the template is written as `\r\n`, the line break of the file.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

1. replace

```json request replace
{"query": "name = 'crlf.txt' and text has 'two' @t", "replace": {"@t": "2\n2b"}}
```
```text response
replaced: 1  matches: 1

notes/
crlf.txt  matches: 1
-2:two
+2:2
+3:2b
```

2. `one\r\n2\r\n2b\r\nthree\r\n`: 19 bytes

```json request
{"query": "name = 'crlf.txt' and size = 19", "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'crlf.txt'
#2 size = 19

notes/
crlf.txt  [#1 #2]
```

## E129.19 `/e`: escaped like the match

On level 0 the template is written as it is; inside the string it is escaped once, like the match.

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has `\"role\": \"admin\"` /e(backslash, 0..1) @role", "replace": {"@role": "\"role\": \"user\""}}
```
```text response
replaced: 1  matches: 2

data/
config.json  matches: 2
-1:{"role": "admin"}
-2:{"payload": "{\"role\": \"admin\"}"}
+1:{"role": "user"}
+2:{"payload": "{\"role\": \"user\"}"}
```

## E129.20 `expect_matches`

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "lang = rust and (text has 'serde_json' /w) @old and text has 'apple'", "replace": {"@old": "simd_json"}, "expect_matches": 2}
```
```text error
error[count_mismatch]: the selection has 3 matches, expected 2
  hint: nothing was changed; check the selection with dry_run: true
```

## E129.21 `output.max_lines`

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "name = 'naming.rs' and text has 'parse query' /n @old", "replace": {"@old": "build request"}, "dry_run": true, "output": {"max_lines": 4}}
```
```text response
replaced: 1  matches: 6  dry run

src/
naming.rs  matches: 6
-1:fn parseQuery() {}
-2:const PARSE_QUERY: u8 = 1;
-3:struct ParseQuery;
-4:mod parse_query {}
… 6 more lines
```

## E129.22 The JSON format

Fixture: `replace`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'to_json' @old or name = 'main.rs'", "replace": {"@old": "to_json_string"}, "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 1,
  "replaced": 1,
  "matches": 1,
  "unchanged": 1,
  "dry_run": true,
  "files": [
    {
      "path": "src/lib.rs",
      "matches": 1,
      "changes": [
        {
          "line": 1,
          "old": ["pub fn to_json(v: &serde_json::Value) -> String {"],
          "new_line": 1,
          "new": ["pub fn to_json_string(v: &serde_json::Value) -> String {"]
        }
      ]
    },
    { "path": "src/main.rs", "unchanged": true }
  ]
}
```
