# Plan for task 55: `traversal.encoding`

## Design

- `src/config/keys.rs`: `Key::Encoding`, `Type::Encoding` — `"auto"` or a WHATWG label (`encoding_rs::Encoding::for_label`,
  any case, not the `replacement` encoding), kept as written; default `"auto"`.
- `src/content.rs`:
  - `Encoding` gets `Other(&'static encoding_rs::Encoding)` for the encodings beyond UTF-8, UTF-16 and UTF-32;
    `name()` is the WHATWG name in lower case.
  - `DecodeOptions { binary_as_text, encoding }`: `decode` and `is_binary_content` take them. With an encoding a
    file without a BOM is decoded with it: no UTF-16 pattern check, binary by a zero byte unless the encoding is
    UTF-16.
- `src/search/mod.rs`: `SearchOptions.decode` instead of the binary flag of `decode`.
- `src/engine.rs`: the setting → the `encoding_rs` encoding, `None` for `auto`.

## Tests

- examples E55.1–E55.8 (fixture `legacy`, made by the setup: UTF-8 with a BOM, Windows-1252, UTF-8 without a BOM)
- unit: `decode` with an encoding (BOM first, a legacy encoding, UTF-16 without the binary check, the binary check
  of other encodings), the encoding type, the request field

## Steps

1. failing tests
2. key and type, decoding, walker

## Risks

- none: `encoding_rs` is a dependency since task 25
