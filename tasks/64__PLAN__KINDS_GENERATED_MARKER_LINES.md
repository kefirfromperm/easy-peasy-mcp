# Plan for task 64: `kinds.generated_marker_lines`

## Design

- `src/config/keys.rs`: `Key::GeneratedMarkerLines`, `Type::AtLeast(1)` — a whole number with a minimum, `a whole
  number of at least 1 is expected` / `must be a whole number of at least 1`; the later keys with a minimum use it.
  Default `5`.
- `src/engine.rs`: `KindOptions.generated_marker_lines` from the settings.

## Tests

- examples E64.1–E64.7 (fixture `marker-lines`: a marker on the third line)
- unit: the type with a minimum, the request field

## Steps

1. failing tests
2. key and type, the kind options

## Risks

- none
