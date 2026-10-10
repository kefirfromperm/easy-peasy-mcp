# Summary of task 57: `traversal.archives`: `none` and `compressed`

Server version: `0.57.0`.

## What was built

- `traversal.archives` (CONFIG.md#traversal, CONFIG.md#archives), default `"none"`, one of `none`, `compressed`,
  `all`: from the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced
  for one request by `traversal.archives`; invalid values like every key with a fixed set of values.
- `none`: compressed files are files like any other (binary by their zero bytes).
- `compressed`: a file named `*.gz` / `*.tgz`, `*.bz2`, `*.xz`, `*.zst`, `*.lz4` (any case) whose content is needed
  is decompressed in memory and searched as its decompressed content; it keeps its path and name, `size` stays the
  size on disk. `max_file_size` applies to the decompressed content (`too_big`); the content of a `.tgz` is a tar
  archive, which is binary; a file which can't be decompressed is not searched by content and gets the new
  `archive_error` warning (spec clarified in its own commit). `binary = exclude` decides by the decompressed start.
- `all` is valid for the spec and fails with `not_implemented` (`the value 'all' of 'traversal.archives' is not
  implemented yet`) in the config file, `update_config` and a request, until task 58.

## Architecture

- `src/compressed.rs` (new): `Compression` by the extension, `decompress` with a limit, `head` for the binary check.
- `src/search/mod.rs`: `Archives`, `SearchOptions.archives`; `Walk::read_content` reads, decompresses and applies
  `max_file_size` (`ReadContent`), `Walk::starts_binary` decompresses the start of a compressed file.
- `src/warning.rs`: `WarningCode::ArchiveError`.
- `src/config/keys.rs`: `Key::Archives`, `Key::unimplemented` for a value of the spec which comes later.

## Dependencies

Pure Rust, so the server builds without a C compiler on every platform: `flate2`, `bzip2` (backend
`libbz2-rs-sys`), `lzma-rust2` (`xz`), `ruzstd`, `lz4_flex` (`frame`).

## Deviations from the plan

- The task file asks for `invalid_value` / `invalid_option` for `all`; AGENTS.md asks for `not_implemented` for a
  valid feature of the spec which is not implemented yet, so it is `not_implemented` (no example shows it).
- The `get_config` examples of tasks 35–56 show the key (separate commit).

## Tests

- examples E57.1–E57.8 (fixture `compressed`; the LZ4 frame is written by hand with an uncompressed block)
- unit: `compressed::tests` (the extensions, every format, the limit, broken data), the `all` value in
  `config::tests` and `request::tests::traversal`

## Limitations and known gaps

- Archives with members (`all`) come with task 58, `max_archive_size` with task 59.
- The markers of `kind = generated` are looked for in the compressed bytes, not in the decompressed content.

## Try it

`{ "query": "name ~ '*.log.gz' and text has 'req-7f3a9c'", "traversal": { "archives": "compressed" } }`
