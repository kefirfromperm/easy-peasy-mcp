# Summary of task 122: the cursor of a file list cut by the budget

Server version: `0.121.1` (a bug fix, patch bump).

## What was fixed

- a file list cut by the [budget](../OUTPUT.md#budget) at level 4 (`detail = files`) has a next cursor which starts
  at the first file not listed, so paging lists every file exactly once
  ([OUTPUT.md#pagination](../OUTPUT.md#pagination)); before, the cursor started `max_files` after the start of the
  page: no cursor when `max_files` reached past the end of the list, skipped files when it was inside it
- the same in the text and the JSON format and in a section of `queries`; `truncated` / `truncated_by` unchanged
- the spec: the pagination section names the budget among the limits which give a next cursor, as the budget section
  already said

## Architecture

- `Response::next_cursor` (`src/output/mod.rs`) puts the cursor of a cut file list at `page_start + files_limit()`,
  the limit the list is cut to (`max_files` lowered to `budget_files`), instead of `page_start + max_files`
- `budget::fit` (`src/output/budget.rs`) tries the whole file list first and searches in halves only below it:
  with the cursor, a list cut by the budget can be longer than the whole list (the header gets `next cursor: ...`),
  so the response size is no longer monotone in the number of files at the top and the search in halves could miss
  the whole list

## Deviations from the plan

- the plan expected no change in `budget.rs`; with the cursor fix alone E108.3, E108.7 and E109.3 fell to the facets
  (10 files and a cursor didn't fit where 11 files without one did), fixed by the second change above, with a unit
  test

## Tests

- examples E122.1–E122.4 (`tasks/122__EXAMPLES__BUDGET_CURSOR.md`, fixture `spec-output-many`): a budget list cut
  before `max_files` reaches the end (pages of 14, 14 and 9 files), one cut inside `max_files` (the second page starts
  at the 14th file, not the 31st), JSON (19 and 18 files), a `queries` section with the cursor in the section's
  `output`
- unit tests: `output::tests::the_cursor_of_a_budget_list_starts_after_the_files_listed` (the cursor for `max_files`
  past the end, inside the list, alone, and no cut), `output::budget::tests::the_whole_list_without_a_cursor`
- all examples of the earlier tasks pass

## Limitations

- the cursor's length depends on the file index (one character more for a two-digit index and so on), so near a
  digit boundary the search in halves can list one file fewer than would fit; never more than fit

## How to try it

```sh
cargo build --release
```

```json
{ "query": "path ~ 'src/**/*.rs'", "output": { "budget": 150, "sort": "path asc" } }
```

In this repository the response lists the files which fit with `next cursor: ...`; repeating the request with
`output.cursor` lists the rest, each file once.
