# Plan for task 10: `empty` target

## Design

- Decision (written into QUERY.md#targets): like `find -empty`, a regular file is empty with size 0, a directory
  without any entry on disk — hidden and ignored entries count; a symlink is never empty (not followed).
- `query/eval.rs`: `Filter::Empty { id, order }`; `FileValues.empty: bool`.
- `query/mod.rs`: `Query.needs_empty` — whether any `empty` condition is in the query, so the search reads the size
  or the directory only when needed.
- `query/subset.rs`: `empty` is supported.
- `query/parser.rs`: an operator right after `empty` is `unexpected_token` with the hint
  `'empty' stands alone, without an operator and a value`.
- `search`: `empty` = file size 0 (`metadata`), or `read_dir(...).next().is_none()` for a directory.
- `description.rs`: `empty`.

## Tests

- examples E10.1–E10.5 (fixture `empties`, empty directories from the setup)
- unit: `Filter::Empty`, the parser hint after `empty`, the subset accepts `empty`, `needs_empty`; search unit test
  of the emptiness of a directory with a hidden entry

## Steps

1. failing unit tests
2. filter, parser hint, subset, search
3. description

## Risks

None known.
