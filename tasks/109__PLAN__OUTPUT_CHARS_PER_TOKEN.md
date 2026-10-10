# Plan for task 109: `output.chars_per_token`

## Design

- `src/config/keys.rs`: `Type::Positive` — a number above 0, with a fraction or not; `Key::CharsPerToken` after
  `Budget`, default `4` (written `4`, `3.5` with a fraction).
- `src/engine.rs`: the budget estimate with the key instead of the constant `CHARS_PER_TOKEN`; `src/description.rs`.
- the tests which used `output.chars_per_token` as the key which is not implemented yet: every key of the spec is
  implemented now, so they check the path with a key which isn't in the spec's sections any more (an unknown key)
  or go.

## Tests

- examples E109.1–E109.8 (fixture `huge` of task 104)
- unit: the type, the request field

## Steps

1. failing tests
2. the type and key, the estimate

## Risks

- none
