# Summary of task 20: Binary files and unknown content

Server version: `0.20.0`.

## What was built

- A file is binary when its first 8K contain a zero byte (CONFIG.md#traversal; the BOM and UTF-16 checks come with
  task 25).
- `binary = skip`, the default: content conditions are unknown for binary files (QUERY.md#files-whose-content-is-
  not-searched), so neither `text has 'x'` nor `not text has 'x'` returns them, but other conditions can
  (`name ~ '*.bin' or text has 'logo'`).
- `binary_skipped` warning for a binary file whose content the search needed; a binary file which the metadata
  conditions exclude is not read and has no warning.
- The `find` description says binary files are not searched.

## Architecture

`content::is_binary`, `ContentState::Unknown`; `Walk::evaluate` holds the two-phase evaluation and reading;
`WarningCode::BinarySkipped` in the order of the spec.

## Deviations from the plan

None.

## Tests

- examples E20.1–E20.4 (fixture `binary`)
- unit: `is_binary` at the borders of 8K, unknown content conditions, warning order

## Limitations and known gaps

- `traversal.binary` (task 53), `max_file_size` (task 54), JSON warnings (task 78)

## Try it

`{ "query": "name ~ '*.png' or text has 'logo'" }`
