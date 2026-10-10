# Plan for task 16: `kind` target

## Design

- `query/eval.rs`: `Kind { Source, Test, Config, Doc, Generated, Other }` (keywords in the order of QUERY.md#kind);
  `Filter::Kind { id, order, values, negated }`; `FileValues.kind`.
- `search/kinds.rs` (from `generated.rs`): `KindRules` with the built-in `kinds.generated`, `test`, `doc`, `config`
  wildcards and the generated markers; `classify(rel, path, is_file, lang)` gives the first kind in the order
  `generated`, `test`, `doc`, `config`, `source`, `other`: the markers are read only for regular files, `source` is
  a language with `source = true`.
- `search::visit`: with a `kind` condition the kind is computed before the evaluation and nothing is skipped
  implicitly; without one a regular file of kind `generated` is skipped after it matched (as before).
- `query`: `Needs.kind` (needs the language too); `unknown_value` for kinds; the subset supports `kind`.
- `description.rs`: `kind in (source, test)`.

## Tests

- examples E16.1–E16.11 (fixture `kinds`)
- unit: `classify` for every kind and the order of the rules (a `*_test.md` file is `test`, a generated Rust file
  is `generated`); `Filter::Kind`; `unknown_value` for kinds

## Steps

1. failing unit tests
2. kind rules, filter, search, validation
3. description

## Risks

None known.
