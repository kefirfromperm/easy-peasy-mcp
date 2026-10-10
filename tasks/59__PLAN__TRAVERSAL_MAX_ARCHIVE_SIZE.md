# Plan for task 59: `traversal.max_archive_size`

## Design

- `src/config/keys.rs`: `Key::MaxArchiveSize`, `Type::Size`, default `"100M"`.
- `src/warning.rs`: `WarningCode::ArchiveTooBig` (`archive_too_big`), after `archive_error`.
- `src/search/mod.rs`: `SearchOptions.max_archive_size`; `visit_members` adds up the uncompressed sizes of the
  members in their order; the member which would exceed the limit stops the archive: it and the rest are not entries,
  the archive gets `archive_too_big`.

## Tests

- examples E59.1–E59.7 (fixture `archives` of task 58)
- unit: the request field, the warning order

## Steps

1. failing tests
2. key, warning, the count of the members

## Risks

- none
