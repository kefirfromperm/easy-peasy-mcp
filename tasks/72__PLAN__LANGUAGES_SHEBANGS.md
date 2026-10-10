# Plan for task 72: `languages.<name>.shebangs`

## Design

- `src/config/languages.rs`: `shebangs` is implemented (a list of strings, the type of task 71); the detection by the
  shebang of a file without an extension has existed since task 15 and uses the table of the request.
- `src/description.rs`: `shebangs` in the `languages` object.

## Tests

- examples E72.1–E72.7 (fixture `shebangs`: `env` with an option, a version, a program path, no shebang)
- unit: the request field

## Steps

1. failing tests
2. the key

## Risks

- none
