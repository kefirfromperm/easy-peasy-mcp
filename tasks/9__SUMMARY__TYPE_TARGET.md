# Summary of task 9: `type` target

Server version: `0.9.0`.

## What was built

- `type` with `=`, `!=`, `in ( ... )` and the values `file`, `dir`, `link` in any case
  (QUERY.md#files-and-directories).
- A query with a `type` condition anywhere (also under `not` or in an `or` branch) has no implicit `type = file`;
  otherwise only regular files are returned.
- Symlinks are `link` and are not followed; the working directory itself is never an entry (spec clarified).
- Directories are written with a trailing separator, `sub/` under the header `lib/`; a symlink as the link
  (OUTPUT.md#paths, clarified).
- `unknown_value` for a wrong value: `unknown value 'folder'`, hint `did you mean 'dir'? expected one of: file, dir,
  link`.
- The `find` description shows `type`.
- The example runner runs a fixture's `text setup` block (`mkdir`, `file`, `symlink`) and sets the times of
  directories and symlinks; AGENTS.md describes the block.

## Architecture

- `query/eval.rs`: `EntryType` and `Filter::Type { values, negated }`; `FileValues.entry_type`.
- `query/mod.rs`: `Query.has_type`; `unknown_value` validation with suggestions (`folder` → `dir`).
- `search`: every entry below the working directory with its type from `file_type()` (not following symlinks);
  only regular files are checked for `generated` and deduplicated by hard links; `Found.is_dir`.

## Deviations from the plan

- An example fix commit: the generated responses wrote directories wrongly.

## Tests

- examples E9.1–E9.8 (fixture `types`, symlinks from the setup)
- unit: type filters for `=`, `!=`, `in`; `has_type` under `not` and `or`; `unknown_value` with and without a
  suggestion; directory lines in the output

## Limitations and known gaps

- junctions on Windows (task 120); `follow_symlinks` (task 46)

## Try it

`{ "query": "type = dir and name = 'migrations'" }`
