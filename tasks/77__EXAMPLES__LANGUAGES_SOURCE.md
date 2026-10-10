# Examples for task 77

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### sourcekind

```text tree
data.json                  2026-10-01 10:00
deploy/app.hcl             2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/sourcekind/`: one line each. Times are UTC.

## E77.1 JSON is a data format of the built-in table

Fixture: `sourcekind`

```json request
{"query": "kind = source"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 kind = source

src/
main.rs  [#1]
```

## E77.2 A new language is code unless it says otherwise

Fixture: `sourcekind`

1. `find`

```json request
{"query": "kind = source", "languages": {"hcl": {"extensions": ["hcl"]}}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 kind = source

deploy/
app.hcl  [#1]
src/
main.rs  [#1]
```

2. `find`

Without a `config` or `doc` rule a file of a data format is `other`.

```json request
{"query": "lang = hcl and kind = other", "languages": {"hcl": {"extensions": ["hcl"], "source": false}}}
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 lang = hcl
#2 kind = other

deploy/
app.hcl  [#1 #2]
```

## E77.3 A built-in data format as code in a request

Fixture: `sourcekind`

```json request
{"query": "kind = source", "languages": {"json": {"extensions": ["json"], "source": true}}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 kind = source

./
data.json  [#1]
src/
main.rs  [#1]
```

## E77.4 A data format for the session

Fixture: `sourcekind`

1. `update_config`

```json request update_config
{"set": "[languages.hcl]\nextensions = [\"hcl\"]\nsource = false"}
```

```text response
[languages.hcl]
extensions = ['hcl']    # was (not set)
names = []    # was (not set)
shebangs = []    # was (not set)
line_comments = []    # was (not set)
block_comments = []    # was (not set)
nested_block_comments = false    # was (not set)
strings = []    # was (not set)
source = false    # was (not set)
```

2. `find`

```json request
{"query": "lang = hcl and kind = other"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = hcl
#2 kind = other

deploy/
app.hcl  [#1 #2]
```

## E77.5 An invalid value in a request

Fixture: `sourcekind`

```json request
{"languages": {"hcl": {"source": "no"}}}
```

```text error
error[invalid_option]: 'languages.hcl.source' must be true or false
```

## E77.6 An invalid value for `update_config`

Fixture: `sourcekind`

```json request update_config
{"set": "languages.hcl.source = 0"}
```

```text error
error[invalid_value]: invalid value for 'languages.hcl.source'
  hint: true or false is expected
```

## E77.7 From the config file

Fixture: `sourcekind`

```toml conf/l.toml
[languages.hcl]
extensions = ["hcl"]
source = false
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "lang = hcl and kind = other"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = hcl
#2 kind = other

deploy/
app.hcl  [#1 #2]
```

## E77.8 An invalid value in the config file

Fixture: `sourcekind`

```toml conf/l.toml
[languages.hcl]
source = "no"
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'languages.hcl.source': true or false is expected
```
