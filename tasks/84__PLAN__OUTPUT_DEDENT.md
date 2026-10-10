# Plan for task 84: `output.dedent`

## Design

- `src/config/keys.rs`: `Key::Dedent`, `Type::Bool`, default `true`.
- `src/output/blocks.rs`: `BlockOptions.dedent`, `Block.indent` — the longest common prefix of spaces and tabs of the
  lines of a block which are not empty or whitespace-only, cut back to the start of the first fragment which starts
  in it (OUTPUT.md#dedent, clarified in its own commit).
- `src/output/lines.rs`: `⇥N` before a block with an indent, the lines without their first `N` characters (a blank
  line shorter than that is empty), the markers as before.
- `src/output/json.rs`: `indent` of a block (left out when 0), the dedented `text`, `col` of the original line.

## Tests

- examples E84.1–E84.10 (fixture `dedent`: indented lines, a match in the indentation); E30.1 and E30.2 are fixed in
  their own commit: their block of indented lines has `⇥4` now
- unit: the indent of a block (common prefix, blank lines, a fragment in the indentation), the text and JSON

## Steps

1. failing tests
2. key, the indent of blocks, the text and JSON

## Risks

- none
