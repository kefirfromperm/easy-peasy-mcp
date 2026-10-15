# Summary of task 145: one header per directory in relevance order

Server version: `0.142.0`.

## What was built

- With `sort = relevance` and `group_dirs = true` (both the defaults) the files of a directory are put together, so
  every directory gets one header ([OUTPUT.md#directory-groups](../OUTPUT.md#directory-groups)): the directories in
  the order of their most relevant file, the files of a directory in their relevance order. The most relevant file
  stays the first one. Before, a header was written whenever the directory changed, and a search over `src` could
  write `src/output/` three times.
- A member of an archive is in the directory of its header (`logs.zip!/app/`); the files of two working directories
  are not merged even when their headers are the same.
- The grouped order is the order of the result: JSON lists its `files` in it, the cursor continues in it, and
  `max_files`, the budget and the cut of a listing take the files in it.
- `group_dirs = false` keeps the order of the scores; the other sorts are not regrouped.
- Spec (its own commit): OUTPUT.md#directory-groups, #relevance (the grouping after the scores, every spec version)
  and #compact-output (the order of `group_dirs` applies to both formats, the headers to the text only). Not a
  breaking change: the relevance order is a ranking, not a contract (TOOLS.md#versioning).

## Architecture

- `output::sort::group_by_dir(files)`: the key of a file is its root and the directory of its header
  (`output::split_found`, now `pub(crate)`); a stable sort by the index of the first file of the key, the files moved,
  not cloned.
- `Engine::search_runs` calls it after `relevance::rank` when the query's `output.group_dirs` is `true`. The result
  is ordered once, before the cache, the cursor, the limits, the budget and the formats see it, so none of them
  changed.

## Deviations from the plan

None.

## Tests

- unit (`output/sort.rs`): `the_files_of_a_directory_together` (interleaved directories, a file in the working
  directory, a subdirectory, an empty list), `archive_members_and_roots_have_their_own_header`.
- examples: E145.1 (lines), E145.2 (files, count by file), E145.3 (JSON), E145.4 (`max_files` and the next page),
  E145.5 (a list cut by the budget and the next page), E145.6 (`group_dirs = false`, `sort = path asc`); the fixture
  `dir-groups` has 14 files in three directories which take turns in the order of the scores.
- earlier examples changed by the decision, each showed a directory split by the relevance order: E9.4, E121.100,
  E133.1, E133.2, E133.4 and E133.15 (the first page now ends after `src/a.rs`, the next page has `notes.md`),
  E136.1–E136.4, E136.9, E136.11.

## Limitations

- A cut by `max_files` or the budget takes whole directories in turn: a file of the third directory with a higher
  score than the last files of the first one can be on the next page (E145.5: `app/d.rs`, fourth by score). The
  directory of the most relevant file still comes first, and `group_dirs = false` gives the pure score order.
- The write tools (`replace`, `rename`, `copy`, `delete`) list their files in path order and are unchanged.

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --root .
```

```json
{"query": "line(text has 'repeated' /i)", "dirs": ["src"], "output": {"detail": "count", "count_by": "file"}}
```

Every directory of `src` has one header.
