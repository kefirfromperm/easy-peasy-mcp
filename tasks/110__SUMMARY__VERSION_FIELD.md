# Summary of task 110: the `version` field

Server version: `0.110.0`.

## What was built

- `version` in the `find` request (TOOLS.md#versioning): a whole number of at least 1; `1` and a missing field use
  spec 1. Another version is `unsupported_version`: `the spec version 2 is not supported`, hint `supported spec
  versions: 1`; a value which is not a whole number of at least 1 is `invalid_option`: `'version' must be a whole
  number of at least 1`.
- Every JSON response has `"version": 1` (since task 78), the text format doesn't.

## Architecture

- `src/request.rs`: `check_version`, `SPEC_VERSIONS`; the field is implemented. `src/error.rs`:
  `ErrorCode::UnsupportedVersion`. `src/description.rs`: `version` in the input schema.

## Deviations from the plan

- TOOLS.md got the error texts (separate commit).

## Tests

- examples E110.1–E110.4 (fixture `detail` of task 79)
- unit: `request::tests::version`

## Limitations and known gaps

None.

## Try it

`{ "version": 1, "query": "text has 'TODO'" }`
