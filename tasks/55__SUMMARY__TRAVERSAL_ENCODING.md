# Summary of task 55: `traversal.encoding`

Server version: `0.55.0`.

## What was built

- `traversal.encoding` (CONFIG.md#traversal, CONFIG.md#encoding), default `"auto"`: `auto` or a WHATWG label in any
  case, kept as written; from the config file, shown by `get_config` with its source, changed and reset by
  `update_config`, replaced for one request by `traversal.encoding`.
- With an encoding every file without a BOM is decoded with it, files with a BOM still by their BOM. The output
  shows the encoding of a file which is not UTF-8 by its WHATWG name in lower case (`Latin1` → `windows-1252`).
- With an encoding there is no UTF-16 detection by the pattern of zero bytes; a zero byte in the first 8K makes a
  file binary unless the encoding is `utf-16le` or `utf-16be` (spec clarified in its own commit).
- Errors (spec added in the same commit): `invalid value for 'traversal.encoding': "auto" or an encoding name like
  "windows-1252" is expected` (file, `update_config` hint), in a request `'traversal.encoding' must be "auto" or an
  encoding name like "windows-1252"`. The `replacement` encoding of the WHATWG list is not accepted.

## Architecture

- `src/content.rs`: `Encoding::Other` for the encodings beyond UTF-8, UTF-16 and UTF-32, `Encoding::of`, `name()`
  in lower case; `DecodeOptions { binary_as_text, encoding }` for `decode` and `is_binary_content`; `without_bom`
  decides the encoding of a file without a BOM or that it is binary.
- `src/config/keys.rs`: `Key::Encoding`, `Type::Encoding`, `encoding(label)`.
- `src/search/mod.rs`: `SearchOptions.decode`.

## Deviations from the plan

- The `get_config` examples of tasks 35–54 show the key (separate commit).

## Tests

- examples E55.1–E55.8 (fixture `legacy`, made by the setup)
- unit: `content::tests::a_forced_encoding_for_files_without_a_bom` and the earlier decoding tests,
  `request::tests::traversal`

## Limitations and known gaps

- `fallback_encoding` comes with task 56.

## Try it

`{ "query": "text has 'café'", "traversal": { "encoding": "windows-1252" } }`
