# Examples for task 35

The result shows the keys implemented so far; every key task adds its key.

## Fixtures

### get-config

```text tree
src/main.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/get-config/`. Times are UTC.

## E35.1 The effective configuration

Fixture: `get-config`

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
```

## E35.2 One section

Fixture: `get-config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update
```

## E35.3 With the sources

Fixture: `get-config`

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
```

## E35.4 An unknown section

Fixture: `get-config`

```json request get_config
{ "section": "outputs" }
```

```text error
error[invalid_option]: unknown section 'outputs'
  hint: did you mean 'output'? expected one of: server, traversal, output, escape, query, kinds, git, languages
```

## E35.5 An unknown field

Fixture: `get-config`

```json request get_config
{ "sections": "output" }
```

```text error
error[invalid_option]: unknown field 'sections'
  hint: did you mean 'section'? expected one of: section, sources
```

## E35.6 A wrong type

Fixture: `get-config`

```json request get_config
{ "sources": "yes" }
```

```text error
error[invalid_option]: 'sources' must be a boolean
```
