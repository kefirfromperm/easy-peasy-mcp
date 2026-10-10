# Plan for task 67: `kinds.doc`

## Design

- `src/config/keys.rs`: `Key::KindsDoc`, `Type::Wildcards` (checked like `exclude`), default the built-in list.
- `src/engine.rs`: `KindOptions.doc` from the settings; the wildcards are compiled with the case sensitivity of the
  paths, like `path ~`.
- `src/description.rs`: the key in the `kinds` object of the input schema.

## Tests

- examples E67.1–E67.8 (fixture `doc-paths`)
- unit: the request field

## Steps

1. failing tests
2. key, kind options

## Risks

- none
