# Plan for task 122: the cursor of a file list cut by the budget

## Design

One line in `src/output/mod.rs`. `Response::next_cursor` puts the cursor of a list cut by the file limit at
`page_start + max_files`; the list itself is cut at `page_start + files_limit()`, where `files_limit()` is
`max_files` lowered to `budget_files` by the budget. The cursor takes `files_limit()` too, so it starts at the first
file not listed.

- `files_cut()` already compares with `files_limit()`, so a list cut by the budget alone now gets a cursor; the
  filter `file < files.len()` stays for the cut lists of `max_lines` / `max_output`
- the budget search in `src/output/budget.rs` renders the whole response for every candidate `budget_files`, so the
  cursor in the header is part of the estimate; no change there
- `truncated_by` keeps comparing with `max_files` (`max_files` is named only when `max_files` itself leaves files
  out) — unchanged, as the task says
- JSON and `queries` sections use the same `next_cursor`, so they are fixed with it

No new types, modules or dependencies.

## Rejected

- moving the cursor to `listed().len()`: the same value, but `next_cursor` already works with the cut, not the
  listing

## Tests

- examples E122.1–E122.4 (`tests/examples.rs`): pages of text, JSON and a `queries` section, every file once
- unit test in `src/output/mod.rs`: a response with `budget_files` below `max_files` (and both below and above the
  number of files) has a cursor at `page_start + budget_files`
- the examples of every earlier task, in particular 101 (cursor) and 108 (budget), still pass

## Steps

1. the unit test, failing (with the examples of the previous commit)
2. the fix
3. summary, status `implemented`, patch version bump `0.121.0` → `0.121.1`

## Risks

- the cursor in the header of a list cut by the budget alone takes space, so fewer files fit than before (E122.1:
  14 instead of 19); this is what the spec asks for: the response fits in the budget with its cursor
