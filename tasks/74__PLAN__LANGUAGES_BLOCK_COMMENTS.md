# Plan for task 74: `languages.<name>.block_comments`

## Design

- `src/config/languages.rs`: `block_comments` joins the implemented keys — a list of `[start, end]` pairs of strings
  (`a list of [start, end] pairs of strings is expected`), the type of task 71; `/c` has used the block comments of
  the language table since task 29.
- `CONFIG.md`: the message of a wrong value.
- `src/description.rs`: `block_comments` in the `languages` object.

## Tests

- examples E74.1–E74.7 (fixture `pascal`: a block comment inside a line and one over two lines)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
