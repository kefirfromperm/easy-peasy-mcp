# Plan for task 65: `kinds.generated`

## Design

- `src/config/keys.rs`: `Key::KindsGenerated`, `Type::Wildcards` (checked like `exclude`), default the built-in list.
- `src/engine.rs`: `KindOptions.generated` from the settings; the wildcards are compiled with the case sensitivity of the
  paths, like `path ~`.
- `src/description.rs`: the key in the `kinds` object of the input schema.

## Tests

- examples E65.1–E65.8 (fixture `gen-paths`)
- unit: the request field

## Steps

1. failing tests
2. key, kind options

## Risks

- none
