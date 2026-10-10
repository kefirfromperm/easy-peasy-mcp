# Examples for task 52

Examples with a command line are startup examples (AGENTS.md). `native` is case-sensitive on Linux and
case-insensitive on macOS and Windows, so the example of the default runs only on Linux.

## Fixtures

### path-case

```text tree
README.md                  2026-10-01 10:00
notes.txt                  2026-10-01 10:00
readme.txt                 2026-10-01 10:00
```

File contents are in `tests/fixtures/path-case/`. Times are UTC.

## E52.1 Native: case-sensitive on Linux

Platform: `linux`

Fixture: `path-case`

1. `find`

```json request
{"query": "name ~ 'readme*'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ 'readme*'

./
readme.txt  [#1]
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
```

## E52.2 Case-insensitive in a request

Fixture: `path-case`

```json request
{"query": "name ~ 'readme*'", "traversal": {"path_case": "insensitive"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'readme*'

./
README.md  [#1]
readme.txt  [#1]
```

## E52.3 Case-sensitive in a request

Fixture: `path-case`

```json request
{"query": "name starts 'README'", "traversal": {"path_case": "sensitive"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name starts 'README'

./
README.md  [#1]
```

## E52.4 `/i` is case-insensitive whatever `path_case` is

Fixture: `path-case`

```json request
{"query": "name starts 'README' /i", "traversal": {"path_case": "sensitive"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name starts 'README' /i

./
README.md  [#1]
readme.txt  [#1]
```

## E52.5 `exclude` follows `path_case`

Fixture: `path-case`

```json request
{"query": "type = file", "traversal": {"path_case": "insensitive", "exclude": ["readme*"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = file

./
notes.txt  [#1]
```

## E52.6 Case-insensitive for the session

Fixture: `path-case`

1. `update_config`

```json request update_config
{"set": "traversal.path_case = \"insensitive\""}
```

```text response
[traversal]
path_case = "insensitive"    # was "native"
```

2. `find`

```json request
{"query": "name ~ 'readme*'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'readme*'

./
README.md  [#1]
readme.txt  [#1]
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
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "insensitive"    # update
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
```

## E52.7 An invalid value in a request

Fixture: `path-case`

```json request
{"traversal": {"path_case": "upper"}}
```

```text error
error[invalid_option]: 'traversal.path_case' must be one of: native, sensitive, insensitive
```

## E52.8 An invalid value for `update_config`

Fixture: `path-case`

```json request update_config
{"set": "traversal.path_case = \"upper\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.path_case'
  hint: expected one of: native, sensitive, insensitive
```

## E52.9 From the config file

Fixture: `path-case`

```toml conf/c.toml
[traversal]
path_case = "insensitive"
```

```text command
easy-peasy-mcp --config conf/c.toml
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
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "insensitive"    # file: conf/c.toml
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
```

2. `find`

```json request
{"query": "name ~ 'readme*'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'readme*'

./
README.md  [#1]
readme.txt  [#1]
```

## E52.10 An invalid value in the config file

Fixture: `path-case`

```toml conf/c.toml
[traversal]
path_case = "upper"
```

```text command
easy-peasy-mcp --config conf/c.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/c.toml:2: invalid value for 'traversal.path_case': expected one of: native, sensitive, insensitive
```
