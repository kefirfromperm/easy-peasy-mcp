# Plan for task 75: `languages.<name>.nested_block_comments`

## Design

- `src/config/languages.rs`: `nested_block_comments` joins the implemented keys (a boolean, `false` for a new
  language); the comment scanner has nested block comments since task 29.
- `src/description.rs`: `nested_block_comments` in the `languages` object.

## Tests

- examples E75.1–E75.7 (fixture `nested`: a block comment inside a block comment)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
