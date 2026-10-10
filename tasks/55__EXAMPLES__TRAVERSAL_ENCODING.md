# Examples for task 55

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### legacy

```text tree
bom.txt                    2026-10-01 10:00
latin.txt                  2026-10-01 10:00
plain.txt                  2026-10-01 10:00
```

```text setup
bytes bom.txt efbbbf636166c3a92077697468206120626f6d0a
bytes latin.txt 636166e9206175206c6169740a
bytes plain.txt 706c61696e20636166c3a90a
```

The files are created by the test setup: `bom.txt` is UTF-8 with a BOM, `café with a bom`; `latin.txt` is
Windows-1252, `café au lait`; `plain.txt` is UTF-8 without a BOM, `plain café`. Times are UTC.

## E55.1 `auto` by default

`latin.txt` is not valid UTF-8: its byte `e9` is replaced.

Fixture: `legacy`

1. `find`

```json request
{"query": "text has 'caf'"}
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'caf'

./
bom.txt  [#1]  lines: 1  matches: 1
1:⟦#1:caf⟧é with a bom
latin.txt  [#1]  lines: 1  matches: 1
1:⟦#1:caf⟧� au lait
plain.txt  [#1]  lines: 1  matches: 1
1:plain ⟦#1:caf⟧é

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
```

## E55.2 An encoding in a request

Every file without a BOM is decoded with it, also the UTF-8 one; the file with a BOM is still UTF-8.

Fixture: `legacy`

```json request
{"query": "text has 'caf'", "traversal": {"encoding": "windows-1252"}}
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'caf'

./
bom.txt  [#1]  lines: 1  matches: 1
1:⟦#1:caf⟧é with a bom
latin.txt  [#1]  lines: 1  matches: 1  windows-1252
1:⟦#1:caf⟧é au lait
plain.txt  [#1]  lines: 1  matches: 1  windows-1252
1:plain ⟦#1:caf⟧Ã©
```

## E55.3 A label is shown by the name of its encoding

Fixture: `legacy`

```json request
{"query": "name = 'latin.txt' and text has 'café'", "traversal": {"encoding": "Latin1"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'latin.txt'
#2 text has 'café'

./
latin.txt  [#1 #2]  lines: 1  matches: 1  windows-1252
1:⟦#2:café⟧ au lait
```

## E55.4 An encoding for the session

Fixture: `legacy`

1. `update_config`

```json request update_config
{"set": "traversal.encoding = \"windows-1252\""}
```

```text response
[traversal]
encoding = "windows-1252"    # was "auto"
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
encoding = "windows-1252"    # update
# fallback_encoding =  (not set)    # default
archives = "none"    # default
```

## E55.5 An invalid value in a request

Fixture: `legacy`

```json request
{"traversal": {"encoding": "latin-99"}}
```

```text error
error[invalid_option]: 'traversal.encoding' must be "auto" or an encoding name like "windows-1252"
```

## E55.6 An invalid value for `update_config`

Fixture: `legacy`

```json request update_config
{"set": "traversal.encoding = \"ascii7\""}
```

```text error
error[invalid_value]: invalid value for 'traversal.encoding'
  hint: "auto" or an encoding name like "windows-1252" is expected
```

## E55.7 From the config file

Fixture: `legacy`

```toml conf/e.toml
[traversal]
encoding = "windows-1252"
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
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "windows-1252"    # file: conf/e.toml
# fallback_encoding =  (not set)    # default
archives = "none"    # default
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

## E55.8 An invalid value in the config file

Fixture: `legacy`

```toml conf/e.toml
[traversal]
encoding = "latin-99"
```

```text command
easy-peasy-mcp --config conf/e.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/e.toml:2: invalid value for 'traversal.encoding': "auto" or an encoding name like "windows-1252" is expected
```
