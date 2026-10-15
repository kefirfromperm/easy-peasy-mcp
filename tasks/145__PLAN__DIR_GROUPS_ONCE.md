# Plan for task 145: One header per directory in relevance order

## Design

- `output/sort.rs`: `group_by_dir(files)` puts the files of a directory together: the key of a file is its root and
  the directory of its header (`split_found`: the directory of the path, or `archive!/dir/` for a member of an
  archive); the directories keep the order of their first file, the files of a directory their order. A stable sort
  by the index of the first file of the key, O(n log n)
- `output/mod.rs`: `split_found` becomes `pub(crate)`, `group_by_dir` is exported next to `sort_files`
- `engine.rs`: after `relevance::rank`, `group_by_dir` when `output.group_dirs` of the query is `true`. The engine
  sorts the files of the result once, before the cache, the cursor, `max_files`, the budget and both formats take
  them, so all of them see the grouped order without a change of their own

Decisions (in the spec commit): the new order is the default (`group_dirs = true`), not a new value of the option, and
for every spec version: the relevance order is a ranking, not a contract (TOOLS.md#versioning); the earlier examples
which showed a split directory are changed (named in the examples commit and the summary). JSON follows the text
order, as the task says, so `group_dirs` changes the order of JSON too (OUTPUT.md#compact-output says so now).

Rejected: grouping in the text writer only: the cursor, `max_files` and the budget would cut in another order than
the one written, and a page could start in the middle of a directory written on the page before.

## Tests

- unit: `group_by_dir` — interleaved directories, the order inside a directory, archive members under their header,
  the same directory of two roots kept apart, an empty list
- examples: E145.1–E145.6; the earlier examples changed by the decision (E9.4, E121.100, E133.1, E133.2, E133.4,
  E136.1–E136.4, E136.9, E136.11)

## Steps

1. red: the unit tests of `group_by_dir` (the example tests run from the examples file)
2. green: `group_by_dir` and the call in the engine
3. summary, status `implemented`, version 0.142.0

## Risks

- several working directories: files of `src/` of two roots must not merge; the key has the root
- the result cache keeps the grouped order; `within` searches the files again and ranks them again, so it groups again
