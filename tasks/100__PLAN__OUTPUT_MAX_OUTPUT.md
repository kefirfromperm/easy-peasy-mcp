# Plan for task 100: `output.max_output`

## Design

- `src/config/keys.rs`: `Key::MaxOutput` after `MaxOutputPerFile`, `Type::AtLeast(100)`, default `40000`.
- `src/output/listing.rs`: a last pass over the listing, in `files` and `lines` detail: the characters of the full
  path of every listed file and of its listed lines (as `max_output_per_file` counts them) are added up; the file
  whose path would go over and the next ones are not listed, the line which would go over and the next lines are not
  listed and no file after its file (OUTPUT.md, clarified in its own commit). `Listing.shown` — the number of the
  listed files which are written.
- `src/output/mod.rs`, `src/output/json.rs`: only the first `shown` files are written; `Limits.max_output`.
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E100.1–E100.9 (fixture `long-names`, fixture `output-limits` of task 99)
- unit: the listing (a path which doesn't fit, a line which doesn't fit), the request field

## Steps

1. failing tests
2. key, the last pass

## Risks

- none
