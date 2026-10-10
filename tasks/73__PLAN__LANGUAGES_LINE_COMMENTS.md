# Plan for task 73: `languages.<name>.line_comments`

## Design

- `src/config/languages.rs`: `line_comments` joins the implemented keys (a list of strings); `/c` has used the line
  comments of the language table since task 29, now with the table of the request. An empty marker is ignored by the
  comment scanner.
- `src/description.rs`: `line_comments` in the `languages` object.

## Tests

- examples E73.1–E73.8 (fixture `terraform`: `#` and `//` comments and a string)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
