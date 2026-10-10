# Plan for task 80: `output.context`

## Design

- `src/config/keys.rs`: `Key::Context`, `Type::Whole`, default `0`.
- `src/output/blocks.rs` (new): the blocks of a file — the matched lines with `before` lines before and `after` lines
  after them (both `context` until tasks 81–82), a block is a run of consecutive lines, so blocks whose context
  touches or overlaps are one; every line is matched or context. The text and the JSON renderers use the same blocks,
  and the later options (blank context lines, dedent, long lines) change them there.
- `src/output/lines.rs`: the text of the blocks: `N:` for a matched line, `N-` for a context line, `--` between
  blocks, the markers of the fragments as before.
- `src/output/json.rs`: the blocks with their context lines (`n`, `text`).
- `src/engine.rs`, `src/output/mod.rs`: the block options in the response.

## Tests

- examples E80.1–E80.9 (fixture `context`: two matches far apart)
- unit: blocks (no context, overlapping and touching context, the start and the end of the file), the request field

## Steps

1. failing tests
2. key, blocks, the text and JSON of blocks

## Risks

- none
