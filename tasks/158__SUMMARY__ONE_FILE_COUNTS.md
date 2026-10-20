# Summary of task 158: The counts of the only file are not repeated

Server version: `0.155.0`.

## What was built

- In spec 2 the text format writes the only file of a result as its path alone
  ([OUTPUT.md#one-file](../OUTPUT.md#one-file)): without `lines: N  matches: N`, which repeat the totals of the
  header, and without its directory header, with its path relative to the working directory (to the `in:` line when
  there is one): `TOOLS.md` instead of `./` and `TOOLS.md  lines: 12  matches: 18`, `src/main.rs` instead of `src/`
  and `main.rs  lines: 1`. About 20–25 characters and a line per one-file response.
- In `files` and `lines` detail and in `count` detail with `count_by = file`. The rest of the file line stays: the
  conditions, `(cut)`, the encoding, the metadata.
- The counts stay when the header doesn't write them (`omit = ["counts"]`).
- Unchanged: `count_by = line`, `since` (its files are the differences), JSON, results with more than one file, spec 1.
  "The result" is all the matched files, so a page of one file of several is unchanged.
- `show = ["file_counts"]` writes the counts; `show = "all"` and `"version": 1` write spec 1.
- Spec: one commit (OUTPUT.md#one-file, the rules of the defaults of spec 2, the `count_by = file` bullet, two spec 2
  examples of OUTPUT.md, the spec 2 row of TOOLS.md#versioning). A change of a spec 2 rule, allowed since spec 2 has
  not been used outside this repository.

## Architecture

- `output::OneFile { path, counts }`, a new field `Response::one_file`, set by `engine` with the other rules of spec 2:
  spec 2, one matched file, no `since`, `show` not `"all"`; `counts` also without `file_counts` in `show`.
- `Response::alone` turns it off for `count_by = line` and the counts off when `counts` is left out of the header;
  `Response::push_file` skips the directory header and the counts by it. The JSON format doesn't read the field.

## Deviations from the plan

- none; besides the examples of tasks 130–157, the end-to-end test of `tests/mcp_stdio.rs` changed (`Cargo.toml`
  without `./`)

## Tests

- examples E158.1–E158.7: `count` by file, `files` and `lines` with one file in an absolute working directory, below
  the working directory and in `.`, with more matches than lines; two files unchanged; `show = ["file_counts"]`,
  `omit = ["counts"]`, `"version": 1`, `show = "all"`; `count_by = line` and JSON unchanged
- unit `output::tests::one_file_alone`
- the earlier examples of spec 2 with one file were updated (tasks 130, 135, 141, 143, 144, 146, 147, 148, 153, 154,
  155, 156, 157): the file line and the directory header only, plus the size of a response written to a file and the
  cursor of a batch item in task 155, which follow from the shorter response

## Limitations and known gaps

- `count_by = line` with one file keeps its file counts and its directory header (out of scope of the task)

## Try it

`{"query": "path ~ 'TOOLS.md' and text has 'indent'", "output": {"detail": "count", "count_by": "file"}}` —
`files: 1  lines: 12  matches: 18  result: r1`, an empty line, `TOOLS.md`.
