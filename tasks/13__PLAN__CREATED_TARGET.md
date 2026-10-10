# Plan for task 13: `created` target

## Design

- `query/eval.rs`: three-valued logic. `Truth { False, Unknown, True }`; `Filter::eval` returns it with the table
  of QUERY.md#files-whose-content-is-not-searched: `not unknown` = unknown, `false and unknown` = false,
  `true or unknown` = true, `xor` with unknown = unknown. A file matches only when the whole query is `True`; the
  contributions take only `True` parts of `or` / `xor`.
- `FileValues.created: Option<f64>` (`None` when the file system has no creation date); `NumberTarget::Created`;
  a number condition on a missing value is `Unknown`. `modified` becomes `Option<f64>` too.
- `query/mod.rs`: `created` uses the date validation and compilation of `modified`; `Query.needs_created`.
- `warning.rs` (new): `WarningCode` (in the order of OUTPUT.md#warnings) and `Warnings`, a collector of entries per
  code; the text lines `warning: <code> (<count>): <3 paths>, … N more` after an empty line (spec clarified).
- `search`: returns `SearchResult { files, warnings }`; an entry evaluated by a query with `created` but without a
  creation date gets `date_unsupported`.
- `output`: the warnings after the files (or after the header of an empty result).
- `description.rs`: `created`.

## Tests

- examples E13.1–E13.6 (creation dates of the fixture are the time of the test run, the conditions are true for
  any of them)
- unit: the truth tables of `not`, `and`, `or`, `xor` with unknown; a `created` condition without a creation date
  is unknown, `not` of it too, `or` with a true part matches; contributions skip unknown parts; warning lines with
  1, 3 and 5 entries

## Steps

1. failing unit tests
2. three-valued evaluation, created, warnings, output
3. description

## Risks

- a file system without creation dates (some Linux file systems) makes the examples fail; tmpfs and ext4 have them
