# Examples for task 36

No configuration key is implemented yet (they come with their own tasks), so the examples show the calls which change
nothing and the errors.

## Fixtures

### update-config

```text tree
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/update-config/`. Times are UTC.

## E36.1 A call which changes nothing

Fixture: `update-config`

```json request update_config
{}
```

```text response
# nothing changed
```

## E36.2 Reset everything

Every value already comes from the defaults.

Fixture: `update-config`

```json request update_config
{"reset": ["all"]}
```

```text response
# nothing changed
```

## E36.3 Invalid TOML

Fixture: `update-config`

```json request update_config
{"set": "output.context = "}
```

```text error
error[invalid_toml]: invalid TOML in 'set'
  hint: line 1, col 18: string values must be quoted, expected literal string
```

## E36.4 A read-only key in `set`

Fixture: `update-config`

```json request update_config
{"set": "server.allowed_roots = [\"/\"]"}
```

```text error
error[read_only]: 'server.allowed_roots' is read-only
  hint: allowed roots are set only at start
```

## E36.5 A read-only key in `reset`

Fixture: `update-config`

```json request update_config
{"reset": ["server.allow_update"]}
```

```text error
error[read_only]: 'server.allow_update' is read-only
  hint: allow_update is set only at start
```

## E36.6 An unknown key

Fixture: `update-config`

```json request update_config
{"set": "[output]\nmax_file = 10"}
```

```text error
error[unknown_key]: unknown key 'output.max_file'
  hint: did you mean 'output.max_files'?
```

## E36.7 An unknown section

Fixture: `update-config`

```json request update_config
{"set": "[colors]\nerror = \"red\""}
```

```text error
error[unknown_key]: unknown key 'colors'
  hint: expected one of: server, traversal, output, escape, query, kinds, git, languages
```

## E36.8 An unknown key in `reset`

Fixture: `update-config`

```json request update_config
{"reset": ["output.max_file"]}
```

```text error
error[unknown_key]: unknown key 'output.max_file'
  hint: did you mean 'output.max_files'?
```

## E36.9 `reset` comes before `set`

Both are invalid; the error of `reset` is reported.

Fixture: `update-config`

```json request update_config
{"set": "output.max_file = 10", "reset": ["server.allowed_roots"]}
```

```text error
error[read_only]: 'server.allowed_roots' is read-only
  hint: allowed roots are set only at start
```

## E36.10 Arguments of a wrong type

Fixture: `update-config`

1. `update_config`

```json request update_config
{"set": 1}
```

```text error
error[invalid_option]: 'set' must be a string
```

2. `update_config`

```json request update_config
{"reset": "all"}
```

```text error
error[invalid_option]: 'reset' must be a list of strings
```

## E36.11 An unknown field

Fixture: `update-config`

```json request update_config
{"sett": "output.context = 2"}
```

```text error
error[invalid_option]: unknown field 'sett'
  hint: did you mean 'set'? expected one of: set, reset
```

## E36.12 A session

A failed update changes nothing: the configuration and the search stay as they were.

Fixture: `update-config`

1. `update_config`

```json request update_config
{"set": "output.max_file = 10"}
```

```text error
error[unknown_key]: unknown key 'output.max_file'
  hint: did you mean 'output.max_files'?
```

2. `get_config`

```json request get_config
{}
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots

[server]
allowed_roots = ['<fixture>']
default_dirs = ['.']
timeout = "30s"
```

3. `find`

```json request find
{"query": "name = 'main.rs'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

src/
main.rs  [#1]
```
