# Plan for task 85: `output.group_dirs`

## Design

- `src/config/keys.rs`: `Key::GroupDirs`, `Type::Bool`, default `true`.
- `src/output/mod.rs`: `Response.group_dirs`; without it the text format writes the full path of every file (as in
  JSON) and no directory headers. JSON is not changed.

## Tests

- examples E85.1–E85.7 (fixture `detail` of task 79)
- unit: the text without groups, the request field

## Steps

1. failing tests
2. key, the text

## Risks

- none
