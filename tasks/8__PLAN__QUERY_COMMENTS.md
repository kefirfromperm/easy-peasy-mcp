# Plan for task 8: Comments in the query

## Design

- The lexer already skips `#` comments outside of strings until the end of the line and records their positions;
  `#` inside a string is part of the string.
- A query of comments only has no tokens: the parser returns the empty query (task 2).
- The legend already writes a gap between tokens — whitespace or a comment — as one space.
- `query/subset.rs`: comments are no longer `not_implemented`.
- `description.rs`: `# starts a comment until the end of the line`.

## Tests

- examples E8.1–E8.4
- unit: the subset accepts comments; a comment-only query parses to the empty query; the legend without comments

## Steps

1. failing unit tests
2. subset change, description

## Risks

None known.
