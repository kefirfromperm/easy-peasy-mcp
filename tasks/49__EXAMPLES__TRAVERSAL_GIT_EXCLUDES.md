# Examples for task 49

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). The global excludes file of git (`core.excludesFile`)
belongs to the machine, so the examples show `.git/info/exclude`; the test server ignores the global one.

## Fixtures

### excludes

```text tree
a.tmp                      2026-10-01 10:00
b.txt                      2026-10-01 10:00
```

```text setup
git . init -q
bytes .git/info/exclude 2a2e746d700a
```

File contents are in `tests/fixtures/excludes/`. The setup makes a git repository whose `.git/info/exclude` has
`*.tmp`. Times are UTC.

## E49.1 `.git/info/exclude` is respected by default

Fixture: `excludes`

1. `find`

```json request
{"query": "type = file"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = file

./
b.txt  [#1]
```

2. `get_config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

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

## E49.2 Without the git excludes in the request

Fixture: `excludes`

```json request
{"query": "name ~ '*.tmp'", "traversal": {"git_excludes": false}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.tmp'

./
a.tmp  [#1]
```

## E49.3 Without the git excludes in a session

Fixture: `excludes`

1. `update_config`

```json request update_config
{"set": "traversal.git_excludes = false"}
```

```text response
[traversal]
git_excludes = false    # was true
```

2. `find`

```json request
{"query": "name ~ '*.tmp'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.tmp'

./
a.tmp  [#1]
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = false    # update
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

## E49.4 An invalid value in a request

Fixture: `excludes`

```json request
{"traversal": {"git_excludes": "no"}}
```

```text error
error[invalid_option]: 'traversal.git_excludes' must be true or false
```

## E49.5 An invalid value for `update_config`

Fixture: `excludes`

```json request update_config
{"set": "traversal.git_excludes = 0"}
```

```text error
error[invalid_value]: invalid value for 'traversal.git_excludes'
  hint: true or false is expected
```

## E49.6 From the config file

Fixture: `excludes`

```toml conf/g.toml
[traversal]
git_excludes = false
```

```text command
easy-peasy-mcp --config conf/g.toml
```

1. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = false    # file: conf/g.toml
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
{"query": "name ~ '*.tmp'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.tmp'

./
a.tmp  [#1]
```

## E49.7 An invalid value in the config file

Fixture: `excludes`

```toml conf/g.toml
[traversal]
git_excludes = "no"
```

```text command
easy-peasy-mcp --config conf/g.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/g.toml:2: invalid value for 'traversal.git_excludes': true or false is expected
```
