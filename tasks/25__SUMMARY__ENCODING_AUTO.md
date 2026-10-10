# Summary of task 25: Encoding detection

Server version: `0.25.0`.

## What was built

- `encoding = auto` (CONFIG.md#encoding): the BOM of UTF-8, UTF-16 LE / BE and UTF-32 LE / BE; UTF-16 without a BOM
  by the zero byte pattern of the first 8K (the rule is now in the spec); otherwise UTF-8.
- The BOM and UTF-16 checks run before the binary detection, so UTF-16 text is not binary.
- Invalid UTF-8 bytes are replaced with `�`, the file gets `decode_errors`.
- The encoding of a file which is not UTF-8 is shown after its counts (`utf-16le`, ...); columns count characters of
  the decoded text, the output is UTF-8.
- Fixture setups can write raw bytes (`bytes <path> <hex>`).

## Architecture

`content::decode` → `Decoded::Binary | Text { content, encoding, errors }`; UTF-16 with `encoding_rs`, UTF-32 by
hand; `Found.encoding`; `WarningCode::DecodeErrors`.

## Deviations from the plan

None.

## Tests

- examples E25.1–E25.2 (fixture `encodings`)
- unit: every BOM, UTF-16 without a BOM, binary detection after it, invalid UTF-8 and UTF-32

## Limitations and known gaps

- other encodings: `traversal.encoding` (task 55) and `fallback_encoding` (task 56); JSON `encoding` (task 78)

## Try it

`{ "query": "text has 'hello'" }` on UTF-16 files
