# Examples for task 47

Examples with a command line are startup examples (AGENTS.md). Hidden means a name starting with `.` here; the
hidden flags of macOS and Windows come with tasks 119–120.

## Fixtures

### hidden

```text tree
.config/app.toml           2026-10-01 10:00
.env                       2026-10-01 10:00
visible.txt                2026-10-01 10:00
```

File contents are in `tests/fixtures/hidden/`. Times are UTC.

## E47.1 Hidden entries are skipped by default

Fixture: `hidden`

1. `find`

```json request
{"query": "type = file"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = file

./
visible.txt  [#1]
```

2. `get_config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 0
# max_depth =  (not set)
follow_symlinks = false
hidden = false
ignore_files = ['.gitignore', '.ignore', '.aiignore']
git_excludes = true
ignore_paths = []
exclude = []
path_case = "native"
```

## E47.2 Hidden files in the request

Fixture: `hidden`

```json request
{"query": "name ~ '.*'", "traversal": {"hidden": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '.*'

./
.env  [#1]
```

## E47.3 Files in a hidden directory

Fixture: `hidden`

```json request
{"query": "name = 'app.toml'", "traversal": {"hidden": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'app.toml'

.config/
app.toml  [#1]
```

## E47.4 Hidden entries in a session

Fixture: `hidden`

1. `update_config`

```json request update_config
{"set": "traversal.hidden = true"}
```

```text response
[traversal]
hidden = true    # was false
```

2. `find`

```json request
{"query": "name ~ '.*'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '.*'

./
.env  [#1]
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = true    # update
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
```

## E47.5 An invalid value in a request

Fixture: `hidden`

```json request
{"traversal": {"hidden": "yes"}}
```

```text error
error[invalid_option]: 'traversal.hidden' must be true or false
```

## E47.6 An invalid value for `update_config`

Fixture: `hidden`

```json request update_config
{"set": "traversal.hidden = \"true\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.hidden'
  hint: true or false is expected
```

## E47.7 From the config file

Fixture: `hidden`

```toml conf/h.toml
[traversal]
hidden = true
```

```text command
easy-peasy-mcp --config conf/h.toml
```

1. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = true    # file: conf/h.toml
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
```

2. `find`

```json request
{"query": "name ~ '.*'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '.*'

./
.env  [#1]
```

## E47.8 An invalid value in the config file

Fixture: `hidden`

```toml conf/h.toml
[traversal]
hidden = 0
```

```text command
easy-peasy-mcp --config conf/h.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/h.toml:2: invalid value for 'traversal.hidden': true or false is expected
```
