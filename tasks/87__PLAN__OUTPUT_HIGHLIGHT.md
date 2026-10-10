# Plan for task 87: `output.highlight`

## Design

- `src/config/keys.rs`: `Key::Highlight` after `After`, `Type::Choice(&["auto", "none", "columns", "markers"])`,
  default `"auto"`.
- `src/output/mod.rs`: `Highlight { None, Columns, Markers }` (`auto` resolved by the format in the engine) and
  `Response.highlight`.
- `src/output/lines.rs`: the text lines with the style: `markers` as before, `none` without markers, `columns` the
  plain line and after it the line `  @ s..e ids  @ …` of the fragments which start in it (a multiline fragment ends
  with `line:col`), also after a repeated line.
- `src/output/json.rs`: `columns` — `fragments` as before; `markers` — the text with the markers, no `fragments`;
  `none` — no `fragments`, `"match": true` on matched lines; the repeated lines likewise.
- `src/engine.rs`: the style from the key and the format; `src/description.rs`: the schema.

## Tests

- examples E87.1–E87.13 (fixture `highlight`, fixture `repeated` of task 86)
- unit: the columns line (several fragments, escape levels, a multiline fragment), the request field

## Steps

1. failing tests
2. key, styles in the text and JSON formats

## Risks

- none
