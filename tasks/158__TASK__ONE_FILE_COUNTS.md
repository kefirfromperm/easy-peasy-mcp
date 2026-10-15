# Task 158: The counts of the only file are not repeated

Status: not implemented

Depends on: task 136, task 141

## Goal

When a result has one file, its file line repeats the totals of the header. Found in run 3 of the experiment
(EXPERIMENT.md, task 151), reproduced on 0.147.0:

```
find  query = "path ~ 'TOOLS.md' and text has 'indent'"
      output = {"detail": "count", "count_by": "file", "highlight": "none"}

files: 1  lines: 12  matches: 18  result: r20
in: /home/.../agent-af51ef0755b9c5946/

./
TOOLS.md  lines: 12  matches: 18
```

`detail = files` writes the same line. `lines: 12  matches: 18` after the file name says nothing the header didn't,
and the directory header `./` of a file in the working directory adds a line of its own. The saving is ~20–25
characters per response with one file; such responses are frequent (a search to confirm one place, a count in one
file).

## Scope

A spec change first (its own commit; OUTPUT.md), then the code.

- the spec step decides: which counts are left out with one file (the file line's or the header's), whether a
  directory header `./` is written for a file in the working directory, the `lines` detail, JSON (likely unchanged:
  its fields are read by code)
- `output.show = ["file_counts"]` still writes them
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- results with more than one file; `count_by = line`

## Examples

`158__EXAMPLES__ONE_FILE_COUNTS.md` covers:

- `count` with `count_by = file`, `files` and `lines` with one file, in the working directory and below it
- the same with two files: unchanged
- `show = ["file_counts"]`

## Acceptance criteria

- the spec states what a one-file response writes before the code is written
- every example in `158__EXAMPLES__ONE_FILE_COUNTS.md` passes as a test; the examples of all earlier tasks still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
