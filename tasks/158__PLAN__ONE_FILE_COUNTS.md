# Plan of task 158: The counts of the only file are not repeated

## Design

The rule is decided in `engine` with the other rules of spec 2 and carried to the text format by a new field of
`output::Response`:

- `Response::one_file: OneFile` with `path: bool` (write the file without its directory header) and `counts: bool`
  (leave out its `lines: N  matches: N`). `engine` sets both when the spec is 2, the result has exactly one matched
  file (all of it, not the page), there is no `since` and `show` is not `"all"`; `counts` also needs `file_counts`
  not in `show`.
- `Response::push_file` (text only) skips the directory header when `one_file.path`, and the counts when
  `one_file.counts` and the header writes the counts (`counts` not left out). Both are off for `count` detail with
  `count_by = line`.
- JSON doesn't read the field: it is unchanged.

Rejected: adding `file_counts` to the effective `Omit`: the JSON format reads the same set, and its `lines` and
`matches` of a file must stay.

## Tests

- examples E158.1–E158.7
- unit `output::tests::one_file_alone`: `files` and `count_by = file` with `one_file`, the header without counts keeps
  the file counts, `count_by = line` unchanged
- earlier examples with one file in spec 2 change: they are fixed in their examples files, the reason in the commit

## Steps

1. red: the unit test, the examples
2. `OneFile` in `Response`, `push_file`, the engine
3. fix the earlier examples with one file in spec 2
4. clippy, fmt, the definition of done, summary, version `0.155.0`

## Risks

- many earlier examples show one file in spec 2; each changed response must be checked to be the rule and nothing else
