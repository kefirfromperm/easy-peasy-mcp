# Plan for task 83: `output.blank_context`

## Design

- `src/config/keys.rs`: `Key::BlankContext`, `Type::Bool`, default `false`.
- `src/output/blocks.rs`: `BlockOptions.blank_context`; without it the empty and whitespace-only context lines of a
  block are left out after the blocks are made, so they still join blocks and the line numbers show the gap.
- `src/engine.rs`: the setting → the block options.

## Tests

- examples E83.1–E83.8 (fixture `blank`: an empty and a whitespace-only line around a match)
- unit: blocks with and without blank context lines, the request field

## Steps

1. failing tests
2. key, blocks

## Risks

- none
