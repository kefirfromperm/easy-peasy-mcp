# Summary of task 53: `traversal.binary`

Server version: `0.53.0`.

## What was built

- `traversal.binary` (CONFIG.md#traversal), default `"skip"`, one of `skip`, `search`, `exclude`: from the config
  file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `traversal.binary`; invalid values like every key with a fixed set of values.
- `skip`: as since task 20 — the content of a binary file is unknown (QUERY.md#files-whose-content-is-not-searched),
  `binary_skipped`.
- `search`: a binary file is decoded as UTF-8 like any file without a BOM, its zero bytes stay in the text.
- `exclude`: a binary file is never returned, like `grep -I`, without a warning — also by conditions which don't
  read the content (`type = file`, `name ~ ...`).
- A file is binary after the BOM and UTF-16 checks: a zero byte in its first 8K (CONFIG.md#traversal).

## Architecture

- `src/content.rs`: `decode(bytes, binary_as_text)`, `is_binary_content` (the decision of `decode` without
  decoding), `by_bom`.
- `src/search/mod.rs`: `Binary`, `SearchOptions.binary`; `evaluate` returns `Evaluated` (with whether the content
  was decoded as text); `visit` checks the first 8K of a returned regular file with `exclude` when its content was
  not read (`starts_binary`).
- `src/config/keys.rs`: `Key::Binary`.

## Deviations from the plan

- The `get_config` examples of tasks 35–52 show the key (separate commit).

## Tests

- examples E53.1–E53.8 (fixture `binary-files`)
- unit: `content::tests::binary_files_searched_as_text`,
  `content::tests::binary_content_after_the_bom_and_utf16_checks`, `request::tests::traversal`

## Limitations and known gaps

- Archives are binary files until `traversal.archives` (tasks 57–58).

## Try it

`{ "query": "text has 'PNG'", "traversal": { "binary": "search" } }`
