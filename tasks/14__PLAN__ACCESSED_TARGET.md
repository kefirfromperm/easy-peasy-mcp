# Plan for task 14: `accessed` target

## Design

- `accessed` works like `created` (task 13): the date values and operators of `modified`, `NumberTarget::Accessed`,
  `FileValues.accessed: Option<f64>`, unknown and `date_unsupported` without an access date; `Needs.accessed`.
- The access time is read from the entry's metadata in `search::visit` before anything reads the file's content
  (the generated marker check), so the search doesn't change what it compares; directories are read by the walk
  after their own entry was visited.
- `query/subset.rs`: `accessed` is supported.
- the example runner sets access times with `atime <path> <date> <time>` (done with the examples).
- `description.rs`: `accessed`.

## Tests

- examples E14.1–E14.6 (fixture `atimes`, access times from the setup, `Now: 2026-10-08 12:00`)
- unit: an `accessed` condition, `Needs.accessed`, a missing access date is unknown

## Steps

1. failing unit tests
2. target, filter, search, subset, description

## Risks

- `relatime` / `noatime` mounts: the setup sets the access times explicitly; the order in `visit` keeps the search
  from changing them before the comparison
