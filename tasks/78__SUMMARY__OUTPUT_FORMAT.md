# Summary of task 78: `output.format`

Server version: `0.78.0`.

## What was built

- `output.format` (CONFIG.md#output, OUTPUT.md#options), default `"text"`, one of `text`, `json`: from the config
  file, shown by `get_config` (a new section `[output]`) with its source, changed and reset by `update_config`,
  replaced for one request by the new request object `output`.
- `json`: the JSON document of OUTPUT.md#json-format in the text content item — `version`, `exists`, `result`,
  `counts`, `counts_exact`, `truncated`, `truncated_by`, `legend` (in the order of the query), `warnings`
  (`{code, count, paths}`, `{code}` for a warning without entries), `files` with the full `path`, `encoding`,
  `lines`, `matches`, `conditions` and `blocks` of consecutive matched lines with `n`, `text` and `fragments`
  (`col` in characters, 1-based, end exclusive; `text`, `ids`, `escape`, and `to` for a multiline fragment, which is
  in its first line with `col` to the end of that line). Empty and default values are left out; the document has no
  whitespace between its tokens.
- Errors are JSON with the format of the request, or of the session when the request has none: `{"error": {code,
  message, query, position: {col, len}, hint}}`; an invalid `output.format` itself is a text error.
- The spec was clarified in its own commit (the multiline `col`, the omitted fields, the layout); AGENTS.md: the
  `json response` and `json error` blocks of examples are compared as JSON values.

## Architecture

- `src/output/json.rs` (new): serde structs in the order of the fields table, `Legend` as an ordered map, `blocks`.
- `src/output/mod.rs`: `Response::totals`, `display_found`.
- `src/warning.rs`: `Warnings::groups`, shared by text and JSON.
- `src/error.rs`: `Report::to_json`.
- `src/engine.rs`: `json_format` of a request; the response and the error in that format.
- `src/config/keys.rs`: `Key::Format`; `src/description.rs`: the `output` object.
- `tests/common/mod.rs`: `same_json`; `tests/examples.rs`, `tests/startup.rs`: JSON blocks.

## Deviations from the plan

- The full `get_config` examples of tasks 35–43 show the new section (separate commit).
- E78.3 was split in its own commit: a directory and files in one result would be ordered by relevance (task 93).
- The config unit tests use `output.chars_per_token` (task 109) as the key which is not implemented yet.

## Tests

- examples E78.1–E78.14 (fixture `json-out`, and `levels`, `encodings`, `timeout` of earlier tasks)
- unit: `output::json::tests` (columns, a multiline fragment, blocks, the order of the fields),
  `request::tests::output`

## Limitations and known gaps

- `detail` comes with task 79; context, dedent, highlight, meta and the other output options with their tasks.

## Try it

`{ "query": "text has 'TODO'", "output": { "format": "json" } }`
