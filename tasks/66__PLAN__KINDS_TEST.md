# Plan for task 66: `kinds.test`

## Design

- `src/config/keys.rs`: `Key::KindsTest`, `Type::Wildcards` (checked like `exclude`), default the built-in list.
- `src/engine.rs`: `KindOptions.test` from the settings; the wildcards are compiled with the case sensitivity of the
  paths, like `path ~`.
- `src/description.rs`: the key in the `kinds` object of the input schema.

## Tests

- examples E66.1–E66.8 (fixture `test-paths`)
- unit: the request field

## Steps

1. failing tests
2. key, kind options

## Risks

- none
