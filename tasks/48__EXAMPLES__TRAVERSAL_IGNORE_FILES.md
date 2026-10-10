# Examples for task 48

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### ignores

```text tree
.aiignore                  2026-10-01 10:00
.gitignore                 2026-10-01 10:00
app.log                    2026-10-01 10:00
keep.log                   2026-10-01 10:00
main.txt                   2026-10-01 10:00
secret/key.txt             2026-10-01 10:00
```

File contents are in `tests/fixtures/ignores/`: `.gitignore` has `*.log`, `.aiignore` has `secret/` and `!keep.log`.
Times are UTC.

## E48.1 The default ignore files

`.gitignore` excludes the logs, `.aiignore` comes later in the list and re-includes `keep.log` and excludes `secret/`.

Fixture: `ignores`

1. `find`

```json request
{"query": "type = file"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = file

./
keep.log  [#1]
main.txt  [#1]
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
binary = "skip"
max_file_size = "10M"
encoding = "auto"
# fallback_encoding =  (not set)
archives = "none"
max_archive_size = "100M"
skip_generated = true
```

## E48.2 No ignore files

Fixture: `ignores`

```json request
{"query": "name ~ '*.log'", "traversal": {"ignore_files": []}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
app.log  [#1]
keep.log  [#1]
```

## E48.3 Only `.gitignore`

Fixture: `ignores`

```json request
{"query": "name = 'key.txt'", "traversal": {"ignore_files": [".gitignore"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'key.txt'

secret/
key.txt  [#1]
```

## E48.4 A later name wins

 With `.gitignore` after `.aiignore`, `keep.log` stays excluded.

Fixture: `ignores`

```json request
{"query": "name ~ '*.log' or name = 'main.txt'", "traversal": {"ignore_files": [".aiignore", ".gitignore"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'
#2 name = 'main.txt'

./
main.txt  [#2]
```

## E48.5 Ignore files for the session

Fixture: `ignores`

1. `update_config`

```json request update_config
{"set": "traversal.ignore_files = [\".gitignore\"]"}
```

```text response
[traversal]
ignore_files = ['.gitignore']    # was ['.gitignore', '.ignore', '.aiignore']
```

2. `find`

```json request
{"query": "name = 'key.txt'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'key.txt'

secret/
key.txt  [#1]
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
hidden = false    # default
ignore_files = ['.gitignore']    # update
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
max_archive_size = "100M"    # default
skip_generated = true    # default
```

## E48.6 An invalid value in a request

Fixture: `ignores`

```json request
{"traversal": {"ignore_files": ".aiignore"}}
```

```text error
error[invalid_option]: 'traversal.ignore_files' must be a list of strings
```

## E48.7 An invalid value for `update_config`

Fixture: `ignores`

```json request update_config
{"set": "traversal.ignore_files = \".aiignore\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.ignore_files'
  hint: a list of file names is expected
```

## E48.8 From the config file

Fixture: `ignores`

```toml conf/i.toml
[traversal]
ignore_files = [".gitignore"]
```

```text command
easy-peasy-mcp --config conf/i.toml
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
hidden = false    # default
ignore_files = ['.gitignore']    # file: conf/i.toml
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
max_archive_size = "100M"    # default
skip_generated = true    # default
```

2. `find`

```json request
{"query": "name = 'key.txt'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'key.txt'

secret/
key.txt  [#1]
```

## E48.9 An invalid value in the config file

Fixture: `ignores`

```toml conf/i.toml
[traversal]
ignore_files = [1]
```

```text command
easy-peasy-mcp --config conf/i.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/i.toml:2: invalid value for 'traversal.ignore_files': a list of file names is expected
```
