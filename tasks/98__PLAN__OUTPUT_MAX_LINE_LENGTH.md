# Plan for task 98: `output.max_line_length`

## Design

- `src/config/keys.rs`: `Key::MaxLineLength` after `MaxMatchesPerFile`, `Type::AtLeast(20)`, default `300`.
- `src/output/lines.rs`: `windows` — the byte ranges of a line shown after the cut (OUTPUT.md#long-lines, clarified
  in its own commit): the merged parts of the fragments in the line with `(max − their characters) / (2 × parts)`
  characters on each side, windows which touch merged; the first `max` characters of a context line; `cut_line` —
  the shown text with `…` and the markers of the style; `render` cuts every line of a block.
- `src/output/json.rs`: `"cut": true` before `text`; the repeated lines are not cut.
- `src/output/mod.rs`: `Response.max_line_length`; `src/engine.rs`, `src/description.rs`.

## Tests

- examples E98.1–E98.10 (fixture `long-lines`)
- unit: the windows (one fragment, several, a context line, a short line, after dedent), the request field

## Steps

1. failing tests
2. key, the windows, the cut lines in both formats

## Risks

- none
