# Plan for task 86: `output.group_repeated`

## Design

- `src/config/keys.rs`: `Key::GroupRepeated`, `Type::Whole(Some("0: off"))`, default `3`.
- `src/output/repeated.rs` (new): the repeated lines of the listed files — every matched line which no multiline
  fragment touches, keyed by its text without the leading and trailing whitespace and its fragments relative to that
  text (columns, ids, escape levels); a key in at least `group_repeated` files is a repeated line with its files in
  their order (the first line of every file), sorted by the number of files and then by the first file. The lines of
  repeated keys are left out of the files.
- `src/output/mod.rs`: the text section `repeated:` after the legend and its empty line (OUTPUT.md#repeated-lines,
  clarified in its own commit); a file without other matched lines is left out in `lines` detail; the blocks are made
  from the fragments of the other lines.
- `src/output/lines.rs`: `marked_line` — a line with its markers, used by the blocks and by the repeated lines.
- `src/output/json.rs`: `repeated` with `count`, `text`, `fragments`, `locations`.

## Tests

- examples E86.1–E86.10 (fixture `repeated`: the same line in three files, once indented)
- unit: the repeated lines (whitespace, ids, a multiline fragment, the minimum, the order), the request field

## Steps

1. failing tests
2. key, repeated lines, the text and JSON

## Risks

- none
