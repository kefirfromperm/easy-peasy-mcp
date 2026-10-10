# Examples for task 110

A JSON response is compared as a JSON value. The examples use the fixture `detail` of task 79: `src/a.rs` and
`src/b.rs` with two lines with `TODO` each.

## E110.1 Spec 1

The same response as without `version`.

Fixture: `detail`

```json request
{"version": 1, "query": "text has 'TODO'", "output": {"detail": "count"}}
```

```text response
files: 2  lines: 4  matches: 4  result: r1
```

## E110.2 The version in JSON

Every JSON response has it, the text format doesn't.

Fixture: `detail`

```json request
{"query": "text has 'TODO'", "output": {"detail": "count", "format": "json"}}
```

```json response
{ "version": 1, "exists": true, "result": "r1", "counts": { "files": 2, "lines": 4, "matches": 4 } }
```

## E110.3 An unsupported version

Fixture: `detail`

```json request
{"version": 2, "query": "text has 'TODO'"}
```

```text error
error[unsupported_version]: the spec version 2 is not supported
  hint: supported spec versions: 1
```

In JSON:

```json request
{"version": 2, "query": "text has 'TODO'", "output": {"format": "json"}}
```

```json error
{ "error": { "code": "unsupported_version", "message": "the spec version 2 is not supported", "hint": "supported spec versions: 1" } }
```

## E110.4 An invalid version

Fixture: `detail`

```json request
{"version": 0, "query": "text has 'TODO'"}
```

```text error
error[invalid_option]: 'version' must be a whole number of at least 1
```

```json request
{"version": "1", "query": "text has 'TODO'"}
```

```text error
error[invalid_option]: 'version' must be a whole number of at least 1
```
