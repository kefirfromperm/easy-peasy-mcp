# Examples for task 44

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### depth

```text tree
a/x.txt                    2026-10-01 10:00
b/y.txt                    2026-10-01 10:00
top.txt                    2026-10-01 10:00
```

File contents are in `tests/fixtures/depth/`. Times are UTC.

## E44.1 The default: every depth

Fixture: `depth`

1. `find`

```json request
{ "query": "name = 'top.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'top.txt'

./
top.txt  [#1]
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
```

## E44.2 A minimum depth in the request

The children of the working directory are depth 1: `top.txt` is skipped, the directories are walked.

Fixture: `depth`

```json request
{ "query": "name ~ '*.txt'", "traversal": { "min_depth": 2 } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

a/
x.txt  [#1]
b/
y.txt  [#1]
```

## E44.3 A minimum depth for the session

Fixture: `depth`

1. `update_config`

```json request update_config
{"set": "[traversal]\nmin_depth = 2"}
```

```text response
[traversal]
min_depth = 2    # was 0
```

2. `find`

```json request
{ "query": "name ~ '*.txt'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

a/
x.txt  [#1]
b/
y.txt  [#1]
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 2    # update
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
```

## E44.4 Invalid values in a request

Fixture: `depth`

1. `find`

```json request
{ "traversal": { "min_depth": -1 } }
```

```text error
error[invalid_option]: 'traversal.min_depth' must be a whole number
```

2. `find`

```json request
{ "traversal": "deep" }
```

```text error
error[invalid_option]: 'traversal' must be an object
```

3. `find`

```json request
{ "traversal": { "min_dept": 1 } }
```

```text error
error[invalid_option]: unknown field 'traversal.min_dept'
  hint: did you mean 'min_depth'? expected one of: min_depth, max_depth, follow_symlinks, hidden, ignore_files, git_excludes, ignore_paths, exclude, path_case, binary, max_file_size, encoding, fallback_encoding, archives, max_archive_size, skip_generated
```

## E44.5 An invalid value for `update_config`

Fixture: `depth`

```json request update_config
{"set": "traversal.min_depth = \"2\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.min_depth'
  hint: a whole number is expected
```

## E44.6 From the config file

Fixture: `depth`

```toml conf/d.toml
[traversal]
min_depth = 2
```

```text command
easy-peasy-mcp --config conf/d.toml
```

1. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 2    # file: conf/d.toml
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
```

2. `find`

```json request
{ "query": "name ~ '*.txt'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

a/
x.txt  [#1]
b/
y.txt  [#1]
```

## E44.7 An invalid value in the config file

Fixture: `depth`

```toml conf/d.toml
[traversal]
min_depth = 1.5
```

```text command
easy-peasy-mcp --config conf/d.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/d.toml:2: invalid value for 'traversal.min_depth': a whole number is expected
```
