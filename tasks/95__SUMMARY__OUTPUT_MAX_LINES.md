# Summary of task 95: `output.max_lines`

Server version: `0.95.0`.

## What was built

- `output.max_lines` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `500`, at least 1: from the config
  file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.max_lines`; invalid values like every key with a minimum.
- In `lines` detail only the first matched lines under the files are listed, in the order of the files; a
  multiline fragment only with all its lines; a file whose lines are all cut is still listed with its counts. The
  repeated lines are not counted. `truncated: yes (max_lines)`, JSON `truncated_by: ["max_lines"]`; the counts stay
  the totals.

## Architecture

- `src/output/listing.rs` (new): `Listing` / `Listed` — per listed file its fragments after the repeated lines and
  the limits, or left out; the limits which cut. Both formats list from it (`Response::listing`, `push_file` split
  from `to_text`); the next limits join it.
- `src/output/mod.rs`: `Limits.max_lines`; `src/config/keys.rs`: `Key::MaxLines`; `src/engine.rs`,
  `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–94 show the key and OUTPUT.md says what `max_lines` and `max_matches` count
  (separate commits).

## Tests

- examples E95.1–E95.9 (fixture `detail` of task 79)
- unit: `output::listing::tests::the_first_matched_lines`, `request::tests::output`

## Limitations and known gaps

- The cursor of the next page comes with task 101.

## Try it

`{ "query": "text has 'TODO'", "output": { "max_lines": 20 } }`
