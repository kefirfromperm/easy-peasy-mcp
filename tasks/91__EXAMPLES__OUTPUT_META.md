# Examples for task 91

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### meta

```text tree
README.md                  2026-10-02 09:30
data/blob.bin              2026-10-01 10:00
docs/                      2026-10-03 12:00
docs/guide.txt             2026-10-03 12:00
src/big.rs                 2026-10-08 14:03
src/small.rs               2026-10-07 18:22
```

```text setup
file data/blob.bin 1572864
file docs/guide.txt 3
file src/big.rs 2150
file src/small.rs 512
```

`README.md` is `# Meta` (7 bytes, in `tests/fixtures/meta/`); the other files are created by the test setup with the
given number of bytes. Times are UTC.

## E91.1 Off by default

Fixture: `meta`

1. `find`

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src/
big.rs  [#1]
small.rs  [#1]
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
detail = "lines"
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 100
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E91.2 Type, size, date, language and kind

Fixture: `meta`

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files", "meta": true}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src/
big.rs  [#1]  file 2.1K 2026-10-08 14:03 rust source 0.93
small.rs  [#1]  file 512B 2026-10-07 18:22 rust source 0.91
```

## E91.3 In JSON

Fixture: `meta`

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files", "meta": true, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 2, "lines": 0, "matches": 0 },
  "legend": { "#1": "name ~ '*.rs'" },
  "files": [
    { "path": "src/big.rs", "type": "file", "size": 2150, "lang": "rust", "kind": "source",
      "modified": "2026-10-08T14:03:00+00:00", "score": 0.93, "conditions": ["#1"] },
    { "path": "src/small.rs", "type": "file", "size": 512, "lang": "rust", "kind": "source",
      "modified": "2026-10-07T18:22:00+00:00", "score": 0.91, "conditions": ["#1"] }
  ]
}
```

## E91.4 Other sizes, languages and kinds

Fixture: `meta`

```json request
{"query": "name ~ '*.{md,bin}'", "output": {"meta": true}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.{md,bin}'

./
README.md  [#1]  file 7B 2026-10-02 09:30 markdown doc 0.87
data/
blob.bin  [#1]  file 1.5M 2026-10-01 10:00 unknown other 0.71
```

## E91.5 A directory

A directory has only its type and date. With the default `sort = relevance` every file shows its score.

Fixture: `meta`

```json request
{"query": "type = dir and name = 'docs'", "output": {"meta": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = dir
#2 name = 'docs'

./
docs/  [#1 #2]  dir 2026-10-03 12:00 0.80
```

```json request
{"query": "type = dir and name = 'docs'", "output": {"meta": true, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "legend": { "#1": "type = dir", "#2": "name = 'docs'" },
  "files": [
    { "path": "docs/", "type": "dir", "modified": "2026-10-03T12:00:00+00:00", "score": 0.8, "conditions": ["#1", "#2"] }
  ]
}
```

## E91.6 After the counts

Fixture: `meta`

```json request
{"query": "text has 'Meta'", "output": {"meta": true}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'Meta'

./
README.md  [#1]  lines: 1  matches: 1  file 7B 2026-10-02 09:30 markdown doc 0.58
1:# ⟦#1:Meta⟧
```

## E91.7 For the session

Fixture: `meta`

1. `update_config`

```json request update_config
{"set": "output.meta = true"}
```

```text response
[output]
meta = true    # was false
```

2. `find`

```json request
{"query": "name = 'small.rs'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'small.rs'

src/
small.rs  [#1]  file 512B 2026-10-07 18:22 rust source 0.93
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
meta = true    # update
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E91.8 An invalid value in a request

Fixture: `meta`

```json request
{"query": "name = 'small.rs'", "output": {"meta": 1}}
```

```text error
error[invalid_option]: 'output.meta' must be true or false
```

## E91.9 An invalid value for `update_config`

Fixture: `meta`

```json request update_config
{"set": "output.meta = \"yes\""}
```

```text error
error[invalid_value]: invalid value for 'output.meta'
  hint: true or false is expected
```

## E91.10 From the config file

The dates are in the time zone of the server.

Fixture: `meta`

```toml conf/o.toml
[output]
meta = true
```

```text env
TZ=UTC
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "name = 'small.rs'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'small.rs'

src/
small.rs  [#1]  file 512B 2026-10-07 18:22 rust source 0.93
```

## E91.11 An invalid value in the config file

Fixture: `meta`

```toml conf/o.toml
[output]
meta = "yes"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.meta': true or false is expected
```
