# Plan for task 68: `kinds.config`

## Design

- `src/config/keys.rs`: `Key::KindsConfig`, `Type::Wildcards` (checked like `exclude`), default the built-in list.
- `src/engine.rs`: `KindOptions.config` from the settings; the wildcards are compiled with the case sensitivity of the
  paths, like `path ~`.
- `src/description.rs`: the key in the `kinds` object of the input schema.

## Tests

- examples E68.1–E68.8 (fixture `config-paths`)
- unit: the request field

## Steps

1. failing tests
2. key, kind options

## Risks

- none
