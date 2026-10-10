# Plan for task 51: `traversal.exclude`

## Design

- `src/config/keys.rs`: `Key::Exclude`, `Type::Wildcards`, default `[]`. The checks of a type return the whole
  problem: `a list of wildcards is expected`, or `invalid wildcard '[a-': unclosed '['` for the first invalid one; in
  a request `'traversal.exclude' must be a list of strings`, or `invalid wildcard '[a-' in 'traversal.exclude'` with
  the hint.
- `src/request.rs`: the request error of a type comes from the type (a message and a hint).
- `src/engine.rs`: the wildcards compiled like `path ~` with the case sensitivity of the paths.
- `src/search/mod.rs`: `SearchOptions.exclude`; one `filter_entry` of the walker for the symlinks and the
  exclusions: an entry whose relative `/` path (NFC) matches a wildcard, a directory also with a `/` after its path,
  is skipped and not descended into.

## Tests

- examples E51.1–E51.11 (fixture `exclude`: `target` and `src` at the top and in `lib`, with directory times)
- unit: the type checks of wildcards (TOML and JSON), the exclusion of a path and of a directory

## Steps

1. failing tests
2. key and type, request errors
3. the walker

## Risks

- none
