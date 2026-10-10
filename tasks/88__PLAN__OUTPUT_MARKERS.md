# Plan for task 88: `output.markers`

## Design

- `src/config/keys.rs`: `Type::Markers` — two non-empty strings (`two non-empty strings are expected` /
  `must be two non-empty strings`), `Key::Markers` after `Highlight`, default `['⟦', '⟧']`.
- `src/output/mod.rs`: `Highlight::Markers(open, close)` carries the markers; `src/output/lines.rs` writes them
  instead of the constants.
- `src/warning.rs`: `WarningCode::MarkerInContent` (`marker_in_content`) after `DateUnsupported`.
- `src/output/mod.rs`: `Response::marker_warnings` — in `lines` detail with markers, the files with a matched line
  which contains a marker; both formats add them to the warnings of the response.
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E88.1–E88.11 (fixture `marker-text`, fixture `highlight` of task 87)
- unit: the type (two strings, empty strings), the warning, the request field

## Steps

1. failing tests
2. key, markers in the lines, the warning

## Risks

- none
