# Plan for task 60: `traversal.skip_generated`

## Design

- `src/config/keys.rs`: `Key::SkipGenerated`, `Type::Bool`, default `true`.
- `src/search/mod.rs`: `SearchOptions.skip_generated`; files and members are skipped as generated only with it and
  without a `kind` condition in the query.

## Tests

- examples E60.1–E60.8 (fixture `generated`: a lock file, a file with the marker, a source file)
- unit: the request field

## Steps

1. failing tests
2. key, walker

## Risks

- none
