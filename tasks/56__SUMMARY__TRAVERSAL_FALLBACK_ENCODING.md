# Summary of task 56: `traversal.fallback_encoding`

Server version: `0.56.0`.

## What was built

- `traversal.fallback_encoding` (CONFIG.md#traversal, CONFIG.md#encoding), default not set: a WHATWG label, kept as
  written; from the config file, shown by `get_config` (`# fallback_encoding =  (not set)`) with its source,
  changed by `update_config` and unset by its `reset`, replaced for one request by `traversal.fallback_encoding`.
- Only with `encoding = auto`, only for a file without a BOM which is not UTF-16 or binary and not valid UTF-8: it is
  decoded with the fallback instead of replacing the invalid bytes, so it has no `decode_errors` warning (unless its
  bytes are invalid in the fallback too); the output shows the fallback as the encoding of the file.
- Errors (spec added in its own commit): `invalid value for 'traversal.fallback_encoding': an encoding name like
  "windows-1251" is expected` (file, `update_config` hint), in a request `'traversal.fallback_encoding' must be an
  encoding name like "windows-1251"`; `auto` is not an encoding name here.

## Architecture

- `src/config/keys.rs`: `Key::FallbackEncoding`, `Type::OptEncoding`.
- `src/content.rs`: `DecodeOptions.fallback`, used by `decode` after the BOM, UTF-16 and binary checks.
- `src/engine.rs`: the setting → the `encoding_rs` encoding.

## Deviations from the plan

- The `get_config` examples of tasks 35–55 show the key (separate commit).

## Tests

- examples E56.1–E56.8 (fixture `legacy` of task 55)
- unit: `content::tests::a_fallback_for_invalid_utf8`, `request::tests::traversal`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'привет'", "traversal": { "fallback_encoding": "windows-1251" } }`
