# Plan for task 21: `text` with `=`, `starts`, `ends`

## Design

- `query/mod.rs`: the content regex of a file scope `text` condition is anchored to the whole normalized content:
  `=` → `\A v \z`, `starts` → `\A v`, `ends` → `v \z` (`v` escaped, `(?i)` with `/i`); the fragment is the matched
  part and may span lines (task 19 renders it).
- `query/subset.rs`: `text` with `=`, `has`, `starts`, `ends`.
- `operator_not_allowed` for `~` / `~~` on `text` exists since task 1.
- `description.rs`: `= starts ends` on `text`.

## Tests

- examples E21.1–E21.6 (fixture `scripts`)
- unit: the compiled regexes match the whole content, its start, its end (with and without a final line break)

## Steps

1. failing unit tests
2. regex anchors, subset, description

## Risks

None known.
