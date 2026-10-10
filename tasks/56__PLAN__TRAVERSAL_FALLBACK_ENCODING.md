# Plan for task 56: `traversal.fallback_encoding`

## Design

- `src/config/keys.rs`: `Key::FallbackEncoding`, `Type::OptEncoding` — a WHATWG label (not `auto`) or not set,
  kept as written; default not set, unset by `reset`.
- `src/content.rs`: `DecodeOptions.fallback`; with `auto` a file without a BOM, not UTF-16 and not binary which is
  not valid UTF-8 is decoded with the fallback: its encoding is the fallback, `decode_errors` only for bytes invalid
  in the fallback.
- `src/engine.rs`: the setting → the `encoding_rs` encoding.

## Tests

- examples E56.1–E56.8 (fixture `legacy` of task 55)
- unit: `decode` with a fallback (valid UTF-8 stays UTF-8, invalid UTF-8 decoded with the fallback, ignored with an
  encoding), the request field

## Steps

1. failing tests
2. key and type, decoding

## Risks

- none
