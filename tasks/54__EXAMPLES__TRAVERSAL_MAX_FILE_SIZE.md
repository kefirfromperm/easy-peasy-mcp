# Examples for task 54

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### big-files

```text tree
a-small.txt                2026-10-01 10:00
b-large.txt                2026-10-01 10:00
```

File contents are in `tests/fixtures/big-files/`: `a-small.txt` (9 bytes) has `a needle`, `b-large.txt`
(1059 bytes) has `the needle is here` and 130 lines `padding`. Times are UTC.

## E54.1 Files up to 10M are searched by default

Fixture: `big-files`

1. `find`

```json request
{"query": "text has 'needle'"}
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'needle'

./
a-small.txt  [#1]  lines: 1  matches: 1
1:a ⟦#1:needle⟧
b-large.txt  [#1]  lines: 1  matches: 1
1:the ⟦#1:needle⟧ is here
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

## E54.2 A smaller limit in a request

`b-large.txt` is not searched by content: `text has 'needle'` is unknown for it.

Fixture: `big-files`

```json request
{"query": "text has 'needle'", "traversal": {"max_file_size": "1K"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'needle'

./
a-small.txt  [#1]  lines: 1  matches: 1
1:a ⟦#1:needle⟧

warning: too_big (1): b-large.txt
```

## E54.3 A big file is returned by other conditions

Fixture: `big-files`

```json request
{"query": "name ~ '*.txt' or text has 'needle'", "traversal": {"max_file_size": "1K"}}
```

```text response
files: 2  lines: 1  matches: 1  result: r1
#1 name ~ '*.txt'
#2 text has 'needle'

./
a-small.txt  [#1 #2]  lines: 1  matches: 1
1:a ⟦#2:needle⟧
b-large.txt  [#1]

warning: too_big (1): b-large.txt
```

## E54.4 A limit for the session

Units in any case, fractions like the `size` target.

Fixture: `big-files`

1. `update_config`

```json request update_config
{"set": "traversal.max_file_size = \"0.5k\""}
```

```text response
[traversal]
max_file_size = "0.5k"    # was "10M"
```

2. `find`

```json request
{"query": "text has 'needle'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'needle'

./
a-small.txt  [#1]  lines: 1  matches: 1
1:a ⟦#1:needle⟧

warning: too_big (1): b-large.txt
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
binary = "skip"    # default
max_file_size = "0.5k"    # update
encoding = "auto"    # default
```

## E54.5 An invalid value in a request

Fixture: `big-files`

```json request
{"traversal": {"max_file_size": "10MB"}}
```

```text error
error[invalid_option]: 'traversal.max_file_size' must be a size like "10M" (units B, K, M, G)
```

## E54.6 An invalid value for `update_config`

Fixture: `big-files`

```json request update_config
{"set": "traversal.max_file_size = 1024"}
```

```text error
error[invalid_value]: invalid value for 'traversal.max_file_size'
  hint: a size like "10M" is expected (units B, K, M, G)
```

## E54.7 From the config file

Fixture: `big-files`

```toml conf/s.toml
[traversal]
max_file_size = "1K"
```

```text command
easy-peasy-mcp --config conf/s.toml
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
binary = "skip"    # default
max_file_size = "1K"    # file: conf/s.toml
encoding = "auto"    # default
```

2. `find`

```json request
{"query": "text has 'needle'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'needle'

./
a-small.txt  [#1]  lines: 1  matches: 1
1:a ⟦#1:needle⟧

warning: too_big (1): b-large.txt
```

## E54.8 An invalid value in the config file

Fixture: `big-files`

```toml conf/s.toml
[traversal]
max_file_size = "big"
```

```text command
easy-peasy-mcp --config conf/s.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/s.toml:2: invalid value for 'traversal.max_file_size': a size like "10M" is expected (units B, K, M, G)
```
