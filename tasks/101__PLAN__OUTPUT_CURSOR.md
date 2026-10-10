# Plan for task 101: `output.cursor`

## Design

- `src/request.rs`: `Request.cursor` — `output.cursor`, a string, not a configuration key; the unknown field hint of
  `output` lists it last.
- `src/cursor.rs` (new): `Cursor { hash, file, fragment, totals }`; `encode` — base64url without padding of
  `{"v":1,"h":…,"f":…,"m":…,"t":[…]}` (`m` left out when 0); `decode` — `cursor_invalid`; `request_hash` — FNV-1a
  32 of the arguments without `output.cursor` as compact JSON with sorted keys, 8 hex digits.
- `src/output/listing.rs`: `Listing.next` — the first file (and the index of its first fragment) which a cursor limit
  (`max_lines`, `max_matches`, `max_output`) left out; the listing starts with the fragments of the first file from
  the page start.
- `src/output/mod.rs`, `src/output/json.rs`: `Response.page` (the start file and fragment, the request hash); the
  listed files start there; `next cursor: …` / `next_cursor` when a cursor limit cut and the search finished
  (OUTPUT.md#pagination, clarified in its own commit).
- `src/engine.rs`: the cursor is checked (`cursor_mismatch`) and its totals compared (`results_changed`, a new
  warning after `timeout`); `src/error.rs`: `CursorMismatch`, `CursorInvalid`; `src/warning.rs`: `ResultsChanged`.
- `src/description.rs`: the schema and the description of `cursor`.

## Tests

- examples E101.1–E101.9 (fixtures `sorting`, `detail`, `output-limits`)
- unit: encode / decode, the hash, the next position of the listing, the request field

## Steps

1. failing tests
2. the cursor, the pages, the errors and the warning

## Risks

- none
