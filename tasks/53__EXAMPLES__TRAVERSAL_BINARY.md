# Examples for task 53

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### binary-files

```text tree
data.bin                   2026-10-01 10:00
notes.txt                  2026-10-01 10:00
```

File contents are in `tests/fixtures/binary-files/`: `data.bin` is four zero bytes and a line break, then
`logo inside`; `notes.txt` has `intro` and `the logo is here`. Times are UTC.

## E53.1 Binary files are skipped by default

Fixture: `binary-files`

1. `find`

```json request
{"query": "text has 'logo'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'logo'

./
notes.txt  [#1]  lines: 1  matches: 1
2:the ⟦#1:logo⟧ is here

warning: binary_skipped (1): data.bin
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
```

## E53.2 Binary files searched as text

Fixture: `binary-files`

```json request
{"query": "text has 'logo'", "traversal": {"binary": "search"}}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'logo'

./
data.bin  [#1]  lines: 1  matches: 1
2:⟦#1:logo⟧ inside
notes.txt  [#1]  lines: 1  matches: 1
2:the ⟦#1:logo⟧ is here
```

## E53.3 Binary files never returned

Without a content condition too, like `grep -I`.

Fixture: `binary-files`

```json request
{"query": "type = file", "traversal": {"binary": "exclude"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = file

./
notes.txt  [#1]
```

## E53.4 Binary files for the session

Fixture: `binary-files`

1. `update_config`

```json request update_config
{"set": "traversal.binary = \"search\""}
```

```text response
[traversal]
binary = "search"    # was "skip"
```

2. `find`

```json request
{"query": "text has 'inside'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'inside'

./
data.bin  [#1]  lines: 1  matches: 1
2:logo ⟦#1:inside⟧
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
path_case = "native"    # default
binary = "search"    # update
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
```

## E53.5 An invalid value in a request

Fixture: `binary-files`

```json request
{"traversal": {"binary": "text"}}
```

```text error
error[invalid_option]: 'traversal.binary' must be one of: skip, search, exclude
```

## E53.6 An invalid value for `update_config`

Fixture: `binary-files`

```json request update_config
{"set": "traversal.binary = true"}
```

```text error
error[invalid_value]: invalid value for 'traversal.binary'
  hint: expected one of: skip, search, exclude
```

## E53.7 From the config file

Fixture: `binary-files`

```toml conf/b.toml
[traversal]
binary = "exclude"
```

```text command
easy-peasy-mcp --config conf/b.toml
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
path_case = "native"    # default
binary = "exclude"    # file: conf/b.toml
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
```

2. `find`

```json request
{"query": "name ~ '*.{bin,txt}'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.{bin,txt}'

./
notes.txt  [#1]
```

## E53.8 An invalid value in the config file

Fixture: `binary-files`

```toml conf/b.toml
[traversal]
binary = "text"
```

```text command
easy-peasy-mcp --config conf/b.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/b.toml:2: invalid value for 'traversal.binary': expected one of: skip, search, exclude
```
