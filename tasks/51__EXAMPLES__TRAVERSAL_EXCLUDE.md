# Examples for task 51

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### exclude

```text tree
lib/                       2026-10-01 10:00
lib/src/out.txt            2026-10-01 10:00
lib/target/out.txt         2026-10-01 10:00
src/                       2026-10-01 10:00
src/out.txt                2026-10-01 10:00
target/                    2026-10-01 10:00
target/out.txt             2026-10-01 10:00
```

File contents are in `tests/fixtures/exclude/`. Times are UTC.

## E51.1 Nothing is excluded by default

Fixture: `exclude`

1. `find`

```json request
{"query": "path ~ '*/out.txt'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path ~ '*/out.txt'

src/
out.txt  [#1]
target/
out.txt  [#1]
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

## E51.2 Every `target` directory

Fixture: `exclude`

1. `find`

```json request
{"query": "path ~ '*/out.txt'", "traversal": {"exclude": ["**/target/**"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ '*/out.txt'

src/
out.txt  [#1]
```

2. `find`

```json request
{"query": "path ~ 'lib/*/out.txt'", "traversal": {"exclude": ["**/target/**"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 path ~ 'lib/*/out.txt'

lib/src/
out.txt  [#1]
```

## E51.3 Only the top level `target` directory

Fixture: `exclude`

```json request
{"query": "name = 'out.txt' and path has 'target'", "traversal": {"exclude": ["target/**"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'out.txt'
#2 path has 'target'

lib/target/
out.txt  [#1 #2]
```

## E51.4 An excluded directory is not an entry

`lib/**` matches `lib/`, so the directory `lib` is excluded too.

Fixture: `exclude`

```json request
{"query": "type = dir", "traversal": {"exclude": ["lib/**"]}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 type = dir

./
src/  [#1]
target/  [#1]
```

## E51.5 Exclusions for the session

Fixture: `exclude`

1. `update_config`

```json request update_config
{"set": "traversal.exclude = [\"**/target/**\"]"}
```

```text response
[traversal]
exclude = ['**/target/**']    # was []
```

2. `find`

```json request
{"query": "path ~ '*/out.txt'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ '*/out.txt'

src/
out.txt  [#1]
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
exclude = ['**/target/**']    # update
path_case = "native"    # default
```

## E51.6 An invalid value in a request

Fixture: `exclude`

```json request
{"traversal": {"exclude": "**/target/**"}}
```

```text error
error[invalid_option]: 'traversal.exclude' must be a list of strings
```

## E51.7 An invalid wildcard in a request

Fixture: `exclude`

```json request
{"traversal": {"exclude": ["**/target/**", "[a-"]}}
```

```text error
error[invalid_option]: invalid wildcard '[a-' in 'traversal.exclude'
  hint: unclosed '['
```

## E51.8 An invalid value for `update_config`

Fixture: `exclude`

```json request update_config
{"set": "traversal.exclude = \"**/target/**\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.exclude'
  hint: a list of wildcards is expected
```

## E51.9 An invalid wildcard for `update_config`

Fixture: `exclude`

```json request update_config
{"set": "traversal.exclude = ['[a-']"}
```

```text error
error[invalid_value]: invalid value for 'traversal.exclude'
  hint: invalid wildcard '[a-': unclosed '['
```

## E51.10 From the config file

Fixture: `exclude`

```toml conf/e.toml
[traversal]
exclude = ["**/target/**"]
```

```text command
easy-peasy-mcp --config conf/e.toml
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
exclude = ['**/target/**']    # file: conf/e.toml
path_case = "native"    # default
```

2. `find`

```json request
{"query": "path ~ '*/out.txt'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ '*/out.txt'

src/
out.txt  [#1]
```

## E51.11 An invalid wildcard in the config file

Fixture: `exclude`

```toml conf/e.toml
[traversal]
exclude = ["[a-"]
```

```text command
easy-peasy-mcp --config conf/e.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/e.toml:2: invalid value for 'traversal.exclude': invalid wildcard '[a-': unclosed '['
```
