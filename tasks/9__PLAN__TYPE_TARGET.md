# Plan for task 9: `type` target

## Design

- `query/eval.rs`: `EntryType { File, Dir, Link }` and `Filter::Type { id, order, values, negated }` (`=` and `in`
  — the type is one of the values; `!=` — negated); `FileValues` gets `entry_type`.
- `query/mod.rs`: `Query.has_type` — whether any `type` condition is in the query (also under `not` or `or`); without
  it the search adds the implicit `type = file`. Validation: `unknown_value` for a value of `type` which isn't
  `file`, `dir`, `link` (any case): message `unknown value 'x'`, the value marked, hint `did you mean 'dir'? expected
  one of: file, dir, link` (synonyms like `folder`, `directory`, `symlink`).
- `query/subset.rs`: `type` with `=`, `!=`, `in` is supported.
- `search`: the walk reports every entry below the working directory (not the working directory itself); the entry
  type from `file_type()` without following symlinks: symlink → `Link`, directory → `Dir`, regular file → `File`.
  Only regular files can be generated (the marker check reads content; directories and symlinks have none to read).
  Hard link deduplication only for regular files. `Found` gets `is_dir`.
- `output`: a directory is written with a trailing separator (spec clarified); a symlink as the link.
- `description.rs`: `type = dir|file|link` and the implicit `type = file`.
- test harness: the `text setup` block of a fixture and the times of directories and symlinks (done with the
  examples).

Junctions on Windows come with task 120; `follow_symlinks` with task 46.

## Tests

- examples E9.1–E9.8 (fixture `types` with symlinks from the setup)
- unit: `Filter::Type` for `=`, `!=`, `in`; `has_type` under `not` / `or`; `unknown_value` with suggestions; the
  subset accepts `type`; output of a directory line

## Steps

1. failing unit tests
2. query: type filter, validation, subset
3. search: entry types, implicit `type = file`; output of directories
4. description

## Risks

- symlinks on Windows need a privilege; the CI task (118) decides how examples with symlinks run there
