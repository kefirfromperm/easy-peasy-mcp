# Plan for task 79: `output.detail`

## Design

- `src/config/keys.rs`: `Key::Detail`, `Type::Choice(["exists", "count", "facets", "files", "lines"])`, default
  `"lines"`; `facets` is a value of the spec which comes with task 105 (`Key::unimplemented`).
- `src/output/mod.rs`: `Detail`; `Response.detail` decides the parts:
  - `exists`: the header `exists: yes` / `exists: no` (with `truncated: yes (timeout)`), the warnings; JSON `version`,
    `exists` (and the truncation, the warnings);
  - `count`: the header with the result id and the warnings; JSON without `legend` and `files`;
  - `files`: the header, the legend, the files with their conditions and counts, without lines; JSON files without
    `blocks`;
  - `lines`: as before.
- `src/search/mod.rs`: `SearchOptions.first_only` — the walk quits at the first matched entry (`exists` stops at the
  first match); `src/engine.rs`: no further working directory after a match, no result id for `exists`.

## Tests

- examples E79.1–E79.10 (fixture `detail`)
- unit: the text and JSON of every level, the request field, `facets` not implemented

## Steps

1. failing tests
2. key, the levels of the response, the early stop

## Risks

- `exists` stops a parallel walk: the warnings it gives depend on what was visited first; examples of `exists` have no
  warnings
