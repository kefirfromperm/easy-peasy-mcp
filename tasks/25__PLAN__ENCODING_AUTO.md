# Plan for task 25: Encoding detection

## Design

- `content.rs`: `decode(bytes) -> Decoded` with `encoding = auto` (CONFIG.md#encoding):
  1. BOM: `EF BB BF` UTF-8, `FF FE 00 00` UTF-32 LE, `00 00 FE FF` UTF-32 BE, `FF FE` UTF-16 LE, `FE FF` UTF-16 BE;
     the BOM is not part of the text
  2. no BOM: UTF-16 LE / BE by the zero byte pattern of the first 8K (written into the spec)
  3. otherwise binary if a zero byte is in the first 8K
  4. otherwise UTF-8; invalid bytes are replaced with `�` and reported
  `Decoded::Binary` or `Decoded::Text { content, encoding, errors }`; UTF-16 with `encoding_rs`, UTF-32 decoded by hand
  (invalid code points replaced).
- `Encoding` names as in the output: `utf-8`, `utf-16le`, `utf-16be`, `utf-32le`, `utf-32be`.
- `search`: the decoding replaces `is_binary` + `from_bytes`; `decode_errors` warning; `Found.encoding` for files whose
  content was read in an encoding other than UTF-8.
- `output`: the encoding after the counts (OUTPUT.md#text-format).
- `warning.rs`: `decode_errors` after `binary_skipped`.

## Tests

- examples E25.1–E25.2 (fixture `encodings`, raw bytes from the setup)
- unit: every BOM, UTF-16 without a BOM (LE, BE), binary after the BOM check, invalid UTF-8, invalid UTF-32

## Steps

1. failing unit tests
2. decoding, search, output, warning

## Risks

- the zero byte heuristic can take a binary file with a regular pattern for UTF-16; the rule is strict (no zero at
  the other positions)
