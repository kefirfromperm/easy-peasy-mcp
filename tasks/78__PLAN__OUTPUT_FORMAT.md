# Plan for task 78: `output.format`

## Design

- `src/config/keys.rs`: `Key::Format` in the section `output`, `Type::Choice(["text", "json"])`, default `"text"`; the
  request object `output` holds the keys of the section.
- `src/output/json.rs` (new): the JSON document of OUTPUT.md#json-format from the response — serde structs in the
  order of the fields table, empty and default values skipped; `legend` as a map in the order of the query; files with
  the full path (the working directory prefix, the trailing separator of a directory, `!/` of a member), `encoding`,
  `lines`, `matches`, `conditions`, `blocks` of consecutive matched lines with `n`, `text` and `fragments`
  (`col` in characters, 1-based, end exclusive; `text`, `ids`, `escape`; a multiline fragment in its first line with
  `to`). The document is written compact (`serde_json::to_string`).
- `src/warning.rs`: `Warnings::groups` — the code, the count and the example paths, shared by text and JSON.
- `src/error.rs`: `Report::to_json` — `{"error": {code, message, query, position {col, len}, hint}}`.
- `src/engine.rs`: the format of a request — its `output.format` when it is a valid value, otherwise the session's —
  decides the response and the error format.
- `tests/examples.rs`, `tests/startup.rs`: `json response` and `json error` blocks compared as JSON values (AGENTS.md,
  changed in its own commit).

## Decisions

- Compact JSON: the agent reads it as data; the examples stay readable because they are compared as values.
- A `json response` is not compared byte for byte; the order of the keys is checked by unit tests of the renderer.

## Tests

- examples E78.1–E78.14 (fixture `json-out`, and `levels`, `encodings`, `timeout` of earlier tasks)
- unit: the JSON of a file, of fragments (columns, a multiline one, escape levels), of warnings, of an error; the
  request field

## Steps

1. failing tests (the harness and the request field)
2. key, JSON renderer, errors, engine

## Risks

- the JSON paths follow the native separators like the text; Windows is checked by task 120
