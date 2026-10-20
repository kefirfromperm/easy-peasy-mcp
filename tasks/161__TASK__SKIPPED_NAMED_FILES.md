# Task 161: The skipped files a non-empty result names

Status: not implemented

Depends on: task 60, task 103, task 129

## Goal

The traversal skips generated, ignored, binary and too big files before any condition is checked. An empty result
says so in its funnel (`skipped: 1 generated (traversal.skip_generated)`, task 103), but a non-empty one says
nothing, also when the query names a skipped file. An agent that selects files by their names gets some of them and
can't tell that the others were left out, and a write tool silently leaves them unchanged. Found in run 6 of the
experiment (EXPERIMENT.md, task 154, the easy-peasy arm): the version bump `replace` on `Cargo.toml` and
`Cargo.lock` changed only `Cargo.toml`; `cargo build` updated `Cargo.lock` later. Reproduced on 0.152.4:

```
find  query = "path ~ 'Cargo.{toml,lock}' and line(text starts 'version = \"0.152')"
      output = {"detail": "files", "highlight": "none", "show": ["warnings"]}

files: 1  lines: 1  result: r19

./
Cargo.toml  lines: 1
```

`git grep -l '^version = "0.152' -- Cargo.toml Cargo.lock` lists both: `Cargo.lock` is `kind = generated` and
`traversal.skip_generated` is `true` by default. One line such as `skipped: Cargo.lock generated
(traversal.skip_generated)` (~15 tokens) would have saved the wrong edit and the time to find it.

## Scope

A spec change first (its own commit; OUTPUT.md#funnel, QUERY.md#generated-files, TOOLS.md for the write tools), then
the code.

- the spec step decides which skipped files a non-empty result names: those matched by the `path` / `name`
  conditions of the query (a file the agent asked for by name), or every skipped file under a `path` condition
  without a wildcard
- where it is written: a `skipped:` line in the header, a warning (left out by default in spec 2, so not enough
  alone), or a hint; the reasons as in the funnel of task 103 (generated, ignored, binary, too big)
- the write tools (`replace`, `rename`, `copy`, `delete`) write it in the dry run and in the real response
- a cap on the names listed (`… N more`), and an `omit` part for it (every new piece of output can be left out)
- spec 1 is unchanged; a change of the spec 2 output follows [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- changing the defaults of `skip_generated`, `binary` or the ignore files
- the funnel of an empty result (task 103)

## Examples

`161__EXAMPLES__SKIPPED_NAMED_FILES.md` covers:

- `path ~ 'Cargo.{toml,lock}'` with `Cargo.lock` generated: the result names it as skipped
- an ignored and a binary file named by `name = ...`
- a broad query (`text has 'x'`) over a tree with generated files: nothing more is written
- `replace` with a dry run on a selection with a skipped file
- the part left out with `omit`; `"version": 1`

## Acceptance criteria

- the spec says which skipped files a non-empty result names and where, before the code is written
- every example in `161__EXAMPLES__SKIPPED_NAMED_FILES.md` passes as a test; the examples of all earlier tasks still
  pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
