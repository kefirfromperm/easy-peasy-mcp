# Summary of task 8: Comments in the query

Server version: `0.8.0`.

## What was built

- `#` outside of strings starts a comment until the end of the line (QUERY.md#grammar), also right after a value and
  after the last condition; `#` inside a string is text.
- A query of comments only is the empty query and returns all files.
- The legend writes the conditions without the comments.
- The `find` description mentions comments.

## Architecture

The lexer recorded comments since task 1; the subset check no longer rejects them.

## Deviations from the plan

None.

## Tests

- examples E8.1–E8.4 (fixtures `names` and `flat`)
- unit: the subset accepts comments, a comment-only query, the legend without comments

## Limitations and known gaps

None new.

## Try it

`{ "query": "name ~ '*.rs'   # Rust only\nand not path ~ '**/tests/**'" }`
