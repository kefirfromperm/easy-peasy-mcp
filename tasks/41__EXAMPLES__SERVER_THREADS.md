# Examples for task 41

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### threads

```text tree
lib/c.rs                   2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
```

File contents are in `tests/fixtures/threads/`: every file has one line with `TODO`. Times are UTC.

## E41.1 The same response with one thread and with all cores

Fixture: `threads`

1. `find`

```json request
{ "query": "text has 'TODO'" }
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'TODO'

lib/
c.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ three
src/
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ one
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ two
```

2. `update_config`

```json request update_config
{"set": "server.threads = 1"}
```

```text response
[server]
threads = 1    # was 0
```

3. `find`

```json request
{ "query": "text has 'TODO'" }
```

```text response
files: 3  lines: 3  matches: 3  result: r2
#1 text has 'TODO'

lib/
c.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ three
src/
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ one
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ two
```

4. `get_config`

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 1    # update
allow_update = true    # default
result_cache = 50    # default

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
max_archive_size = "100M"    # default
```

## E41.2 The default

Fixture: `threads`

```json request get_config
{ "section": "server" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[server]
allowed_roots = ['<fixture>']
default_dirs = ['.']
timeout = "30s"
threads = 0
allow_update = true
result_cache = 50
```

## E41.3 An invalid value for `update_config`

Fixture: `threads`

```json request update_config
{"set": "server.threads = -1"}
```

```text error
error[invalid_value]: invalid value for 'server.threads'
  hint: a whole number is expected (0: the number of CPU cores)
```

## E41.4 Threads from the config file

Fixture: `threads`

```toml conf/t.toml
[server]
threads = 2
```

```text command
easy-peasy-mcp --config conf/t.toml
```

1. `get_config`

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 2    # file: conf/t.toml
allow_update = true    # default
result_cache = 50    # default

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
max_archive_size = "100M"    # default
```

2. `find`

```json request
{ "query": "text has 'TODO'" }
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'TODO'

lib/
c.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ three
src/
a.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ one
b.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:TODO⟧ two
```

## E41.5 An invalid value in the config file

Fixture: `threads`

```toml conf/t.toml
[server]
threads = "many"
```

```text command
easy-peasy-mcp --config conf/t.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/t.toml:2: invalid value for 'server.threads': a whole number is expected (0: the number of CPU cores)
```
