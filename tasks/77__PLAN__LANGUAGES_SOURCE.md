# Plan for task 77: `languages.<name>.source`

## Design

- `src/config/languages.rs`: `source` joins the implemented keys (a boolean, `true` for a new language); the kinds
  have used `source` of the language since task 16. All keys of a language are implemented now.
- `src/description.rs`: `source` in the `languages` object.

## Tests

- examples E77.1–E77.8 (fixture `sourcekind`: a JSON file, an HCL file, a Rust file)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
