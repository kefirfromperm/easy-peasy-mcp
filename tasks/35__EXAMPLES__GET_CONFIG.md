# Examples for task 35

No configuration key is implemented yet (they come with their own tasks), so the result has only its first line; the
read-only line and the sections appear with the keys.

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
```

## E35.2 One section

Fixture: `get-config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
```

## E35.3 With the sources

Fixture: `get-config`

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
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
