# Summary of task 13: `created` target

Server version: `0.13.0`.

## What was built

- `created` with the operators and date values of `modified` (QUERY.md#targets).
- Three-valued logic (QUERY.md#files-whose-content-is-not-searched): `not unknown` = unknown, `false and unknown` =
  false, `true or unknown` = true, `xor` with unknown = unknown; a file is returned only when the whole query is
  true, and only true parts of `or` / `xor` are listed as its conditions.
- Without a creation date `created` is unknown and the entry gets the warning `date_unsupported`.
- Warnings (OUTPUT.md#warnings, clarified): grouped by code in the order of the spec table, one line per code at
  the end after an empty line: `warning: <code> (<count>): <3 paths>, … N more`.
- The `find` description shows `created`.

## Architecture

- `query/eval.rs`: `Truth` with `not`, `and`, `or`, `xor`; `Filter::eval` returns it, `Filter::matches` is
  `eval == True`; `FileValues.modified` / `created` are `Option<f64>`; `NumberTarget::Created`.
- `Query.needs` (`Needs { empty, metadata, created }`) replaces the separate flags.
- `warning.rs`: `WarningCode`, `Warnings` (collector and text lines); `search` returns `SearchResult { files,
  warnings }`; `output::display_path` writes native paths with the trailing separator of directories.

## Deviations from the plan

None.

## Tests

- examples E13.1–E13.6 (fixture `flat`; conditions true for any creation date of the test run)
- unit: truth tables, a missing creation date is unknown (with `not`, `or`, `and`), contributions without unknown
  parts, warning lines with 1 and 6 entries

## Limitations and known gaps

- the examples need a file system with creation dates (tmpfs, ext4, APFS, NTFS have them)

## Try it

`{ "query": "created > -1d" }`
