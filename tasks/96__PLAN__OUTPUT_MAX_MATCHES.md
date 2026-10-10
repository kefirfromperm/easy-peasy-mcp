# Plan for task 96: `output.max_matches`

## Design

- `src/config/keys.rs`: `Key::MaxMatches` after `MaxLines`, `Type::AtLeast(1)`, default `1000`.
- `src/output/listing.rs`: after `max_lines` the fragments of a file are cut to the matches left, in the order of the
  files and of the fragments; a line without listed fragments is not listed, a line with some is listed with those
  marked (OUTPUT.md#limits-and-truncation). `max_lines` and `max_matches` count what is left after both.
- `src/output/mod.rs`: `Limits.max_matches`; `src/engine.rs`, `src/description.rs`.

## Tests

- examples E96.1–E96.10 (fixture `sorting` of task 92)
- unit: the listing with both limits, the request field

## Steps

1. failing tests
2. key, the cut

## Risks

- the cursor of the next page comes with task 101
