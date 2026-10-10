# Plan for task 95: `output.max_lines`

## Design

- `src/config/keys.rs`: `Key::MaxLines` after `MaxFiles`, `Type::AtLeast(1)`, default `500`.
- `src/output/listing.rs` (new): `Listing` — what is listed of every listed file in `lines` detail: its fragments
  after the repeated lines and the limits, or left out (all its lines repeated); the limits which cut. `max_lines`
  keeps the fragments of the first matched lines in the order of the files (a multiline fragment only with all its
  lines); a file whose lines are all cut is listed without lines (OUTPUT.md, clarified in its own commit). The
  limits of tasks 96–100 join it.
- `src/output/mod.rs`, `src/output/json.rs`: both formats list from the `Listing`; `Limits.max_lines`.
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E95.1–E95.9 (fixture `detail` of task 79)
- unit: the listing (the cut, a multiline fragment, a cut file), the request field

## Steps

1. failing tests
2. key, the listing

## Risks

- the cursor of the next page comes with task 101
