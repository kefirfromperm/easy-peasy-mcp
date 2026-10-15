# Plan for task 148: Paths relative to the only working directory

## Design

The spec step ([OUTPUT.md#one-working-directory](../OUTPUT.md#one-working-directory)) makes `relative` with one
working directory in spec 2 write the paths as `in_dir`, so the code reuses `in_dir` as it is:

- `engine.rs`: `in_dirs` decides from the path style, the spec version, `show` and the number of working directories:
  `in_dir` always; `relative` in spec 2 with exactly one working directory, unless `show` has `dir`. Everything
  downstream (the `in:` line, the stripped paths of the files, warnings, facets, hints, repeated locations, JSON
  `dirs`, `since`) already follows `in_dirs`
- `respond` computes `spec` and `show` before `in_dirs`
- the working directory `.` goes the same way as with `in_dir`: no line, the paths unchanged
- `within`: its working directories are the ones of the referred result, so the rule follows their number
- the write tools keep the files of a result with their full relative paths (`keep_result` runs before the paths are
  stripped), so `result = "rN"` is unchanged; their responses don't use `in_dirs`

Rejected: making `in_dir` the default of spec 2 — it would also change several working directories, which the task
keeps as they are.

## Tests

- unit: `engine::tests::one_working_directory_is_written_once` — `in_dirs` for `relative` in spec 1 and 2, one and
  two working directories, `show = ["dir"]`, `in_dir`, `absolute`
- examples: E148.1–E148.6; earlier spec 2 examples with one working directory other than `.` would change (none are
  known: the spec 2 examples search the fixture root)

## Steps

1. red: the unit test; the examples already fail on E148.*
2. green: `in_dirs` with the spec rule
3. summary, status `implemented`, version 0.145.0

## Risks

- unit or integration tests outside the examples which search one absolute directory in spec 2 and compare its paths
  change; they are fixed with the reason
