# Plan for task 142: the number of files of a repeated line

## Spec decision

`×N` (and JSON `count`) counts every file of the result which has the line: the JSON field already said "the number
of all the files", so this is a fix of the text, not a breaking change. The locations are those of the listed files
of the page, `… M more` counts all the others. A line is grouped when at least `group_repeated` files of the result
have it and at least one is listed; every page writes it with the same `N`.

## Design

- `output::repeated::repeated(files, listed, min)`: `files` all the files of the result, `listed` the range of the
  listed ones. The lines are grouped over all the files; a `RepeatedLine` gets `count` (all its files) and `files`
  (its listed files, with their index in all the files); it is kept when `count >= min` and `files` is not empty.
  `removed` stays per listed file (index from the start of `listed`), as `Listing` uses it.
- `Response::repeated` passes `self.files` and the range of `listed()`; `push_repeated` writes `×{count}` and
  `… {count - written} more`; JSON `count` is `count`.
- `Response::locations` looks the path up with the index in all the files, which fixes the paths of the locations
  on a page after the first (they were looked up with the index in the listed files).

## Decisions

- Grouping over all the matched files costs a pass over their matched lines; they are in memory already.
- Rejected: `×100 of 137+` — the agent wants the number, and every page would need the "of".

## Tests

- examples E142.1–E142.5
- unit `repeated::tests`: a line counted in files outside of the listed range, a line only outside is not kept
- all earlier examples

## Steps

1. red: the unit tests of `repeated` with the new signature
2. `repeated`, `Response` text and JSON
3. fmt, clippy, all tests, release build

## Risks

- an earlier example with a repeated line and cut files changes: it showed the wrong count, fixed with the reason
