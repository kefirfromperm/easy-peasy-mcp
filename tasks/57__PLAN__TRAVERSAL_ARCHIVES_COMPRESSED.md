# Plan for task 57: `traversal.archives`: `none` and `compressed`

## Design

- `src/config/keys.rs`: `Key::Archives`, `Type::Choice(["none", "compressed", "all"])`, default `"none"`; the value
  `all` is valid for the spec but fails with `not_implemented` until task 58 (`Key::unimplemented`, checked by the
  config file, `update_config` and the request).
- `src/compressed.rs` (new): `Compression` of a file name by its extension in any case — `.gz` / `.tgz` gzip, `.bz2`,
  `.xz`, `.zst`, `.lz4` (frame format); `decompress(compression, reader, limit)` in memory: the bytes, `TooBig` when
  the decompressed content exceeds the limit, or an error.
- `src/search/mod.rs`: `SearchOptions.archives` (`Archives::None`, `Compressed`). When the content of a compressed
  file is needed with `compressed`, it is decompressed with `max_file_size` as the limit instead of the size check on
  disk: too big — `too_big`, broken — `archive_error`, both with unknown content; the decompressed bytes go through
  `decode` like any file (the tar inside a `.tgz` is binary). `binary = exclude` checks the decompressed start.
- `src/warning.rs`: `WarningCode::ArchiveError` in the order of the table.
- `size` stays the size on disk; path and name are the file's.

## Dependencies

All pure Rust, so the server builds on every platform without a C compiler:

- `flate2` — gzip (`MultiGzDecoder`)
- `bzip2` — bzip2, its default backend `libbz2-rs-sys` is pure Rust
- `lzma-rust2` (feature `xz`) — xz
- `ruzstd` — zstd decoder
- `lz4_flex` (feature `frame`) — LZ4 frames

## Tests

- examples E57.1–E57.8 (fixture `compressed`: one file per format, a `.tgz`, a broken `.gz`, a small `.gz` with a big
  content)
- unit: the compression of names, decompression of every format, the limit, a broken stream; the `all` value is
  `not_implemented`; the request field

## Steps

1. failing tests
2. key, warning, decompression, walker

## Risks

- the LZ4 fixture is written by hand (an uncompressed block in a frame); its header checksum is checked by the decoder
