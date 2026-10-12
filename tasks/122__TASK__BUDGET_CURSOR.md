# Task 122: The cursor of a file list cut by the budget

Status: in progress

Depends on: task 108, task 101

## Goal

An agent which pages through a file list cut by the [budget](../OUTPUT.md#budget) gets every file exactly once. Today
the next page can skip files, or there is no next page at all.

## The bug

At budget level 4 (`detail = files`) the list is cut to the files which fit, but the next cursor starts after
`max_files` files instead of after the files listed (`next_cursor` in `src/output/mod.rs` adds
`limits.max_files`, not `files_limit()`):

- fewer files fit than `max_files`, and `max_files` reaches past the end: the response says
  `truncated: yes (budget)` but has no `next cursor`; the rest of the files can't be reached

  ```json
  { "query": "path ~ 'src/**/*.rs'", "output": { "budget": 150, "sort": "path asc" } }
  ```

  in this repository: 21 of 47 files listed, no cursor

- `max_files` is inside the list: the cursor jumps `max_files` ahead and the files between the last one listed and
  the cursor are never shown

  ```json
  { "query": "path ~ 'src/**/*.rs'", "output": { "budget": 150, "sort": "path asc", "max_files": 20 } }
  ```

  in this repository: pages of 20, 15 and 7 files, 5 of the 47 files never listed

## Scope

- the next cursor of a list cut by the budget starts at the first file not listed
  ([OUTPUT.md#budget](../OUTPUT.md#budget): "the list cut by the budget has a `next cursor`")
- the same in the text and the JSON format, and in a section of `queries`
- `truncated` / `truncated_by` unchanged

## Out of scope

- how the budget chooses the level and the number of files that fit

## Spec

- [OUTPUT.md#budget](../OUTPUT.md#budget)
- [OUTPUT.md#pagination](../OUTPUT.md#pagination)

## Examples

`122__EXAMPLES__BUDGET_CURSOR.md` covers:

- a budget list cut before `max_files` reaches the end: the response has a next cursor; the following pages list the
  rest, every file once
- a budget list cut inside `max_files`: the next page starts right after the last file listed
- the same in the JSON format

## Acceptance criteria

- a test reproduces both cases before the fix (AGENTS.md#tests: a bug fix starts with a test)
- every example in `122__EXAMPLES__BUDGET_CURSOR.md` passes as a test; the examples of all earlier tasks still pass
- a bug fix only: the server version gets a patch bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
