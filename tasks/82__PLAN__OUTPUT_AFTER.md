# Plan for task 82: `output.after`

## Design

- `src/config/keys.rs`: `Key::After`, `Type::OptWhole`, not set by default (written `# after =  (not set)`).
- `src/engine.rs`: the lines after a matched line are `after` when it is set, otherwise `context`.

## Tests

- examples E82.1–E82.8 (fixture `context` of task 80)
- unit: the request field

## Steps

1. failing tests
2. key, the block options

## Risks

- none
