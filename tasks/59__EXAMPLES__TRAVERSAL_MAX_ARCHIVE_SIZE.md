# Examples for task 59

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `archives` of task 58:
`logs/a.zip` has the members `inner/zip.log` (36 bytes) and `inner/nested.zip` (an archive), `logs/broken.zip` is
not a zip archive.

## E59.1 Archives up to 100M by default

Fixture: `archives`

1. `find`

```json request
{"query": "path ~ 'logs/a.zip!/**'", "traversal": {"archives": "all"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path ~ 'logs/a.zip!/**'

logs/a.zip!/inner/
nested.zip  [#1]
zip.log  [#1]

warning: archive_error (1): logs/broken.zip
```

2. `get_config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
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

## E59.2 A smaller limit in a request

The first member fits, the second one would exceed the limit: it and the rest are skipped.

Fixture: `archives`

```json request
{"query": "path ~ 'logs/a.zip!/**'", "traversal": {"archives": "all", "max_archive_size": "40"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ 'logs/a.zip!/**'

logs/a.zip!/inner/
zip.log  [#1]

warning: archive_error (1): logs/broken.zip
warning: archive_too_big (1): logs/a.zip
```

## E59.3 A limit for the session

Fixture: `archives`

1. `update_config`

```json request update_config
{"set": "traversal.max_archive_size = \"40\""}
```

```text response
[traversal]
max_archive_size = "40"    # was "100M"
```

2. `find`

```json request
{"query": "name = 'nested.zip' or name = 'zip.log'", "traversal": {"archives": "all"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'nested.zip'
#2 name = 'zip.log'

logs/a.zip!/inner/
zip.log  [#2]

warning: archive_error (1): logs/broken.zip
warning: archive_too_big (1): logs/a.zip
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
max_archive_size = "40"    # update
skip_generated = true    # default
```

## E59.4 An invalid value in a request

Fixture: `archives`

```json request
{"traversal": {"max_archive_size": "1T"}}
```

```text error
error[invalid_option]: 'traversal.max_archive_size' must be a size like "10M" (units B, K, M, G)
```

## E59.5 An invalid value for `update_config`

Fixture: `archives`

```json request update_config
{"set": "traversal.max_archive_size = 100"}
```

```text error
error[invalid_value]: invalid value for 'traversal.max_archive_size'
  hint: a size like "10M" is expected (units B, K, M, G)
```

## E59.6 From the config file

Fixture: `archives`

```toml conf/m.toml
[traversal]
archives = "all"
max_archive_size = "40"
```

```text command
easy-peasy-mcp --config conf/m.toml
```

1. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "all"    # file: conf/m.toml
max_archive_size = "40"    # file: conf/m.toml
skip_generated = true    # default
```

2. `find`

```json request
{"query": "path ~ 'logs/a.zip!/**'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ 'logs/a.zip!/**'

logs/a.zip!/inner/
zip.log  [#1]

warning: archive_error (1): logs/broken.zip
warning: archive_too_big (1): logs/a.zip
```

## E59.7 An invalid value in the config file

Fixture: `archives`

```toml conf/m.toml
[traversal]
max_archive_size = "huge"
```

```text command
easy-peasy-mcp --config conf/m.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/m.toml:2: invalid value for 'traversal.max_archive_size': a size like "10M" is expected (units B, K, M, G)
```
