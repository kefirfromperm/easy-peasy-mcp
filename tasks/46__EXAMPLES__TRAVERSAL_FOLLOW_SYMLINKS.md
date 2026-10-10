# Examples for task 46

Examples with a command line are startup examples (AGENTS.md). The symlinks need a file system with symlinks; on
Windows creating them needs the developer mode or an elevated process.

## Fixtures

### links

```text tree
src/main.rs                2026-10-01 10:00
```

```text setup
symlink src/alias.rs -> main.rs
symlink vendor -> src
symlink loop -> .
symlink outside -> ..
```

File contents are in `tests/fixtures/links/`: `src/main.rs` has `fn main() {}`. `outside` points to the directory
which holds the fixture, out of the allowed root. Times are UTC.

## E46.1 Symlinks are not followed by default

A symlink is not a regular file: without a `type` condition it is not returned.

Fixture: `links`

1. `find`

```json request
{"query": "text has 'fn main'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'fn main'

src/
main.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn main⟧() {}
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
```

## E46.2 Following symlinks

`src/alias.rs` and `src/main.rs` are the same file: it is returned once, under the first path. `vendor/` is the same directory as `src/`, its files come later in path order. `loop/` points to the working directory itself and is skipped; `outside/` points out of the allowed roots.

Fixture: `links`

```json request
{"query": "text has 'fn main'", "traversal": {"follow_symlinks": true}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'fn main'

src/
alias.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn main⟧() {}

warning: permission_denied (1): outside/
```

## E46.3 A followed symlink to a file is a file and a link

Fixture: `links`

```json request
{"query": "type = file and type = link", "traversal": {"follow_symlinks": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 type = file
#2 type = link

src/
alias.rs  [#1 #2]

warning: permission_denied (1): outside/
```

## E46.4 Following symlinks in a session

Fixture: `links`

1. `update_config`

```json request update_config
{"set": "traversal.follow_symlinks = true"}
```

```text response
[traversal]
follow_symlinks = true    # was false
```

2. `find`

```json request
{"query": "text has 'fn main'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'fn main'

src/
alias.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn main⟧() {}

warning: permission_denied (1): outside/
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
follow_symlinks = true    # update
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

## E46.5 An invalid value in a request

Fixture: `links`

```json request
{"traversal": {"follow_symlinks": "yes"}}
```

```text error
error[invalid_option]: 'traversal.follow_symlinks' must be true or false
```

## E46.6 An invalid value for `update_config`

Fixture: `links`

```json request update_config
{"set": "traversal.follow_symlinks = 1"}
```

```text error
error[invalid_value]: invalid value for 'traversal.follow_symlinks'
  hint: true or false is expected
```

## E46.7 From the config file

Fixture: `links`

```toml conf/l.toml
[traversal]
follow_symlinks = true
```

```text command
easy-peasy-mcp --config conf/l.toml
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
follow_symlinks = true    # file: conf/l.toml
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
{"query": "text has 'fn main'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'fn main'

src/
alias.rs  [#1]  lines: 1  matches: 1
1:⟦#1:fn main⟧() {}

warning: permission_denied (1): outside/
```

## E46.8 An invalid value in the config file

Fixture: `links`

```toml conf/l.toml
[traversal]
follow_symlinks = "yes"
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'traversal.follow_symlinks': true or false is expected
```
