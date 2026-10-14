# Examples for task 133

Spec: `1`

`output.file` writes the response to a file under `server.output_dir` and returns a summary with its path
(OUTPUT.md#output-to-a-file). The examples run with `server.output_dir` in a temporary directory of the test,
`<output_dir>`; a `text file` / `json file` block is the content of a file of `<output_dir>` after the request before
it.

## Fixtures

### output-file

```text tree
notes.md           2026-10-01 10:00
src/a.rs           2026-10-01 10:00
src/b.rs           2026-10-01 10:00
```

File contents are in `tests/fixtures/output-file/`: `notes.md` `# Notes`, `TODO write the docs`; `src/a.rs` a function
with two `TODO` comments, `// TODO split this function into parse and check` and `// TODO rename x`; `src/b.rs`
`// TODO remove`, `fn b() {}`.

## E133.1 A text response written to a named file

Fixture: `output-file`

The file holds the response as it would be returned; the tool returns its header and the path.

```json request
{"query": "text has 'TODO'", "output": {"file": "todos.txt"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
written: <output_dir>/todos.txt  lines: 16  bytes: 337
```
```text file todos.txt
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ remove
./
notes.md  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ write the docs
src/
a.rs  [#1]  lines: 2  matches: 2
⇥4
2:// ⟦#1:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#1:TODO⟧ rename x
```

## E133.2 `true` names the file by the result id; JSON

Fixture: `output-file`

The summary has the fields of the header and `file`, `lines`, `bytes`.

```json request
{"query": "text has 'TODO'", "output": {"file": true, "format": "json"}}
```
```json response
{"version":1,"exists":true,"result":"r1","counts":{"files":3,"lines":4,"matches":4},"file":"<output_dir>/r1.json","lines":1,"bytes":808}
```
```json file r1.json
{"version":1,"exists":true,"result":"r1","counts":{"files":3,"lines":4,"matches":4},"legend":{"#1":"text has 'TODO'"},"files":[{"path":"src/b.rs","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"lines":[{"n":1,"text":"// TODO remove","fragments":[{"col":[4,8],"text":"TODO","ids":["#1"]}]}]}]},{"path":"notes.md","lines":1,"matches":1,"conditions":["#1"],"blocks":[{"lines":[{"n":2,"text":"TODO write the docs","fragments":[{"col":[1,5],"text":"TODO","ids":["#1"]}]}]}]},{"path":"src/a.rs","lines":2,"matches":2,"conditions":["#1"],"blocks":[{"indent":4,"lines":[{"n":2,"text":"// TODO split this function into parse and check","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]},{"indent":4,"lines":[{"n":4,"text":"// TODO rename x","fragments":[{"col":[8,12],"text":"TODO","ids":["#1"]}]}]}]}]}
```

## E133.3 `true` without a result id

Fixture: `output-file`

`detail = exists` has no result id: the files are `out-1.txt`, `out-2.txt`, ...; a response with a result id is
named by it.

1. exists

```json request
{"query": "text has 'TODO'", "output": {"file": true, "detail": "exists"}}
```
```text response
exists: yes
written: <output_dir>/out-1.txt  lines: 1  bytes: 12
```
```text file out-1.txt
exists: yes
```

2. exists again

```json request
{"query": "text has 'TODO'", "output": {"file": true, "detail": "exists"}}
```
```text response
exists: yes
written: <output_dir>/out-2.txt  lines: 1  bytes: 12
```

3. count

```json request
{"query": "text has 'TODO'", "output": {"file": true, "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
written: <output_dir>/r1.txt  lines: 1  bytes: 43
```
```text file r1.txt
files: 3  lines: 4  matches: 4  result: r1
```

## E133.4 The limits of the configuration are for the context, not for a file

Fixture: `output-file`

`max_output = 100` of the session cuts the response in the context, not the one in a file; the same value given in the
request cuts the file too, with a cursor to the next page, which goes to another file.

1. the session limit

```json request update_config
{"set": "output.max_output = 100"}
```
```text response
[output]
max_output = 100    # was 40000
```

2. in the context

```json request
{"query": "text has 'TODO'"}
```
```text response
files: 3  lines: 4  matches: 4  result: r1  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiMWFkODBmOTAiLCJmIjoyLCJ0IjpbMyw0LDRdfQ
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ remove
./
notes.md  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ write the docs
src/
a.rs  [#1]  lines: 2  matches: 2
```

3. in a file

```json request
{"query": "text has 'TODO'", "output": {"file": "all.txt"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r2
written: <output_dir>/all.txt  lines: 16  bytes: 337
```
```text file all.txt
files: 3  lines: 4  matches: 4  result: r2
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ remove
./
notes.md  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ write the docs
src/
a.rs  [#1]  lines: 2  matches: 2
⇥4
2:// ⟦#1:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#1:TODO⟧ rename x
```

4. the limit in the request

```json request
{"query": "text has 'TODO'", "output": {"max_output": 100, "file": "page1.txt"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r3  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYWEwODc3OGEiLCJmIjoyLCJ0IjpbMyw0LDRdfQ
written: <output_dir>/page1.txt  lines: 11  bytes: 334
```
```text file page1.txt
files: 3  lines: 4  matches: 4  result: r3  truncated: yes (max_output)  next cursor: eyJ2IjoxLCJoIjoiYWEwODc3OGEiLCJmIjoyLCJ0IjpbMyw0LDRdfQ
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ remove
./
notes.md  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ write the docs
src/
a.rs  [#1]  lines: 2  matches: 2
```

5. the next page

```json request
{"query": "text has 'TODO'", "output": {"max_output": 100, "file": "page2.txt", "cursor": "eyJ2IjoxLCJoIjoiYWEwODc3OGEiLCJmIjoyLCJ0IjpbMyw0LDRdfQ"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r4
written: <output_dir>/page2.txt  lines: 10  bytes: 202
```
```text file page2.txt
files: 3  lines: 4  matches: 4  result: r4
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
⇥4
2:// ⟦#1:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#1:TODO⟧ rename x
```

## E133.5 Directories of the path, an absolute path, a file replaced

Fixture: `output-file`

The directories of a relative path are created; an absolute path inside `<output_dir>` is written as its canonical
path; an existing file is replaced.

1. a relative path with directories

```json request
{"query": "text has 'TODO'", "output": {"file": "reports/2026/todo.txt", "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
written: <output_dir>/reports/2026/todo.txt  lines: 1  bytes: 43
```
```text file reports/2026/todo.txt
files: 3  lines: 4  matches: 4  result: r1
```

2. an absolute path

```json request
{"query": "text has 'TODO'", "output": {"file": "<output_dir>/reports/../count.txt", "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r2
written: <output_dir>/count.txt  lines: 1  bytes: 43
```
```text file count.txt
files: 3  lines: 4  matches: 4  result: r2
```

3. the same file again

```json request
{"query": "name = 'notes.md' and text has 'TODO'", "output": {"file": "count.txt", "detail": "count"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r3
written: <output_dir>/count.txt  lines: 1  bytes: 43
```
```text file count.txt
files: 1  lines: 1  matches: 1  result: r3
```

## E133.6 Paths outside of the output directory and wrong values

Fixture: `output-file`

The path is checked before the search; `false` is no file.

1. `..` out of it

```json request
{"query": "text has 'TODO'", "output": {"file": "reports/../../x.txt"}}
```
```text error
error[output_file_denied]: the output file 'reports/../../x.txt' is outside of the output directory
  hint: files are written to <output_dir>
```

2. an absolute path elsewhere

```json request
{"query": "text has 'TODO'", "output": {"file": "<fixture>/x.txt"}}
```
```text error
error[output_file_denied]: the output file '<fixture>/x.txt' is outside of the output directory
  hint: files are written to <output_dir>
```

3. a number

```json request
{"query": "text has 'TODO'", "output": {"file": 1}}
```
```text error
error[invalid_option]: 'output.file' must be a path or true
```

4. an empty path

```json request
{"query": "text has 'TODO'", "output": {"file": ""}}
```
```text error
error[invalid_option]: 'output.file' must be a path or true
```

5. `false`

```json request
{"query": "text has 'TODO'", "output": {"file": false, "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
```

## E133.7 A file which can't be written

Fixture: `output-file`

Platform: `linux`

`a.txt` is a file, so it can't be the directory of `a.txt/b.txt`; the error of the system is the hint.

1. a.txt

```json request
{"query": "text has 'TODO'", "output": {"file": "a.txt", "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
written: <output_dir>/a.txt  lines: 1  bytes: 43
```

2. a.txt/b.txt

```json request
{"query": "text has 'TODO'", "output": {"file": "a.txt/b.txt", "detail": "count"}}
```
```text error
error[output_file_failed]: the output file '<output_dir>/a.txt/b.txt' can't be written
  hint: File exists (os error 17)
```

## E133.8 `output.file` and the output directory are not set by `update_config`

Fixture: `output-file`

1. output.file

```json request update_config
{"set": "output.file = \"a.txt\""}
```
```text error
error[invalid_value]: invalid value for 'output.file'
  hint: output.file is set per request
```

2. server.output_dir

```json request update_config
{"set": "server.output_dir = \"/\""}
```
```text error
error[read_only]: 'server.output_dir' is read-only
  hint: output_dir is set only at start
```

3. server.max_output_file_size

```json request update_config
{"reset": ["server.max_output_file_size"]}
```
```text error
error[read_only]: 'server.max_output_file_size' is read-only
  hint: max_output_file_size is set only at start
```

## E133.9 A batch with a file of one query

Fixture: `output-file`

The section of the query with its own file is its summary; the file holds the response of the query alone.

```json request
{"queries": [{"label": "rs", "query": "name ~ '*.rs' and text has 'TODO'", "output": {"file": "rs.txt"}}, {"label": "md", "query": "name ~ '*.md' and text has 'TODO'"}]}
```
```text response
== rs ==  files: 2  lines: 3  matches: 3  result: r1
written: <output_dir>/rs.txt  lines: 13  bytes: 284
== md ==  files: 1  lines: 1  matches: 1  result: r2
#1 name ~ '*.md'
#2 text has 'TODO'

./
notes.md  [#1 #2]  lines: 1  matches: 1
2:⟦#2:TODO⟧ write the docs
```
```text file rs.txt
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 text has 'TODO'

src/
b.rs  [#1 #2]  lines: 1  matches: 1
1:// ⟦#2:TODO⟧ remove
a.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:// ⟦#2:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#2:TODO⟧ rename x
```

## E133.10 A batch in one file

Fixture: `output-file`

`output.file` of the request writes the batch response; the summary has the first line of every section. A batch has
no result id of its own: `true` names its file `out-<n>`.

1. text

```json request
{"output": {"file": "all.txt"}, "queries": [{"label": "rs", "query": "name ~ '*.rs' and text has 'TODO'"}, {"label": "md", "query": "name ~ '*.md' and text has 'TODO'"}]}
```
```text response
== rs ==  files: 2  lines: 3  matches: 3  result: r1
== md ==  files: 1  lines: 1  matches: 1  result: r2
written: <output_dir>/all.txt  lines: 20  bytes: 458
```
```text file all.txt
== rs ==  files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 text has 'TODO'

src/
b.rs  [#1 #2]  lines: 1  matches: 1
1:// ⟦#2:TODO⟧ remove
a.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:// ⟦#2:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#2:TODO⟧ rename x
== md ==  files: 1  lines: 1  matches: 1  result: r2
#1 name ~ '*.md'
#2 text has 'TODO'

./
notes.md  [#1 #2]  lines: 1  matches: 1
2:⟦#2:TODO⟧ write the docs
```

2. JSON

```json request
{"output": {"file": true, "format": "json"}, "queries": [{"label": "rs", "query": "name ~ '*.rs' and text has 'TODO'"}, {"label": "md", "query": "name ~ '*.md' and text has 'TODO'"}]}
```
```json response
{"version":1,"queries":[{"label":"rs","exists":true,"result":"r3","counts":{"files":2,"lines":3,"matches":3}},{"label":"md","exists":true,"result":"r4","counts":{"files":1,"lines":1,"matches":1}}],"file":"<output_dir>/out-1.json","lines":1,"bytes":1022}
```
```json file out-1.json
{"version":1,"queries":[{"label":"rs","exists":true,"result":"r3","counts":{"files":2,"lines":3,"matches":3},"legend":{"#1":"name ~ '*.rs'","#2":"text has 'TODO'"},"files":[{"path":"src/b.rs","lines":1,"matches":1,"conditions":["#1","#2"],"blocks":[{"lines":[{"n":1,"text":"// TODO remove","fragments":[{"col":[4,8],"text":"TODO","ids":["#2"]}]}]}]},{"path":"src/a.rs","lines":2,"matches":2,"conditions":["#1","#2"],"blocks":[{"indent":4,"lines":[{"n":2,"text":"// TODO split this function into parse and check","fragments":[{"col":[8,12],"text":"TODO","ids":["#2"]}]}]},{"indent":4,"lines":[{"n":4,"text":"// TODO rename x","fragments":[{"col":[8,12],"text":"TODO","ids":["#2"]}]}]}]}]},{"label":"md","exists":true,"result":"r4","counts":{"files":1,"lines":1,"matches":1},"legend":{"#1":"name ~ '*.md'","#2":"text has 'TODO'"},"files":[{"path":"notes.md","lines":1,"matches":1,"conditions":["#1","#2"],"blocks":[{"lines":[{"n":2,"text":"TODO write the docs","fragments":[{"col":[1,5],"text":"TODO","ids":["#2"]}]}]}]}]}]}
```

## E133.11 `--output-dir`

Fixture: `output-file`

A relative directory is resolved against the start directory; `get_config` shows it with the source `cli`.

```text command
easy-peasy-mcp --output-dir out
```

1. get_config

```json request get_config
{"section": "server", "sources": true}
```
```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
allow_update = true    # default
allow_write = false    # default
result_cache = 50    # default
output_dir = '<fixture>/out'    # cli
max_output_file_size = "100M"    # default
```

2. a file

```json request
{"query": "text has 'TODO'", "output": {"file": "x.txt", "detail": "count"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1
written: <fixture>/out/x.txt  lines: 1  bytes: 43
```

## E133.12 `output_dir = ""` turns the option off

Fixture: `output-file`

```toml config.toml
[server]
output_dir = ""
```
```text command
easy-peasy-mcp --config config.toml
```

1. get_config

```json request get_config
{"section": "server"}
```
```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']
default_dirs = ['.']
timeout = "30s"
threads = 0
allow_update = true
allow_write = false
result_cache = 50
output_dir = ''
max_output_file_size = "100M"
```

2. a file

```json request
{"query": "text has 'TODO'", "output": {"file": "x.txt"}}
```
```text error
error[output_file_disabled]: output to a file is disabled
  hint: the configuration has server.output_dir = ""
```

## E133.13 An invalid `server.output_dir`

Fixture: `output-file`

```toml config.toml
[server]
output_dir = 1
```
```text command
easy-peasy-mcp --config config.toml
```
```text stderr exit 1
easy-peasy-mcp: config.toml:2: invalid value for 'server.output_dir': a path is expected
```

## E133.14 An invalid `server.max_output_file_size`

Fixture: `output-file`

```toml config.toml
[server]
max_output_file_size = "big"
```
```text command
easy-peasy-mcp --config config.toml
```
```text stderr exit 1
easy-peasy-mcp: config.toml:2: invalid value for 'server.max_output_file_size': a size like "100M" is expected (units B, K, M, G)
```

## E133.15 `max_output_file_size` cuts a response in a file

Fixture: `output-file`

The cap of 100 characters cuts the listing like `max_output = 100`; the warning is in the file and in the summary, the
cursor goes on in another file. The default `server.output_dir` is `<output_dir>`.

```toml config.toml
[server]
max_output_file_size = "100B"
```
```text command
easy-peasy-mcp --config config.toml
```

1. the first page

```json request
{"query": "text has 'TODO'", "output": {"file": "page1.txt"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r1  truncated: yes (max_output_file_size)  next cursor: eyJ2IjoxLCJoIjoiZjYyOTJhOTkiLCJmIjoyLCJ0IjpbMyw0LDRdfQ
written: <output_dir>/page1.txt  lines: 13  bytes: 376

warning: output_file_truncated
```
```text file page1.txt
files: 3  lines: 4  matches: 4  result: r1  truncated: yes (max_output_file_size)  next cursor: eyJ2IjoxLCJoIjoiZjYyOTJhOTkiLCJmIjoyLCJ0IjpbMyw0LDRdfQ
#1 text has 'TODO'

src/
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ remove
./
notes.md  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ write the docs
src/
a.rs  [#1]  lines: 2  matches: 2

warning: output_file_truncated
```

2. the next page

```json request
{"query": "text has 'TODO'", "output": {"file": "page2.txt", "cursor": "eyJ2IjoxLCJoIjoiZjYyOTJhOTkiLCJmIjoyLCJ0IjpbMyw0LDRdfQ"}}
```
```text response
files: 3  lines: 4  matches: 4  result: r2
written: <output_dir>/page2.txt  lines: 10  bytes: 202
```
```text file page2.txt
files: 3  lines: 4  matches: 4  result: r2
#1 text has 'TODO'

src/
a.rs  [#1]  lines: 2  matches: 2
⇥4
2:// ⟦#1:TODO⟧ split this function into parse and check
--
⇥4
4:// ⟦#1:TODO⟧ rename x
```
