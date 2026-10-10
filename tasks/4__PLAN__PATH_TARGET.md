# Plan for task 4: `path` target

## Design

- `query/subset.rs`: `path` joins `name` as an allowed target (operators `=`, `has`, `starts`, `ends`, `~`).
- `query/mod.rs`: `Filter::Name` becomes `Filter::Text { id, target, matcher }` with `target` `Name` or `Path`;
  the matcher is the same for both (task 3).
- `search`: a `FileInfo { name, path }` is built per entry: the NFC file name and the NFC path relative to the
  working directory with `/` separators (`slash_path`); the filter picks the value of its target.
- wildcards already treat `/` as the separator: `*`, `?`, `[!...]` don't cross it, `**/` matches zero or more
  directories.
- `description.rs`: `path ~ 'src/**/*.ts'` and the path operators.

## Tests

- examples E4.1–E4.8
- unit: subset accepts `path`; `Filter::Text` on paths; `slash_path` of nested relative paths

## Steps

1. failing unit tests
2. filter and search changes
3. description

## Risks

None known; `\` as a separator on Windows is task 120.
