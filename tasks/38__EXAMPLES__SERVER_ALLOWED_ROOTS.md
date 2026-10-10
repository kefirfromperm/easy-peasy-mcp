# Examples for task 38

`server.allowed_roots` is the first configuration key: `get_config` shows it. Examples with a command line are
startup examples (AGENTS.md).

## Fixtures

### roots

```text tree
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/roots/`. Times are UTC.

## E38.1 The default: the start directory

Fixture: `roots`

```json request get_config
{}
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
```

## E38.2 With the source

Fixture: `roots`

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
threads = 0    # default
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
```

## E38.3 Another section

The read-only line is always there.

Fixture: `roots`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update
```

## E38.4 A root from the command line

Fixture: `roots`

```text command
easy-peasy-mcp --root .
```

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[server]
allowed_roots = ['<fixture>']    # cli
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
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
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E38.5 The working directory outside of the roots

Fixture: `roots`

```text command
easy-peasy-mcp --root src
```

```json request
{ "query": "name = 'main.rs'" }
```

```text error
error[dir_not_allowed]: the working directory '.' is outside of the allowed roots
  hint: get_config shows the allowed roots
```

## E38.6 Roots from the config file

Relative paths are resolved against the start directory.

Fixture: `roots`

Platform: `linux`

```toml conf/roots.toml
[server]
allowed_roots = ["src", "."]
```

```text command
easy-peasy-mcp --config conf/roots.toml
```

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[server]
allowed_roots = ['<fixture>/src', '<fixture>']    # file: conf/roots.toml
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
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
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E38.7 `--root` replaces the list of the file

Fixture: `roots`

```toml conf/roots.toml
[server]
allowed_roots = ["src"]
```

```text command
easy-peasy-mcp --config conf/roots.toml --root .
```

```json request get_config
{}
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
```

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E38.8 `~` is the home directory

Fixture: `roots`

Platform: `linux`

```text env
HOME=<fixture>
```

```text command
easy-peasy-mcp --root ~
```

```json request get_config
{}
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
```

## E38.9 An invalid value

Fixture: `roots`

```toml conf/bad.toml
[server]
allowed_roots = "src"
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:2: invalid value for 'server.allowed_roots': a list of paths is expected
```

## E38.10 A root which doesn't exist

Fixture: `roots`

```text command
easy-peasy-mcp --root missing
```

```text stderr exit 1
easy-peasy-mcp: the allowed root 'missing' doesn't exist
```

## E38.11 A root of the config file which doesn't exist

Fixture: `roots`

```toml conf/bad.toml
[server]

allowed_roots = [
  ".",
  "missing",
]
```

```text command
easy-peasy-mcp --config conf/bad.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/bad.toml:3: the allowed root 'missing' doesn't exist
```

## E38.12 `--root` without a directory

Fixture: `roots`

```text command
easy-peasy-mcp --root
```

```text stderr exit 2
usage: easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--log-level <level>]
               [--version]
```

## E38.13 Read-only for update_config

Fixture: `roots`

```json request update_config
{ "set": "server.allowed_roots = []" }
```

```text error
error[read_only]: 'server.allowed_roots' is read-only
  hint: allowed roots are set only at start
```
