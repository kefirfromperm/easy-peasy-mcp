# Summary of task 99: `output.max_output_per_file`

Server version: `0.99.0`.

## What was built

- `output.max_output_per_file` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `4000`, at least 100:
  from the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for
  one request by `output.max_output_per_file`; invalid values like every key with a minimum.
- In `lines` detail the characters of the matched and context lines of a file as they are shown (after the dedent
  and the cut of long lines, without line numbers and markers) are added up; the line which would go over and the
  next lines of the file are not listed. The file gets `(cut)` / `"truncated": true`, `truncated_by` names
  `max_output_per_file`, the next file is listed as usual.

## Architecture

- `src/output/listing.rs`: `first_over` (the blocks of the file and their shown lines), `Listing.until`, the per file
  cuts before `max_lines` / `max_matches`; `Listing::new` takes the block options.
- `src/output/blocks.rs`: `before`; both formats render a file from the listing (`push_file` and the JSON `file` take
  the listing and the index).
- `src/output/mod.rs`: `Limits.max_output_per_file`; `src/config/keys.rs`: `Key::MaxOutputPerFile`; `src/engine.rs`,
  `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–98 show the key and OUTPUT.md says what the limit counts (separate commits).

## Tests

- examples E99.1–E99.10 (fixture `output-limits`, fixture `long-lines` of task 98)
- unit: `output::listing::tests::the_first_characters_of_every_file`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "context": 3, "max_output_per_file": 500 } }`
