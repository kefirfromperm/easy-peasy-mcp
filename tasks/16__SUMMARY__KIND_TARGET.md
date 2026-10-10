# Summary of task 16: `kind` target

Server version: `0.16.0`.

## What was built

- `kind` with `=`, `!=`, `in ( ... )` and the values `source`, `test`, `config`, `doc`, `generated`, `other`, in
  any case (QUERY.md#kind).
- Every entry gets the first kind in the order `generated`, `test`, `doc`, `config`, `source`, `other`, by the
  built-in rules of CONFIG.md#kinds: path wildcards (relative to the working directory, with `path_case`), the
  generated markers in the first 5 lines of regular files, `source` for a language with `source = true`.
- A query with a `kind` condition skips nothing implicitly (QUERY.md#generated-files): `kind = generated` returns
  generated files.
- `unknown_value` for kinds.
- The `find` description shows `kind`.

## Architecture

- `search/kinds.rs` replaces `generated.rs`: `KindRules` (built-in wildcards and markers) with `classify` and
  `is_generated`.
- `Kind`, `Filter::Kind`, `FileValues.kind`, `Needs.kind` (which also needs the language).

## Deviations from the plan

None.

## Tests

- examples E16.1–E16.11 (fixture `kinds`: one file of every kind, a test file which also matches a doc rule, a Rust
  file with a generated marker)
- unit: `classify` for every kind and the rule order, markers only for regular files, `Filter::Kind`,
  `unknown_value` for kinds

## Limitations and known gaps

- the rules can't be changed yet (tasks 63–68)

## Try it

`{ "query": "kind = test and name ~ '*.go'" }`
