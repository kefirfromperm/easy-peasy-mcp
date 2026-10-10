# Plan for task 62: `query.fuzzy_min_score`

## Design

- `src/config/keys.rs`: `Key::FuzzyMinScore` in the section `query`, `Type::Score` — a number (float or whole) from 0
  to 1, written as it is (`0.5`, `1`); `Value::Number`. The request object `query_options` holds the keys of `query`.
- `src/query/mod.rs`: `ParseOptions.fuzzy_min_score` replaces `fuzzy::MIN_SCORE` for `~~`.
- `src/engine.rs`: the setting → the parse options.
- `src/description.rs`: the `query_options` object in the input schema.

## Tests

- examples E62.1–E62.8 (fixture `fuzzy` of task 32)
- unit: the number type in TOML and JSON, the request field

## Steps

1. failing tests
2. key and type, parse options

## Risks

- none
