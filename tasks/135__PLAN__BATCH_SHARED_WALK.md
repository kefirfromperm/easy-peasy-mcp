# Plan for task 135: one walk for the queries of a batch

## Design

### `search` — one walker, several queries

- `search::search_shared(root, &[(&Query, &SearchOptions)]) -> Vec<SearchResult>`: one `WalkBuilder` (the walk
  options of the first search: the group key makes them equal), one `Walk` per query. `search` becomes
  `search_shared` with one query, so a query alone and a query of a batch run the same code.
- Per entry of the walker an `EntryCache` (one thread visits an entry, so `OnceCell` / `Cell`):
  - `raw`: the read of the file once — `TooBig` (before the read), `Failed(Option<WarningCode>)`, `Broken(WarningCode)`
    (a broken or too big compressed file), `Bytes`; every query replays the warning of the read on its own warnings
    and checks its own deadline after the read, as now
  - `decoded`: `decode` of the bytes once (the decode options are in the group key); a query which needs comments
    clones the content and adds its comments, the others borrow it
  - `first_line` (shebang of the language detection) and `starts_binary` (`binary = exclude`) once
  - `scanned` / `matched`: the progress counts the entry once, the first time a query evaluates or matches it
- the slow read hook sleeps at most until the latest deadline of the queries still running (none if one has none)
- the walker visits an entry for every query whose deadline hasn't passed and which isn't done (`first_only` with a
  match); it quits when no query is left
- archives (`archives = all`): `visit_members_shared` iterates the members once, the size limit of the archive is
  counted once; per member a `MemberCache` reads the member lazily (`fill` up to a length, so the 8K head of
  `binary = exclude` and the whole content share one read) and decodes it once; every query evaluates it, gets the
  archive warnings and its found members as now
- `search_files` (`within`) uses the same caches with one query

### `engine` — groups

- `try_find` is split into `prepare` (parse the request, settings, the file target, the query, `within`/`since`,
  `output.groups`, the page, the search options) → `Prepared`, the search, and `finish` (groups, cursor check,
  cancellation, `respond`, `output.file`). `find` without `queries` runs the three in a row
- `batch`:
  1. prepare every item in order (a wrong item is the error of its section)
  2. group the prepared items without `within` by their `WalkKey`: the canonical working directories and the values
     of the keys of the `traversal` and `kinds` sections, the language list and `path_case`; the groups run in the
     order of their first item, an item with `within` alone at its place
  3. a group: the deadline of every query from the start of the group, one lease of the threads, `search_group`
     (the working directories one after another like `search_all`: a query which timed out or found its `exists`
     match doesn't walk the next directory), then sorting per query
  4. the responses in item order with the budget split as now: the arguments of an item are made again with its
     parts of `budget` / `max_output` and `finish` responds with them
- `search_all` becomes `search_group` with one query

## Decisions

- One `Walk` per query, not a query-of-queries evaluator: the evaluation, the funnel, the warnings and the found
  entries stay per query, so every section is exactly the response alone; only the I/O is shared.
- Grouping by setting values instead of comparing `SearchOptions`: `SearchOptions` holds compiled regexes and
  `Arc`s; the values of the keys are plain and say the same.
- Git status and kinds are still looked up per query (they are not the content of the file); a shared `GitIndex`
  can come later if a profile shows it.
- Rejected: running the queries one after another on a cached tree (the entries and contents held in memory) — the
  memory of a whole tree for a batch.

## Tests

- examples E135.1–E135.7 (green before the code: the responses are those of the queries alone) and the startup
  example E135.8 (red: 15 scanned entries instead of 5)
- unit `search::tests::shared_search_answers_like_alone`: two queries over a temp tree with `search_shared` give the
  results of `search` alone and count every entry once in `Progress`
- unit `engine::tests::walk_keys`: items with the same `dirs`/`traversal` have the same key, an own `dirs` or
  `traversal.hidden` another one
- all earlier examples, the batch examples of task 113 unchanged

## Steps

1. `EntryCache` and `search_shared` in `search`, `search` on top of it; unit test green
2. `prepare` / `finish` in the engine, `try_find` on top of them
3. `WalkKey`, `search_group`, the batch in three phases; E135.8 green
4. clippy, fmt, the whole test suite, the release binary

## Risks

- timing of E135.8: 700ms per read and one thread give a notification while the fifth entry is read
- archives: a member's head and content share the bytes read; a member bigger than `max_file_size` with
  `binary = exclude` takes its head from the bytes read before the limit (before: from the bytes after it)
