# Summary of task 101: `output.cursor`

Server version: `0.101.0`.

## What was built

- `output.cursor` (OUTPUT.md#pagination): a request field only, a string (`'output.cursor' must be a string`); the
  unknown field hint of `output` lists it; `update_config` doesn't know it.
- A response cut by `max_files`, `max_lines`, `max_matches` or `max_output` and not stopped by the timeout has the
  cursor of its next page: `next cursor: …` in the text header after `truncated: yes (...)`, `next_cursor` in JSON
  after `truncated_by`. The per file limits give no cursor.
- The next page runs the search again and starts at the first file which was not listed, or inside the file whose
  lines were cut at its first fragment which was not listed; it has the totals of the whole result and a new result
  id.
- The cursor is opaque: base64url without padding of `{"v":1,"h":<hash>,"f":<file>,"m":<fragment>,"t":[files, lines,
  matches]}`, the hash FNV-1a 32 of the arguments without `output.cursor` (compact JSON, sorted keys); no paths.
- `cursor_invalid` (`the cursor can't be decoded`), `cursor_mismatch` (`the cursor belongs to another request`), and
  the warning `results_changed` when the totals differ from the cursor's.

## Architecture

- `src/cursor.rs` (new): `Cursor::encode` / `decode`, `request_hash`, base64url.
- `src/output/listing.rs`: `Listing.next`, `first_left_out`, the page start inside the first file.
- `src/output/mod.rs`, `src/output/json.rs`: `Page` in the response (`page_start`, `files_cut`, `next_cursor`).
- `src/engine.rs`: `page` checks the cursor, `totals_of` for `results_changed`; `src/request.rs`: `Request.cursor`,
  `sections` split from `parse`; `src/error.rs`, `src/warning.rs`, `src/description.rs`.

## Deviations from the plan

- OUTPUT.md says when a cursor is given, where the next page starts and the error texts (separate commit); the
  examples of tasks 94–97 and 100 show the next cursors (a commit per task).

## Tests

- examples E101.1–E101.9 (fixtures `sorting`, `detail`, `output-limits`)
- unit: `cursor::tests::encoded_and_decoded`, `output::listing::tests::where_the_next_page_starts`,
  `request::tests::cursor`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "detail": "files", "max_files": 20 } }`, then the same request with
`"cursor"` set to `next_cursor`.
