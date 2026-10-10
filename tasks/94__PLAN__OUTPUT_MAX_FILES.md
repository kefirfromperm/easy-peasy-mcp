# Plan for task 94: `output.max_files`

## Design

- `src/config/keys.rs`: `Key::MaxFiles` after `Markers`, `Type::AtLeast(1)`, default `100`.
- `src/output/mod.rs`: `Limits { max_files }` in the response (the limits of tasks 95–100 join it); the files of the
  response are all matched files, in `files` and `lines` detail only the first `max_files` are listed (the repeated
  lines are grouped among them); the counts stay the totals of all files.
- `truncated_by` of the response in the order of the limits table (OUTPUT.md, clarified in its own commit): the text
  header `truncated: yes (max_files, timeout)`, JSON `truncated` / `truncated_by`.
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E94.1–E94.11 (fixture `sorting` of task 92)
- unit: the listed files and the header, the request field

## Steps

1. failing tests
2. key, the limit, `truncated_by`

## Risks

- the cursor of the next page comes with task 101
