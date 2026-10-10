# Plan for task 76: `languages.<name>.strings`

## Design

- `src/config/languages.rs`: `strings` joins the implemented keys (a list of strings); the comment scanner has
  skipped strings since task 29.
- `src/description.rs`: `strings` in the `languages` object.

## Tests

- examples E76.1–E76.7 (fixture `strings`: a comment marker inside a string)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
