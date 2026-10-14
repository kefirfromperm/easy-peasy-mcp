# Examples for task 50

Spec: `1`

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### ignore-paths

```text tree
.gitignore                 2026-10-01 10:00
a.log                      2026-10-01 10:00
b.txt                      2026-10-01 10:00
conf/personal.ignore       2026-10-01 10:00
conf/top.ignore            2026-10-01 10:00
keep.log                   2026-10-01 10:00
src/x.tmp                  2026-10-01 10:00
src/y/x.tmp                2026-10-01 10:00
```

File contents are in `tests/fixtures/ignore-paths/`: `.gitignore` has `!keep.log`, `conf/personal.ignore` has `*.log`,
`conf/top.ignore` has `/x.tmp`. Times are UTC.

## E50.1 No extra ignore files by default

Fixture: `ignore-paths`

1. `find`

```json request
{"query": "name ~ '*.log'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
a.log  [#1]
keep.log  [#1]
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

## E50.2 An extra ignore file in the request

`conf/personal.ignore` excludes the logs with the lowest priority, so `!keep.log` of `.gitignore` re-includes
`keep.log`.

Fixture: `ignore-paths`

```json request
{"query": "name ~ '*.log'", "traversal": {"ignore_paths": ["conf/personal.ignore"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
keep.log  [#1]
```

## E50.3 The rules apply as if the file were in the working directory

`/x.tmp` excludes `x.tmp` directly in the working directory `src`, not `src/y/x.tmp`.

Fixture: `ignore-paths`

```json request
{"dirs": ["src"], "query": "name = 'x.tmp'", "traversal": {"ignore_paths": ["conf/top.ignore"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'x.tmp'

src/y/
x.tmp  [#1]
```

## E50.4 A file which doesn't exist is skipped

Fixture: `ignore-paths`

```json request
{"query": "name ~ '*.log'", "traversal": {"ignore_paths": ["conf/missing.ignore"]}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
a.log  [#1]
keep.log  [#1]
```

## E50.5 Extra ignore files for the session

Fixture: `ignore-paths`

1. `update_config`

```json request update_config
{"set": "traversal.ignore_paths = [\"conf/personal.ignore\"]"}
```

```text response
[traversal]
ignore_paths = ['conf/personal.ignore']    # was []
```

2. `find`

```json request
{"query": "name ~ '*.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
keep.log  [#1]
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
git_excludes = true    # default
ignore_paths = ['conf/personal.ignore']    # update
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

## E50.6 An invalid value in a request

Fixture: `ignore-paths`

```json request
{"traversal": {"ignore_paths": "conf/personal.ignore"}}
```

```text error
error[invalid_option]: 'traversal.ignore_paths' must be a list of strings
```

## E50.7 An invalid value for `update_config`

Fixture: `ignore-paths`

```json request update_config
{"set": "traversal.ignore_paths = [1]"}
```

```text error
error[invalid_value]: invalid value for 'traversal.ignore_paths'
  hint: a list of paths is expected
```

## E50.8 From the config file

Fixture: `ignore-paths`

```toml conf/p.toml
[traversal]
ignore_paths = ["conf/personal.ignore"]
```

```text command
easy-peasy-mcp --config conf/p.toml
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
git_excludes = true    # default
ignore_paths = ['conf/personal.ignore']    # file: conf/p.toml
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
{"query": "name ~ '*.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
keep.log  [#1]
```

## E50.9 A path in the home directory

Fixture: `ignore-paths`

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```toml conf/p.toml
[traversal]
ignore_paths = ["~/conf/personal.ignore"]
```

```text command
easy-peasy-mcp --config conf/p.toml
```

```json request
{"query": "name ~ '*.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

./
keep.log  [#1]
```

## E50.10 An invalid value in the config file

Fixture: `ignore-paths`

```toml conf/p.toml
[traversal]
ignore_paths = "conf/personal.ignore"
```

```text command
easy-peasy-mcp --config conf/p.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/p.toml:2: invalid value for 'traversal.ignore_paths': a list of paths is expected
```
