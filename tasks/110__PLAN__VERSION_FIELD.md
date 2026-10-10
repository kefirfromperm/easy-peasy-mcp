# Plan for task 110: the `version` field

## Design

- `src/request.rs`: `version` — a whole number of at least 1 (`invalid_option` otherwise); the supported spec
  versions (`1`) pass, another one is `unsupported_version` with the supported list (TOOLS.md#versioning, clarified in
  its own commit); the field is implemented.
- `src/error.rs`: `ErrorCode::UnsupportedVersion`.
- JSON responses have `"version": 1` already; `src/description.rs`: the schema of `version`.

## Tests

- examples E110.1–E110.4 (fixture `detail` of task 79)
- unit: the request field

## Steps

1. failing tests
2. the field and the error

## Risks

- none
