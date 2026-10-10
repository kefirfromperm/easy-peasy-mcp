# Summary of task 54: `traversal.max_file_size`

Server version: `0.54.0`.

## What was built

- `traversal.max_file_size` (CONFIG.md#traversal), default `"10M"`: a size like the values of the `size` target,
  kept as written; from the config file, shown by `get_config` with its source, changed and reset by
  `update_config`, replaced for one request by `traversal.max_file_size`.
- A file bigger than the limit is not searched by content: when the conditions need its content it is not read,
  every content condition is unknown for it (QUERY.md#files-whose-content-is-not-searched), and the response has a
  `too_big` warning (OUTPUT.md#warnings); other conditions still return it.
- Errors (spec added in its own commit): `invalid value for 'traversal.max_file_size': a size like "10M" is expected
  (units B, K, M, G)` (file, `update_config` hint), in a request `'traversal.max_file_size' must be a size like "10M"
  (units B, K, M, G)`.

## Architecture

- `src/config/keys.rs`: `Key::MaxFileSize`, `Type::Size` (`query::parse_size`); `Settings::size` in bytes.
- `src/warning.rs`: `WarningCode::TooBig`, in the order of the table.
- `src/search/mod.rs`: `SearchOptions.max_file_size`; `Walk::too_big` before the content is read.

## Deviations from the plan

- The `get_config` examples of tasks 35–53 show the key (separate commit).

## Tests

- examples E54.1–E54.8 (fixture `big-files`)
- unit: `warning::tests::too_big_comes_after_permission_denied`, `request::tests::traversal`

## Limitations and known gaps

- The decompressed content of compressed files and archive members comes with tasks 57–58.

## Try it

`{ "query": "name ~ '*.log' and text has 'req-7f3a9c'", "traversal": { "max_file_size": "1G" } }`
