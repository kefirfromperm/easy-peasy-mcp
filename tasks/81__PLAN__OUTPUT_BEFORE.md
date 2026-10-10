# Plan for task 81: `output.before`

## Design

- `src/config/keys.rs`: `Key::Before`, `Type::OptWhole`, not set by default (written `# before =  (not set)`).
- `src/engine.rs`: the lines before a matched line are `before` when it is set, otherwise `context`.

## Tests

- examples E81.1–E81.8 (fixture `context` of task 80)
- unit: the request field

## Steps

1. failing tests
2. key, the block options

## Risks

- none
