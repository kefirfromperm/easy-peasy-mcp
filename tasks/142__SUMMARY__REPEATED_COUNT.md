# Summary of task 142: the number of files of a repeated line

Server version: `0.139.0`.

## What was built

- `×N` of a [repeated line](../OUTPUT.md#repeated-lines) and its JSON `count` are the number of all the files of the
  result which have the line, also the files after `max_files` and on the other pages of a cursor. Before, the
  lines were grouped among the listed files only, and a line in 137 files was `×100` with the default `max_files`.
- The locations are those of the listed files of the page, `… M more` counts all the other files of the result.
- A line is grouped when at least `group_repeated` files of the result have it and at least one of them is listed;
  every page of a cursor writes it with the same `N` and the locations of its own files, so the pages are not added
  up. A line only in files which are not listed comes with its files on their page.
- Spec (its own commit): OUTPUT.md#repeated-lines says what `N` counts. Not a breaking change: the JSON field was
  already "the number of all the files", the text now says the same, no earlier example changed.
- Bug fixed on the way: the locations of a page after the first had the paths of the first files of the result
  (the index among the listed files was looked up among all the files); E142.4 covers it.

## Architecture

- `output::repeated::repeated(files, listed, min)`: one pass over the matched lines of all the files; a `Group` per
  line counts its files and keeps its listed files (`RepeatedLine::files`, indexes among all the files) and the lines
  to leave out of the listed files (`Repeated::removed`, per listed file); `RepeatedLine::count` is the number of all
  its files, which orders the lines.
- `Response::listed_range` gives the indexes of the listed files; `repeated()` passes it with all the files;
  `push_repeated` and the JSON `repeated` use `count`.

## Deviations from the plan

- None in the behavior. A `Group` struct instead of a tuple for clippy.

## Tests

- examples E142.1–E142.5 (fixture `repeated-count`): more files than `max_files` in text, with
  `repeated_locations = 0`, in JSON, the three pages of a cursor, a budget
- unit `repeated::tests::files_which_are_not_listed_count`; the earlier unit tests with the listed range
- all earlier examples unchanged

## Limitations and known gaps

- The locations of the files which are not listed are only on their pages; `line(text = '<the line>')` with
  `detail = files` lists all the files of a line in one call, as the spec says.
- `cargo clippy` of the system toolchain 1.99 (the pinned 1.98 is not installed here) still reports its
  `assert_is_empty` lint on 18 asserts of the tests, the same as before this task (3 of them in the earlier tests of
  `repeated`, which only got the listed range).

## Try it

`{"query": "line(text = 'Spec: `1`')", "dirs": ["tasks"], "output": {"repeated_locations": 0}}` — `×N` is the number
of all the examples files with the line, the locations those of the first 100.
