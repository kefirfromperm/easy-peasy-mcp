# Plan for task 20: Binary files and unknown content

## Design

- `content.rs`: `is_binary(bytes)` — a zero byte in the first 8K (the BOM and UTF-16 checks come with task 25).
- `query/eval.rs`: `ContentState::Unknown` — the content is not searched; content conditions are unknown, so the
  three-valued logic of task 13 never returns the file because of its content, but other conditions still can.
- `search`: when the search needs the content of a file (the evaluation without content isn't false) and the file is
  binary, the content is `Unknown` and the file gets `binary_skipped`. A binary file which the metadata conditions
  exclude is not read and has no warning.
- `warning.rs`: `binary_skipped` in the order of OUTPUT.md#warnings (before `date_unsupported`); the text format of
  task 13 (JSON with `output.format`, task 78).

## Tests

- examples E20.1–E20.4 (fixture `binary`)
- unit: `is_binary` with a zero byte at the start, at 8191, at 8192 (not binary); content conditions under
  `ContentState::Unknown`; warning order

## Steps

1. failing unit tests
2. detection, content state, warning

## Risks

None known.
