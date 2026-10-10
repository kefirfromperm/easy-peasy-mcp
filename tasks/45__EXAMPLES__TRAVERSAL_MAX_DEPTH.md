# Examples for task 45

Examples with a command line are startup examples (AGENTS.md). The fixture `depth` is the one of task 44.

## E45.1 A maximum depth in the request

The children of the working directory are depth 1.

Fixture: `depth`

```json request
{ "query": "name ~ '*.txt'", "traversal": { "max_depth": 1 } }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

./
top.txt  [#1]
```

## E45.2 Not set by default

Fixture: `depth`

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
```

## E45.3 A maximum depth for the session

Fixture: `depth`

1. `update_config`

```json request update_config
{"set": "traversal.max_depth = 1"}
```

```text response
[traversal]
max_depth = 1    # was (not set)
```

2. `find`

```json request
{ "query": "name ~ '*.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

./
top.txt  [#1]
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
max_depth = 1    # update
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
```

4. `update_config`

```json request update_config
{"reset": ["traversal.max_depth"]}
```

```text response
[traversal]
# max_depth =  (not set)    # was 1, reset
```

## E45.4 With a minimum depth

Fixture: `depth`

```json request
{ "query": "name ~ '*.txt'", "traversal": { "min_depth": 2, "max_depth": 2 } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

a/
x.txt  [#1]
b/
y.txt  [#1]
```

## E45.5 An invalid value in a request

Fixture: `depth`

```json request
{ "traversal": { "max_depth": "1" } }
```

```text error
error[invalid_option]: 'traversal.max_depth' must be a whole number
```

## E45.6 An invalid value for `update_config`

Fixture: `depth`

```json request update_config
{"set": "traversal.max_depth = -2"}
```

```text error
error[invalid_value]: invalid value for 'traversal.max_depth'
  hint: a whole number is expected
```

## E45.7 From the config file

Fixture: `depth`

```toml conf/d.toml
[traversal]
max_depth = 1
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
min_depth = 0    # default
max_depth = 1    # file: conf/d.toml
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
```

2. `find`

```json request
{ "query": "name ~ '*.txt'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'

./
top.txt  [#1]
```

## E45.8 An invalid value in the config file

Fixture: `depth`

```toml conf/d.toml
[traversal]
max_depth = "deep"
```

```text command
easy-peasy-mcp --config conf/d.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/d.toml:2: invalid value for 'traversal.max_depth': a whole number is expected
```
