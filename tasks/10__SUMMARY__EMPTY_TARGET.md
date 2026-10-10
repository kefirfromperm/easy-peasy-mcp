# Summary of task 10: `empty` target

Server version: `0.10.0`.

## What was built

- `empty` without operator and value (QUERY.md#targets): true for a regular file of size 0 and for a directory
  without any entry on disk. Decision, written into the spec: like `find -empty`, hidden and ignored entries make a
  directory non-empty; a symlink is never empty.
- An operator after `empty` is `unexpected_token` with the hint `'empty' stands alone, without an operator and a
  value`.
- The `find` description shows `empty`.

## Architecture

`Filter::Empty`, `FileValues.empty`; `Query.needs_empty` makes the search read the size (`metadata`) or the first
directory entry (`read_dir`) only for queries with `empty`.

## Deviations from the plan

None.

## Tests

- examples E10.1–E10.5 (fixture `empties`, empty directories from the setup)
- unit: `empty` evaluation and `needs_empty`, the parser hint, the subset, `is_empty` for files and directories with
  a hidden entry

## Limitations and known gaps

None new.

## Try it

`{ "query": "type = dir and empty" }`
