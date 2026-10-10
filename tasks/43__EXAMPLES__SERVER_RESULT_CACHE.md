# Examples for task 43

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### result-cache

```text tree
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/result-cache/`. Times are UTC.

## E43.1 The default

Fixture: `result-cache`

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

## E43.2 Turning the cache off and on

Without the cache a response has no result id; the ids count the kept results.

Fixture: `result-cache`

1. `find`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```

2. `update_config`

```json request update_config
{"set": "server.result_cache = 0"}
```

```text response
[server]
result_cache = 0    # was 50
```

3. `find`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0
#1 name = 'main.rs'

src/
main.rs  [#1]
```

4. `update_config`

```json request update_config
{"reset": ["server.result_cache"]}
```

```text response
[server]
result_cache = 50    # was 0, reset
```

5. `find`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E43.3 An invalid value for `update_config`

Fixture: `result-cache`

```json request update_config
{"set": "server.result_cache = 1.5"}
```

```text error
error[invalid_value]: invalid value for 'server.result_cache'
  hint: a whole number is expected (0: off)
```

## E43.4 From the config file

Fixture: `result-cache`

```toml conf/c.toml
[server]
result_cache = 0
```

```text command
easy-peasy-mcp --config conf/c.toml
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
threads = 0    # default
allow_update = true    # default
result_cache = 0    # file: conf/c.toml

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
```

2. `find`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0
#1 name = 'main.rs'

src/
main.rs  [#1]
```

## E43.5 An invalid value in the config file

Fixture: `result-cache`

```toml conf/c.toml
[server]
result_cache = -1
```

```text command
easy-peasy-mcp --config conf/c.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/c.toml:2: invalid value for 'server.result_cache': a whole number is expected (0: off)
```
