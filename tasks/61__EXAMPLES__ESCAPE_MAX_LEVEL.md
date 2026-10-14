# Examples for task 61

Spec: `1`

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### levels

```text tree
nested.json                2026-10-01 10:00
```

File contents are in `tests/fixtures/levels/`: `nested.json` has `"role": "admin"` on the levels 0 to 3 of the
`backslash` style, one per line:

```text
{"role": "admin"}
"{\"role\": \"admin\"}"
"{\\\"role\\\": \\\"admin\\\"}"
"{\\\\\\\"role\\\\\\\": \\\\\\\"admin\\\\\\\"}"
```

Times are UTC.

## E61.1 Up to level 3 by default

Fixture: `levels`

1. `find`

```json request
{"query": "name = 'nested.json' and text has `\"role\": \"admin\"` /e(backslash, any)"}
```

```text response
files: 1  lines: 4  matches: 4  result: r1
#1 name = 'nested.json'
#2 text has `"role": "admin"` /e(backslash, any)

./
nested.json  [#1 #2]  lines: 4  matches: 4
1:{⟦#2/e0:"role": "admin"⟧}
2:"{⟦#2/e1:\"role\": \"admin\"⟧}"
3:"{⟦#2/e2:\\\"role\\\": \\\"admin\\\"⟧}"
4:"{⟦#2/e3:\\\\\\\"role\\\\\\\": \\\\\\\"admin\\\\\\\"⟧}"
```

2. `get_config`

```json request get_config
{ "section": "escape" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[escape]
max_level = 3
```

## E61.2 A lower level in a request

`any` is the levels 0 to 1.

Fixture: `levels`

```json request
{"query": "name = 'nested.json' and text has `\"role\": \"admin\"` /e(backslash, any)", "escape": {"max_level": 1}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'nested.json'
#2 text has `"role": "admin"` /e(backslash, any)

./
nested.json  [#1 #2]  lines: 2  matches: 2
1:{⟦#2/e0:"role": "admin"⟧}
2:"{⟦#2/e1:\"role\": \"admin\"⟧}"
```

## E61.3 A query asks for a level above the configured one

Fixture: `levels`

```json request
{"query": "text has 'a' /e(backslash, 2)", "escape": {"max_level": 1}}
```

```text error
error[invalid_escape_level]: invalid escape level 2
  text has 'a' /e(backslash, 2)
                             ^
  hint: max level is 1 (configuration escape.max_level)
```

## E61.4 A level for the session

Fixture: `levels`

1. `update_config`

```json request update_config
{"set": "escape.max_level = 2"}
```

```text response
[escape]
max_level = 2    # was 3
```

2. `find`

```json request
{"query": "name = 'nested.json' and text has `\"role\": \"admin\"` /e(backslash, any)"}
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'nested.json'
#2 text has `"role": "admin"` /e(backslash, any)

./
nested.json  [#1 #2]  lines: 3  matches: 3
1:{⟦#2/e0:"role": "admin"⟧}
2:"{⟦#2/e1:\"role\": \"admin\"⟧}"
3:"{⟦#2/e2:\\\"role\\\": \\\"admin\\\"⟧}"
```

3. `get_config`

```json request get_config
{ "section": "escape", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[escape]
max_level = 2    # update
```

## E61.5 An invalid value in a request

Fixture: `levels`

```json request
{"escape": {"max_level": "3"}}
```

```text error
error[invalid_option]: 'escape.max_level' must be a whole number
```

## E61.6 An unknown key in a request

Fixture: `levels`

```json request
{"escape": {"depth": 1}}
```

```text error
error[invalid_option]: unknown field 'escape.depth'
  hint: expected one of: max_level
```

## E61.7 An invalid value for `update_config`

Fixture: `levels`

```json request update_config
{"set": "escape.max_level = -1"}
```

```text error
error[invalid_value]: invalid value for 'escape.max_level'
  hint: a whole number is expected
```

## E61.8 From the config file

Fixture: `levels`

```toml conf/e.toml
[escape]
max_level = 1
```

```text command
easy-peasy-mcp --config conf/e.toml
```

1. `get_config`

```json request get_config
{ "section": "escape", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[escape]
max_level = 1    # file: conf/e.toml
```

2. `find`

```json request
{"query": "name = 'nested.json' and text has `\"role\": \"admin\"` /e(backslash, any)"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'nested.json'
#2 text has `"role": "admin"` /e(backslash, any)

./
nested.json  [#1 #2]  lines: 2  matches: 2
1:{⟦#2/e0:"role": "admin"⟧}
2:"{⟦#2/e1:\"role\": \"admin\"⟧}"
```

## E61.9 An invalid value in the config file

Fixture: `levels`

```toml conf/e.toml
[escape]
max_level = "3"
```

```text command
easy-peasy-mcp --config conf/e.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/e.toml:2: invalid value for 'escape.max_level': a whole number is expected
```
