# Summary of task 135: one walk for the queries of a batch

Server version: `0.138.0`.

## What was built

- The queries of a batch with the same walk share it (OUTPUT.md#batch): every entry is visited once, its content
  read and decoded once, and every query of the walk is evaluated on it. A batch of N queries over one tree costs
  one walk and one read of every file instead of N; every section is byte for byte the response of its query alone.
- The walk of an item is its working directories (as written and canonical), the values of the `traversal` and
  `kinds` keys, `server.threads`, the languages and the case of paths; an item with its own `dirs` or `traversal`
  can get a walk of its own. An item with `within` searches the files of the kept result, not walked.
- The walks run one after another in the order of their first item; the responses are made in item order with the
  budget and `max_output` split as before (task 113).
- Every query keeps its own timeout, counted from the start of its walk: a query past its deadline stops being
  evaluated and is `truncated: yes (timeout)`, the walk goes on for the others. A `detail = exists` query stops at
  its first match; the walk stops when all its queries have stopped.
- The progress of a batch counts every entry of a shared walk once, and an entry matched by several queries once
  (TOOLS.md#progress).
- Spec change first (its own commit): OUTPUT.md#batch said the queries "run in parallel"; it now describes the
  shared walk, the order of the walks and responses and the timeouts.

## Architecture

- `search::search_shared(root, &[(&Query, &SearchOptions)])`: one `WalkBuilder` (the walk options of the first
  search), one `Walk` per query. Per entry a `Shared` holds `OnceCell`s of the raw read (`Raw`: `TooBig`, `Failed`,
  `Broken`, `Bytes`), the decoded content, the first line (shebang) and the binary head, and `Cell`s for the progress.
  `Walk::fetch` gives one query the content (`Fetched`): it replays the warning of the read on that query's warnings
  and checks that query's deadline, as the old per-query read did. A query with `/c` clones the decoded content to
  add its comments; the others borrow it. `search` is `search_shared` with one query.
- Archives: `visit_members` iterates the members once for all queries; a `SharedMember` reads a member lazily, so
  its 8K head (for `binary = exclude`) and its whole content share one read.
- `engine`: a find is `prepare` (request, settings, file target, `prepare_run`: query, `within`/`since`,
  `output.groups`, search options without deadline, `WalkKey`) → `search_runs` (deadlines from the start, one lease
  of the threads, `search_within` or `search_dirs`, sorting) → `Prepared::into_run` + `Run::paged` (page and cursor
  totals) → `finish` (cancellation, `respond`, `output.file`). `try_find`, `run` (write tools) and `batch` use these.
- `batch`: prepares every item, `search_batch` groups the prepared items by `WalkKey` and searches each group, then
  every item is answered in order with arguments made again with its parts of the budget.
- `search_dirs` walks the working directories one after another for a group, like `search_all` before, which is now
  `search_dirs` with one query (used by the hints).

## Deviations from the plan

- The page and the cursor check moved out of `prepare_run`: the cursor hash of a batch item is made of its final
  arguments, which hold its parts of the budget (E122.4 showed it). `try_find` and `run` still check the cursor before
  the search; a batch item checks it when it is answered, after the search of its group.
- `server.threads` is part of the walk key.
- The leases of a batch are given back when its group is searched, not after the response: the walks of later
  groups must get threads; the hints of a batch response run without a lease.
- E135.8 counts five files: the config file of a startup example is written into the fixture. Its response was
  first written by hand without the blank line before `warnings left out` (fixed in its own commit).

## Tests

- examples E135.1–E135.7 (fixture `shared-walk`): sections equal to the queries alone (E135.2, E135.5), the
  warnings in every section which reads `data.bin` (E135.3), own `dirs` and `traversal.hidden` (E135.4), a short
  timeout in a group with slow reads (E135.6), `exists` with `lines` (E135.7); startup example E135.8: the progress of
  three queries over five files reaches 5 and never 6 or 7
- unit `search::tests::shared_search_answers_like_alone` (results like alone, every entry scanned once),
  `engine::tests::walk_keys`
- `tests/search.rs::a_batch_reads_every_file_once`: three queries over three files with 400ms reads take about
  0.5s (1.3s with the old per-query walks)
- all earlier examples unchanged, the batch examples of task 113 and the cursor examples of task 122 included

## Limitations and known gaps

- Git statuses and kinds (the generated markers) are still looked up per query; the readability check of a matched
  file too (an open, not a read).
- A warning which several sections have is written in every section (out of scope; `output.omit = ["warnings"]`).
- The groups of a batch run one after another, not in parallel.
- `cargo clippy` of the system toolchain 1.99 (the pinned 1.98 is not installed here) reports its new
  `assert_is_empty` lint on 18 asserts of earlier tests (none of this task); the code of this task is clean.

## Try it

```json
{ "queries": [
    { "label": "defs", "query": "text =~ `fn\\s+load_config`" },
    { "label": "calls", "query": "text has 'load_config('" },
    { "label": "tests", "query": "kind = test and text has 'load_config' /n", "output": { "detail": "count" } } ],
  "output": { "detail": "files" } }
```

One walk of the working directory answers all three.
