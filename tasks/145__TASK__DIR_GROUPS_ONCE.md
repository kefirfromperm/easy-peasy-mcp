# Task 145: One header per directory in relevance order

Status: in progress

Depends on: task 85, task 93

## Goal

With `group_dirs = true` and the default `sort = relevance`, the files of a directory are not next to each other,
and the same directory header is written again and again (OUTPUT.md#directory-groups: "the same directory can get
several headers"). A result can pay a header for almost every file, which is what `group_dirs` is meant to save.

Found while working on task 142: `line(text has 'repeated' /i)` with `detail = count, count_by = file` over `src`
listed 11 files under 7 headers, `src/output/` three times:

```
src/output/
repeated.rs  lines: 23  matches: 25
mod.rs  lines: 68  matches: 86
listing.rs  lines: 25  matches: 27
src/
engine.rs  lines: 3  matches: 4
...
src/output/
json.rs  lines: 10  matches: 14
src/config/
keys.rs  lines: 8  matches: 10
src/output/
budget.rs  lines: 2  matches: 2
```

## Scope

A spec change first (its own commit; OUTPUT.md#directory-groups, OUTPUT.md#relevance). The spec step decides:

- the order with `sort = relevance` and `group_dirs = true`: the directories in the order of their most relevant
  file, the files of a directory together in their relevance order; the most relevant file stays the first one
- whether it is the new default or a value of an option (`group_dirs = "once"`?): the relevance order is "a ranking,
  not a contract" (TOOLS.md#versioning), so a new order is not a breaking change; spec 1 examples keep their responses
  if the spec step says so (a spec 1 server keeps the old order)
- the same order in JSON (no headers there, but the order of `files` follows the text) and for the cursor: a page
  continues in the grouped order
- `max_files`, the budget and the cut of a listing take the files in the grouped order
- the other sorts (`path`, `modified`, `size`, `matches`) are unchanged

## Out of scope

- nested headers (a tree of directories)
- grouping by anything else than the directory

## Spec

- [OUTPUT.md#directory-groups](../OUTPUT.md#directory-groups), [OUTPUT.md#relevance](../OUTPUT.md#relevance),
  [OUTPUT.md#pagination](../OUTPUT.md#pagination), [TOOLS.md#versioning](../TOOLS.md#versioning)

## Examples

`145__EXAMPLES__DIR_GROUPS_ONCE.md` covers:

- files of three directories interleaved by relevance: one header per directory, the order of the directories and of
  the files in them, in `lines`, `files` and `count` with `count_by = file`
- JSON, `max_files` with a cursor and its next page, a budget
- `group_dirs = false` and `sort = path asc` unchanged

## Acceptance criteria

- the spec describes the order before the code is written
- every example in `145__EXAMPLES__DIR_GROUPS_ONCE.md` passes as a test; the examples of all earlier tasks still pass
  (or are changed by a decided change, each named in the summary)
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
