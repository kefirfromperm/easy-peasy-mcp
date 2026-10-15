# Task 135: One walk for the queries of a batch

Status: in progress

Depends on: task 113, task 133, task 134

## Goal

A batch of N queries over the same directories costs one directory walk and one read of every file instead of N.
An agent can batch its questions about a tree (definitions, calls, tests) without paying for the tree N times.
OUTPUT.md#batch already promises this: "queries with the same `dirs` and `traversal` share one directory walk".

Found by trying the server: a batch of two queries over the repository wrote the same `binary_skipped (25)` and
`decode_errors (1)` warnings in both sections. `Engine::batch` calls `try_find` once per query, and every call walks
the tree and reads the files again (`search::search` builds a new `Walk` each time). Task 113 left this as a known
gap ([113__SUMMARY__BATCH.md](113__SUMMARY__BATCH.md#limitations-and-known-gaps)).

## Scope

A spec change first (its own commit): OUTPUT.md#batch says the queries "run in parallel", but the budget split gives
a query what the earlier ones didn't use, so the responses are made in order. The spec should say what really
happens: the queries with the same walk share it, and their responses are made in the order of the items.

- the items are grouped by their walk: the same working directories (after `dirs` of the item and of the request are
  resolved) and the same walk and file options of `traversal` (depth, symlinks, hidden files, ignore files,
  `exclude`, `binary`, `max_file_size`, encodings, archives, `path_case`, `skip_generated`), and `kinds` /
  `languages` where they decide what an entry is
- one walk per group: every entry is visited once and its content read and decoded once, then every query of the
  group is evaluated on it. The response of every query is the same as when it runs alone: files, counts, fragments,
  funnel, hints, warnings, relevance order, result id
- the timeout of every query stays its own: a query whose deadline has passed stops being evaluated and gets
  `truncated: yes (timeout)`, the walk goes on for the others; the walk stops when all queries of the group have
  stopped, or at the first match of a query with `detail = exists` only when it is the last one still running
- progress counts the entries of the shared walk once; cancellation stops the whole batch
- items with `within` (shared by all the items) search the files of the kept result and are not walked; an item with
  its own `dirs` or `traversal` forms its own group
- the result ids are given in the order of the items, as now; the budget and `max_output` are split as now

## Out of scope

- writing a warning which several sections have only once: every section stays the full response of its query (a
  separate task, if wanted; `output.omit = ["warnings"]` of task 134 already drops them per query)
- sharing walks between separate `find` calls (a cache of the tree)
- running different groups in parallel

## Spec

- [OUTPUT.md#batch](../OUTPUT.md#batch): the shared walk, the order of the responses, the timeouts per query
- [TOOLS.md#progress](../TOOLS.md#progress): the progress of a batch counts the entries of its walks

## Examples

`135__EXAMPLES__BATCH_SHARED_WALK.md` covers:

- a batch of three queries over the same tree: every section byte for byte the response of the query alone (shown
  by the same requests without `queries`)
- warnings of the shared walk in every section that has them, as now
- a query with its own `dirs` and one with its own `traversal.hidden`: their own walks, the same responses
- different timeouts in one group with `Slow reads`: the query with the short timeout is cut, the others are not
- a startup exchange with `Slow reads` and a progress token: the last progress of a batch of three queries over four
  files counts four scanned files, not twelve
- `detail = exists` in a group with a `lines` query: both answers as alone

## Acceptance criteria

- the spec describes the shared walk and the order of the responses before the code is written
- every example in `135__EXAMPLES__BATCH_SHARED_WALK.md` passes as a test; the examples of all earlier tasks still
  pass, the batch examples of task 113 unchanged
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
