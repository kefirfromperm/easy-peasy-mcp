# Plan for task 53: `traversal.binary`

## Design

- `src/config/keys.rs`: `Key::Binary`, `Type::Choice(["skip", "search", "exclude"])`, default `"skip"`.
- `src/content.rs`: `decode(bytes, binary_as_text)` — with `search` a file with a zero byte in its first 8K is
  decoded as UTF-8 like any other file without a BOM; `is_binary_content(bytes)` — the decision of `decode` without
  decoding: a BOM or the UTF-16 pattern of zero bytes makes a file text, otherwise a zero byte in the first 8K binary.
- `src/search/mod.rs`: `SearchOptions.binary` (`Binary::Skip`, `Search`, `Exclude`).
  - `skip`: as before — the content of a binary file is unknown, `binary_skipped`.
  - `search`: `decode` doesn't stop at binary files.
  - `exclude`: a binary file is never returned and gets no warning; a file whose content was read is binary if
    `decode` said so; a regular file which matches without its content is checked by its first 8K before it is
    returned.
- `src/engine.rs`: the setting → `Binary`.

## Tests

- examples E53.1–E53.8 (fixture `binary-files`: `data.bin` with zero bytes and a line of text)
- unit: `decode` with `binary_as_text`, `is_binary_content` (zero bytes, a BOM, UTF-16 without a BOM), the request
  field

## Steps

1. failing tests
2. key, decode, walker

## Risks

- `exclude` reads the first 8K of every returned regular file; only with `exclude`
