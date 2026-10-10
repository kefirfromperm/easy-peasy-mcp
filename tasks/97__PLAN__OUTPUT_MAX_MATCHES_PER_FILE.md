# Plan for task 97: `output.max_matches_per_file`

## Design

- `src/config/keys.rs`: `Key::MaxMatchesPerFile` after `MaxMatches`, `Type::AtLeast(1)`, default `50`.
- `src/output/listing.rs`: in `lines` detail the fragments of a file are first cut to `max_matches_per_file`, then
  `max_lines` and `max_matches` count what is left (OUTPUT.md, clarified in its own commit); `Listing.cut` marks the
  files a per file limit cut.
- `src/output/mod.rs`, `src/output/json.rs`: `(cut)` after the counts, `"truncated": true` per file;
  `Limits.max_matches_per_file`; `src/engine.rs`, `src/description.rs`.

## Tests

- examples E97.1–E97.10 (fixture `sorting` of task 92)
- unit: the listing (the cut files, the order of the limits), the request field

## Steps

1. failing tests
2. key, the cut, `(cut)`

## Risks

- none
