# Examples for task 56

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `legacy` of task 55:
`bom.txt` is UTF-8 with a BOM, `latin.txt` Windows-1252 (`café au lait`), `plain.txt` UTF-8 without a BOM.

## E56.1 No fallback by default

The invalid byte of `latin.txt` is replaced.

Fixture: `legacy`

1. `find`

```json request
{"query": "name = 'latin.txt' and text has 'au lait'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'latin.txt'
#2 text has 'au lait'

./
latin.txt  [#1 #2]  lines: 1  matches: 1
1:caf� ⟦#2:au lait⟧

warning: decode_errors (1): latin.txt
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

## E56.2 A fallback in a request

Only the file which is not valid UTF-8 is decoded with it.

Fixture: `legacy`

```json request
{"query": "text has 'caf'", "traversal": {"fallback_encoding": "windows-1252"}}
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'caf'

./
bom.txt  [#1]  lines: 1  matches: 1
1:⟦#1:caf⟧é with a bom
latin.txt  [#1]  lines: 1  matches: 1  windows-1252
1:⟦#1:caf⟧é au lait
plain.txt  [#1]  lines: 1  matches: 1
1:plain ⟦#1:caf⟧é
```

## E56.3 Only with `encoding = auto`

Fixture: `legacy`

```json request
{"query": "name = 'latin.txt' and text has 'au lait'", "traversal": {"encoding": "utf-8", "fallback_encoding": "windows-1252"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'latin.txt'
#2 text has 'au lait'

./
latin.txt  [#1 #2]  lines: 1  matches: 1
1:caf� ⟦#2:au lait⟧

warning: decode_errors (1): latin.txt
```

## E56.4 A fallback for the session, then reset

Fixture: `legacy`

1. `update_config`

```json request update_config
{"set": "traversal.fallback_encoding = \"windows-1252\""}
```

```text response
[traversal]
fallback_encoding = "windows-1252"    # was (not set)
```

2. `find`

```json request
{"query": "text has 'café au'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'café au'

./
latin.txt  [#1]  lines: 1  matches: 1  windows-1252
1:⟦#1:café au⟧ lait
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
max_file_size = "10M"    # default
encoding = "auto"    # default
fallback_encoding = "windows-1252"    # update
archives = "none"    # default
max_archive_size = "100M"    # default
skip_generated = true    # default
```

4. `update_config`

```json request update_config
{"reset": ["traversal.fallback_encoding"]}
```

```text response
[traversal]
# fallback_encoding =  (not set)    # was "windows-1252", reset
```

## E56.5 An invalid value in a request

Fixture: `legacy`

```json request
{"traversal": {"fallback_encoding": "auto"}}
```

```text error
error[invalid_option]: 'traversal.fallback_encoding' must be an encoding name like "windows-1251"
```

## E56.6 An invalid value for `update_config`

Fixture: `legacy`

```json request update_config
{"set": "traversal.fallback_encoding = \"cyrillic-1\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.fallback_encoding'
  hint: an encoding name like "windows-1251" is expected
```

## E56.7 From the config file

Fixture: `legacy`

```toml conf/f.toml
[traversal]
fallback_encoding = "windows-1252"
```

```text command
easy-peasy-mcp --config conf/f.toml
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
max_file_size = "10M"    # default
encoding = "auto"    # default
fallback_encoding = "windows-1252"    # file: conf/f.toml
archives = "none"    # default
max_archive_size = "100M"    # default
skip_generated = true    # default
```

2. `find`

```json request
{"query": "text has 'café au'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'café au'

./
latin.txt  [#1]  lines: 1  matches: 1  windows-1252
1:⟦#1:café au⟧ lait
```

## E56.8 An invalid value in the config file

Fixture: `legacy`

```toml conf/f.toml
[traversal]
fallback_encoding = 1251
```

```text command
easy-peasy-mcp --config conf/f.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/f.toml:2: invalid value for 'traversal.fallback_encoding': an encoding name like "windows-1251" is expected
```
