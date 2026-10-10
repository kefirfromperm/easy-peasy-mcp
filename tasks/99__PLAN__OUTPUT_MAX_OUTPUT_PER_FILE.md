# Plan for task 99: `output.max_output_per_file`

## Design

- `src/config/keys.rs`: `Key::MaxOutputPerFile` after `MaxLineLength`, `Type::AtLeast(100)`, default `4000`.
- `src/output/listing.rs`: after `max_matches_per_file` the blocks of the file (with their context) are made and the
  characters of their lines as shown (after the dedent and the cut of long lines, without the line numbers and the
  markers) are added up; the line which would go over ends the file: `Listing.until` — the first line not listed —
  its fragments from there are dropped, the file is marked cut (OUTPUT.md, clarified in its own commit). `Listing::new`
  takes the block options.
- `src/output/blocks.rs`: `before` — the blocks without the lines from `until` on, used by both formats.
- `src/output/mod.rs`: `Limits.max_output_per_file`; `src/engine.rs`, `src/description.rs`.

## Tests

- examples E99.1–E99.10 (fixture `output-limits`, fixture `long-lines` of task 98)
- unit: the listing (matched lines, context lines), the request field

## Steps

1. failing tests
2. key, the cut of the output of a file

## Risks

- none
