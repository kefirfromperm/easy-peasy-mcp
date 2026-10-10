# Plan for task 54: `traversal.max_file_size`

## Design

- `src/config/keys.rs`: `Key::MaxFileSize`, `Type::Size` — a string which is a size of the `size` target
  (`query::parse_size`), kept as written; default `"10M"`.
- `src/warning.rs`: `WarningCode::TooBig` (`too_big`), second in the order of the table.
- `src/search/mod.rs`: `SearchOptions.max_file_size` in bytes; when the content of a file is needed and the file is
  bigger, it is not read: its content is unknown and it gets `too_big`.
- `src/engine.rs`: the setting → bytes.

## Tests

- examples E54.1–E54.8 (fixture `big-files`: a 9 bytes and a 1059 bytes file)
- unit: the size type, the request field, the order of the warnings

## Steps

1. failing tests
2. key, warning, walker

## Risks

- none
